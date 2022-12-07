// This file is part of Darwinia.
//
// Copyright (C) 2018-2022 Darwinia Network
// SPDX-License-Identifier: GPL-3.0
//
// Darwinia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Darwinia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

//! # Darwinia account migration pallet
//!
//! ## Overview
//!
//! Darwinia2 uses ECDSA as its signature algorithm instead of SR25519.
//! These two algorithm are not compatible.
//! Thus, an account migration is required.
//!
//! ## Technical detail
//!
//! Users must send an extrinsic themselves to migrate their account(s).
//! This extrinsic should be unsigned, the reason is the same as `pallet-claims`.
//! This extrinsic's payload must contain a signature to the new ECDSA address, signed by their
//! origin SR25519 key.
//!
//! This pallet will store all the account data from Darwinia1 and Darwinia Parachain.
//! This pallet's genesis will be write into the chain spec JSON directly.
//! The data will be processed off-chain(ly).
//! After the verification, simply perform a take & put operation.
//!
//! ```
//! user -> send extrinsic -> verify -> put(storages, ECDSA, take(storages, SR25519))
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
// #![deny(missing_docs)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod test;

// darwinia
use dc_primitives::{Balance, Index};
// substrate
use frame_support::{log, pallet_prelude::*};
use frame_system::{pallet_prelude::*, AccountInfo};
use pallet_balances::AccountData;
use sp_core::sr25519::{Public, Signature};
use sp_io::hashing;
use sp_runtime::traits::Verify;

type AccountId20 = [u8; 20];
type AccountId32 = [u8; 32];
type Message = [u8; 32];

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config<AccountId = AccountId20> {
		/// Override the [`frame_system::Config::RuntimeEvent`].
		type RuntimeEvent: From<Event> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The migration destination was already taken by someone.
		AccountAlreadyExisted,
		/// The migration source was not exist.
		AccountNotFound,
		/// Invalid signature.
		InvalidSignature,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event {
		/// An account has been migrated.
		Migrated { from: AccountId32, to: AccountId20 },
	}

	#[pallet::storage]
	#[pallet::getter(fn account_of)]
	pub type Accounts<T: Config> =
		StorageMap<_, Identity, AccountId32, AccountInfo<Index, AccountData<Balance>>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// TODO: update weight
		/// Migrate all the account data under the `from` to `to`.
		#[pallet::weight(0)]
		pub fn migrate(
			origin: OriginFor<T>,
			from: AccountId32,
			to: AccountId20,
			signature: Signature,
		) -> DispatchResult {
			ensure_none(origin)?;

			// Make sure the `to` is not existed on chain.
			if <frame_system::Account<T>>::contains_key(to) {
				Err(<Error<T>>::AccountAlreadyExisted)?;
			}

			let account = <Accounts<T>>::take(from).ok_or(<Error<T>>::AccountNotFound)?;
			let message = Self::signable_message(&to);

			Self::deposit_event(Event::Migrated { from, to });

			Ok(())
		}
	}
	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			// The migration destination was already taken by someone.
			const E_ACCOUNT_ALREADY_EXISTED: u8 = 0;
			// The migration source was not exist.
			const E_ACCOUNT_NOT_FOUND: u8 = 1;
			// Invalid signature.
			const E_INVALID_SIGNATURE: u8 = 1;

			let Call::migrate { from, to, signature } = call else {
				return InvalidTransaction::Call.into();
			};

			// Make sure the `to` is not existed on chain.
			if <frame_system::Account<T>>::contains_key(to) {
				return InvalidTransaction::Custom(E_ACCOUNT_ALREADY_EXISTED).into();
			}

			let Some(account) = <Accounts<T>>::take(from) else {
				return InvalidTransaction::Custom(E_ACCOUNT_NOT_FOUND).into();
			};
			let message = Self::signable_message(to);

			if verify_sr25519_signature(from, &message, signature) {
				ValidTransaction::with_tag_prefix("account-migration")
					.and_provides(from)
					.priority(100)
					.longevity(TransactionLongevity::max_value())
					.propagate(true)
					.build()
			} else {
				InvalidTransaction::Custom(E_INVALID_SIGNATURE).into()
			}
		}
	}
	impl<T> Pallet<T>
	where
		T: Config,
	{
		fn signable_message(account_id_20: &AccountId20) -> Message {
			hashing::blake2_256(
				&[T::Version::get().spec_name.as_ref(), b"::account-migration", account_id_20]
					.concat(),
			)
		}
	}
}
pub use pallet::*;

fn verify_sr25519_signature(
	public_key: &AccountId32,
	message: &Message,
	signature: &Signature,
) -> bool {
	// Actually, `&[u8]` is `[u8; 32]` here.
	// But for better safety.
	let Ok(public_key) = &Public::try_from(public_key.as_slice()) else {
		log::error!("[pallet::account-migration] `public_key` must be valid; qed");

		return false;
	};

	signature.verify(message.as_slice(), public_key)
}
