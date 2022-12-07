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

// substrate
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_core::{
	crypto::ByteArray,
	sr25519::{Public, Signature},
	H160, H256,
};
use sp_io::hashing::blake2_256;
use sp_runtime::traits::Verify;
use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_evm::Config {
		/// Override the [`frame_system::Config::RuntimeEvent`].
		type RuntimeEvent: From<Event> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event {
		/// An account has been migrated.
		Migrated { from: H256, to: H160 },
	}

	#[pallet::genesis_config]
	#[cfg_attr(feature = "std", derive(Default))]
	pub struct GenesisConfig {}
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// since signature and chain_id verification is done in `validate_unsigned`
		// we can skip doing it here again.
		// TODO: update weight
		#[pallet::weight(0)]
		pub fn migrate(
			origin: OriginFor<T>,
			_chain_id: u64,
			from: H256,
			to: H160,
			_sig: Signature,
		) -> DispatchResult {
			ensure_none(origin)?;

			// TODO

			Self::deposit_event(Event::Migrated { from, to });

			Ok(())
		}
	}
	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			let Call::migrate { chain_id, from, to, sig } = call else {
				return InvalidTransaction::Call.into();
			};

			if *chain_id != <T as pallet_evm::Config>::ChainId::get() {
				return InvalidTransaction::BadProof.into();
			}
			// Check if exist
			// if !Balances::<T>::contains_key(from) {
			// 	return InvalidTransaction::BadSigner.into();
			// }

			let message = ClaimMessage::new(<T as pallet_evm::Config>::ChainId::get(), from, to);
			if let Ok(signer) = Public::from_slice(from.as_ref()) {
				let is_valid = sig.verify(&blake2_256(&message.raw_bytes())[..], &signer);

				if is_valid {
					return ValidTransaction::with_tag_prefix("MigrateClaim")
						.priority(TransactionPriority::max_value())
						.propagate(true)
						.build();
				}
			}
			InvalidTransaction::BadSigner.into()
		}
	}
}
pub use pallet::*;

/// ClaimMessage is the metadata that needs to be signed when the user invokes claim dispatch.
///
/// It consists of three parts, namely the chain_id, the H256 account for darwinia 1.0, and
/// the H160 account for darwinia 2.0.
pub struct ClaimMessage<'m> {
	pub chain_id: u64,
	pub from: &'m H256,
	pub to: &'m H160,
}

impl<'m> ClaimMessage<'m> {
	fn new(chain_id: u64, from: &'m H256, to: &'m H160) -> Self {
		Self { chain_id, from, to }
	}

	fn raw_bytes(&self) -> Vec<u8> {
		let mut result = Vec::new();
		result.extend_from_slice(&self.chain_id.to_be_bytes());
		result.extend_from_slice(self.from.as_ref());
		result.extend_from_slice(self.to.as_ref());
		result
	}
}
