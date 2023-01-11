// This file is part of Darwinia.
//
// Copyright (C) 2018-2023 Darwinia Network
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
//! ```nocompile
//! user -> send extrinsic -> verify -> put(storages, ECDSA, take(storages, SR25519))
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

#[cfg(test)]
mod tests;

// darwinia
use darwinia_deposit::Deposit;
use darwinia_staking::Ledger;
use dc_primitives::{AccountId as AccountId20, AssetId, Balance, BlockNumber, Index};
// substrate
use frame_support::{
	log, migration,
	migration::put_storage_value,
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement::KeepAlive, LockableCurrency, WithdrawReasons},
	StorageHasher,
};
use frame_system::{pallet_prelude::*, AccountInfo, RawOrigin};
use pallet_balances::AccountData;
use pallet_identity::{RegistrarInfo, Registration};
use pallet_vesting::VestingInfo;
use sp_core::sr25519::{Public, Signature};
use sp_io::hashing;
use sp_runtime::{
	traits::{IdentityLookup, Verify},
	AccountId32,
};
use sp_std::prelude::*;

type Message = [u8; 32];
const KTON_ID: u64 = 1026;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::config]
	pub trait Config:
		frame_system::Config<
			Index = Index,
			BlockNumber = BlockNumber,
			AccountId = AccountId20,
			AccountData = AccountData<Balance>,
			Lookup = IdentityLookup<AccountId20>,
		> + pallet_assets::Config<Balance = Balance, AssetId = AssetId>
		+ pallet_balances::Config<Balance = Balance>
		+ pallet_vesting::Config<Currency = pallet_balances::Pallet<Self>>
		+ pallet_identity::Config<Currency = pallet_balances::Pallet<Self>>
		+ darwinia_deposit::Config
		+ darwinia_staking::Config
	{
		/// Override the [`frame_system::Config::RuntimeEvent`].
		type RuntimeEvent: From<Event> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Chain's ID, which is used for constructing the message. (follow EIP-712 SPEC)
		#[pallet::constant]
		type ChainId: Get<u64>;
	}

	#[allow(missing_docs)]
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event {
		/// An account has been migrated.
		Migrated { from: AccountId32, to: AccountId20 },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Exceed maximum vesting count.
		ExceedMaxVestings,
		/// Exceed maximum deposit count.
		ExceedMaxDeposits,
	}

	/// [`frame_system::Account`] data.
	///
	/// <https://github.dev/paritytech/substrate/blob/19162e43be45817b44c7d48e50d03f074f60fbf4/frame/system/src/lib.rs#L545>
	#[pallet::storage]
	#[pallet::getter(fn account_of)]
	pub type Accounts<T: Config> =
		StorageMap<_, Blake2_128Concat, AccountId32, AccountInfo<Index, AccountData<Balance>>>;

	/// [`pallet_asset::AssetAccount`] data.
	///
	/// https://github.dev/paritytech/substrate/blob/polkadot-v0.9.30/frame/assets/src/types.rs#L115
	// The size of encoded `pallet_asset::AssetAccount` is 18 bytes.
	#[pallet::storage]
	#[pallet::getter(fn kton_account_of)]
	pub type KtonAccounts<T: Config> = StorageMap<_, Blake2_128Concat, AccountId32, [u8; 18]>;

	/// [`pallet_vesting::Vesting`] data.
	///
	/// <https://github.dev/paritytech/substrate/blob/19162e43be45817b44c7d48e50d03f074f60fbf4/frame/vesting/src/lib.rs#L188>
	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn vesting_of)]
	pub type Vestings<T: Config> =
		StorageMap<_, Blake2_128Concat, AccountId32, Vec<VestingInfo<Balance, BlockNumber>>>;

	/// [`darwinia_deposit::Deposits`] data.
	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn deposit_of)]
	pub type Deposits<T: Config> = StorageMap<_, Blake2_128Concat, AccountId32, Vec<Deposit>>;

	/// [`darwinia_staking::Ledgers`] data.
	#[pallet::storage]
	#[pallet::getter(fn ledger_of)]
	pub type Ledgers<T: Config> = StorageMap<_, Blake2_128Concat, AccountId32, Ledger<T>>;

	/// [`pallet_identity::IdentityOf`] data.
	///
	/// <https://github.com/paritytech/substrate/blob/polkadot-v0.9.30/frame/identity/src/lib.rs#L163>
	#[pallet::storage]
	#[pallet::getter(fn identity_of)]
	pub type IdentityOf<T: Config> = StorageMap<
		_,
		Twox64Concat,
		AccountId32,
		Registration<Balance, ConstU32<100>, ConstU32<100>>,
	>;

	/// [`pallet_identity::Registrars`] data.
	///
	/// <https://github.com/paritytech/substrate/blob/polkadot-v0.9.30/frame/identity/src/lib.rs#L199>
	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn registrars)]
	pub type Registrars<T: Config> =
		StorageValue<_, Vec<Option<RegistrarInfo<Balance, AccountId32>>>, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Migrate all the account data under the `from` to `to`.
		#[pallet::weight(0)]
		pub fn migrate(
			origin: OriginFor<T>,
			from: AccountId32,
			to: AccountId20,
			_signature: Signature,
		) -> DispatchResult {
			ensure_none(origin)?;

			let account = <Accounts<T>>::take(&from)
				.ok_or("[pallet::account-migration] already checked in `pre_dispatch`; qed")?;

			<frame_system::Account<T>>::insert(to, account);

			if let Some(a) = <KtonAccounts<T>>::take(&from) {
				migration::put_storage_value(
					b"Assets",
					b"Account",
					&[
						Blake2_128Concat::hash(&KTON_ID.encode()),
						Blake2_128Concat::hash(&to.encode()),
					]
					.concat(),
					a,
				);
			}
			if let Some(v) = <Vestings<T>>::take(&from) {
				let locked = v.iter().map(|v| v.locked()).sum();

				<pallet_vesting::Vesting<T>>::insert(
					to,
					BoundedVec::try_from(v).map_err(|_| <Error<T>>::ExceedMaxVestings)?,
				);

				// https://github.dev/paritytech/substrate/blob/19162e43be45817b44c7d48e50d03f074f60fbf4/frame/vesting/src/lib.rs#L248
				let reasons = WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE;

				// https://github.dev/paritytech/substrate/blob/19162e43be45817b44c7d48e50d03f074f60fbf4/frame/vesting/src/lib.rs#L86
				<pallet_balances::Pallet<T>>::set_lock(*b"vesting ", &to, locked, reasons);
			}
			if let Some(l) = <Ledgers<T>>::take(&from) {
				if let Some(ds) = <Deposits<T>>::take(&from) {
					<pallet_balances::Pallet<T> as Currency<_>>::transfer(
						&to,
						&darwinia_deposit::account_id(),
						ds.iter().map(|d| d.value).sum(),
						KeepAlive,
					)?;
					<darwinia_deposit::Deposits<T>>::insert(
						to,
						BoundedVec::try_from(ds).map_err(|_| <Error<T>>::ExceedMaxDeposits)?,
					);
				}

				let staking_pot = darwinia_staking::account_id();
				<pallet_balances::Pallet<T> as Currency<_>>::transfer(
					&to,
					&staking_pot,
					l.staked_ring + l.unstaking_ring.iter().map(|(r, _)| r).sum::<Balance>(),
					KeepAlive,
				)?;

				let sum = l.staked_kton + l.unstaking_kton.iter().map(|(k, _)| k).sum::<Balance>();
				if let Some(amount) = <pallet_assets::Pallet<T>>::maybe_balance(KTON_ID, to) {
					if amount >= sum {
						<pallet_assets::Pallet<T>>::transfer(
							RawOrigin::Signed(to).into(),
							KTON_ID,
							staking_pot,
							sum,
						)?;
					}
				}

				<darwinia_staking::Ledgers<T>>::insert(to, l);
			}

			if let Some(identity) = IdentityOf::<T>::take(from.clone()) {
				put_storage_value(
					b"Identity",
					b"IdentityOf",
					&Twox64Concat::hash(&to.encode()),
					identity.clone(),
				);
			}
			let mut chain_rs = <pallet_identity::Pallet<T>>::registrars().into_inner();
			for (i, rs) in Self::registrars().iter().enumerate() {
				if let Some(rs) = rs {
					if rs.account == from {
						chain_rs.push(Some(RegistrarInfo {
							account: to,
							fee: rs.fee,
							fields: rs.fields,
						}));

						Registrars::<T>::mutate(|rs| rs.remove(i));
					}
				}
			}
			put_storage_value(b"Identity", b"Registrars", &[], chain_rs);

			Self::deposit_event(Event::Migrated { from, to });

			Ok(())
		}

		// TODO: migrate multi-sig
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
			const E_INVALID_SIGNATURE: u8 = 2;

			let Call::migrate { from, to, signature } = call else {
				return InvalidTransaction::Call.into();
			};

			if !<Accounts<T>>::contains_key(from) {
				return InvalidTransaction::Custom(E_ACCOUNT_NOT_FOUND).into();
			}
			if <frame_system::Account<T>>::contains_key(to) {
				return InvalidTransaction::Custom(E_ACCOUNT_ALREADY_EXISTED).into();
			}

			let message = sr25519_signable_message(
				T::ChainId::get(),
				T::Version::get().spec_name.as_ref(),
				to,
			);

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
}
pub use pallet::*;

fn sr25519_signable_message(
	chain_id: u64,
	spec_name: &[u8],
	account_id_20: &AccountId20,
) -> Message {
	hashing::blake2_256(
		&[
			&hashing::blake2_256(
				&[&chain_id.to_le_bytes(), spec_name, b"::account-migration"].concat(),
			),
			account_id_20.0.as_slice(),
		]
		.concat(),
	)
}

fn verify_sr25519_signature(
	public_key: &AccountId32,
	message: &Message,
	signature: &Signature,
) -> bool {
	// Actually, `&[u8]` is `[u8; 32]` here.
	// But for better safety.
	let Ok(public_key) = &Public::try_from(public_key.as_ref()) else {
		log::error!("[pallet::account-migration] `public_key` must be valid; qed");

		return false;
	};

	signature.verify(message.as_slice(), public_key)
}
