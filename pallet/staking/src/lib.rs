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

//! # Darwinia parachain staking

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod weights;
pub use weights::WeightInfo;

// substrate
use frame_support::pallet_prelude::*;

#[frame_support::pallet]
pub mod pallet {
	// darwinia
	use crate::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Override the [`frame_system::Config::RuntimeEvent`].
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Dummy.
		Dummy,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Dummy.
		Dummy,
	}

	/// Dummy.
	#[pallet::storage]
	#[pallet::getter(fn dummy)]
	pub type Dummy<T: Config> = StorageValue<_, (), ValueQuery>;

	#[derive(Default)]
	#[pallet::genesis_config]
	pub struct GenesisConfig {}
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {}
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);
	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(now: T::BlockNumber) -> Weight {
			0
		}
	}
	#[pallet::call]
	impl<T> Pallet<T> where T: Config {

	}
	impl<T> Pallet<T> where T: Config {}
}
pub use pallet::*;
