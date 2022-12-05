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

// substrate
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
	pub trait Config: frame_system::Config + pallet_evm::Config {}

	// Storage the migrated balance map from darwinia-1.0 chain
	#[pallet::storage]
	pub(super) type Balances<T> = StorageMap<_, Blake2_128Concat, AccountId32, u128>;

	#[pallet::error]
	pub enum Error<T> {
		InvalidSignature,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Unsigned transaction
		#[pallet::weight(0)]
		pub fn claim_to(
			origin: OriginFor<T>,
			old_account_id_pub_key: AccountId32,
			new_account_id: H160,
			sig: Signature,
		) -> DispatchResult {
			ensure_none(origin)?;

			// deposit to new_account_id

			// Update the balances storage

			// Add event
			todo!();
		}
	}
	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			if let Call::claim_to { old_account_id_pub_key, new_account_id, sig } = call {
				let message = MessageType::new(
					<T as pallet_evm::Config>::ChainId::get(),
					old_account_id_pub_key,
					new_account_id,
				);
				if let Ok(signer) = sr25519::Public::from_slice(old_account_id_pub_key.as_ref()) {
					let is_valid = sig.verify(&message.claim_message()[..], &signer);

					if is_valid {
						return ValidTransaction::with_tag_prefix("MigrateClaim")
							.priority(TransactionPriority::max_value())
							.propagate(true)
							.build();
					} else {
						return InvalidTransaction::BadSigner.into();
					}
				}
				todo!();
			} else {
				InvalidTransaction::Call.into()
			}
		}
	}
}

pub struct MessageType<'m> {
	pub chain_id: u64,
	pub old_account_id_pub_key: &'m AccountId32,
	pub new_account_id_pub_key: &'m H160,
}

impl<'m> MessageType<'m> {
	fn new(
		chain_id: u64,
		old_account_id_pub_key: &'m AccountId32,
		new_account_id_pub_key: &'m H160,
	) -> Self {
		Self { chain_id, old_account_id_pub_key, new_account_id_pub_key }
	}

	fn claim_message(&self) -> [u8; 32] {
		let mut result = Vec::new();
		result.extend_from_slice(&self.chain_id.to_be_bytes());
		result.extend_from_slice(self.old_account_id_pub_key.as_slice());
		result.extend_from_slice(&self.new_account_id_pub_key.0);

		let hash = blake2_256(&result[..]);
		hash
	}
}
