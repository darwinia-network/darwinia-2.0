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

// substrate
use sp_core::H160;
use sp_runtime::AccountId32;

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
	pub trait Config: frame_system::Config {}

	// Storage the migrated balance map from darwinia-1.0 chain
	#[pallet::storage]
	pub(super) type Balances<T> = StorageMap<_, Blake2_128Concat, AccountId32, u128>;

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Unsigned transaction
		#[pallet::weight(0)]
		pub fn claim_to(
			origin: OriginFor<T>, // remove this
			old_account_id: AccountId32,
			new_account_id: H160,
			sig: Signature,
			message: Vec<u8>,
		) -> DispatchResult {
			// verify signature

			// deposit to new_account_id

			// Update the balances storage

			// Add event
			todo!();
		}
	}
}
