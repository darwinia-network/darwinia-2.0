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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod test;

type AccountIdOf<R> = <R as frame_system::pallet::Config>::AccountId;
pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// substrate
use frame_support::traits::Currency;
use sp_core::{blake2_256, sr25519, ByteArray, Pair, H160};
use sp_runtime::{traits::Verify, AccountId32};

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_core::sr25519::Signature;

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_evm::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<Self::AccountId>;
	}

	// Storage the migrated balance map from darwinia-1.0 chain
	// TODO: check if u128 if enough?
	#[pallet::storage]
	pub(super) type Balances<T> = StorageMap<_, Blake2_128Concat, AccountId32, u128>;

	#[pallet::error]
	pub enum Error<T> {
		InvalidSignature,
		AccountNotExist,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event {
		Claim { old_account_id_pub_key: AccountId32, new_account_id: H160, amount: u128 },
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		AccountIdOf<T>: From<H160>,
		BalanceOf<T>: From<u128>,
	{
		// Unsigned transaction
		#[pallet::weight(0)]
		pub fn claim_to(
			origin: OriginFor<T>,
			old_account_id_pub_key: AccountId32,
			new_account_id: H160,
			_sig: Signature,
		) -> DispatchResult {
			ensure_none(origin)?;

			if let Some(amount) = Balances::<T>::take(&old_account_id_pub_key) {
				<T as pallet::Config>::Currency::deposit_into_existing(
					&new_account_id.into(),
					amount.into(),
				)?;

				Self::deposit_event(Event::Claim {
					old_account_id_pub_key,
					new_account_id,
					amount,
				});
			}
			Err(Error::<T>::AccountNotExist.into())
		}
	}
	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T>
	where
		AccountIdOf<T>: From<H160>,
		BalanceOf<T>: From<u128>,
	{
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			if let Call::claim_to { old_account_id_pub_key, new_account_id, sig } = call {
				let message = ClaimMessage::new(
					<T as pallet_evm::Config>::ChainId::get(),
					old_account_id_pub_key,
					new_account_id,
				);
				if let Ok(signer) = sr25519::Public::from_slice(old_account_id_pub_key.as_ref()) {
					let is_valid = sig.verify(&message.raw_bytes()[..], &signer);

					if is_valid {
						return ValidTransaction::with_tag_prefix("MigrateClaim")
							.priority(TransactionPriority::max_value())
							// TODO: ADD more config?
							.propagate(true)
							.build();
					}
				}
				return InvalidTransaction::BadSigner.into();
			} else {
				InvalidTransaction::Call.into()
			}
		}
	}
}

pub struct ClaimMessage<'m> {
	pub chain_id: u64,
	pub old_account_id_pub_key: &'m AccountId32,
	pub new_account_id_pub_key: &'m H160,
}

impl<'m> ClaimMessage<'m> {
	fn new(
		chain_id: u64,
		old_account_id_pub_key: &'m AccountId32,
		new_account_id_pub_key: &'m H160,
	) -> Self {
		Self { chain_id, old_account_id_pub_key, new_account_id_pub_key }
	}

	fn raw_bytes(&self) -> [u8; 32] {
		let mut result = Vec::new();
		result.extend_from_slice(&self.chain_id.to_be_bytes());
		result.extend_from_slice(self.old_account_id_pub_key.as_slice());
		result.extend_from_slice(&self.new_account_id_pub_key.0);

		blake2_256(&result[..])
	}
}
