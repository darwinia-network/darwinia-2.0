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

//! # Darwinia deposit pallet
//!
//! This is a completely specialized deposit pallet designed only for Darwinia parachain.
//! So, this pallet will eliminate the generic parameters as much as possible.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod weights;
pub use weights::WeightInfo;

// core
use core::time::Duration;

// darwinia
use dc_types::{Balance, Timestamp};

// substrate
use frame_support::{log, pallet_prelude::*, traits::UnixTime};
use frame_system::pallet_prelude::*;

type DepositId = u8;

/// Deposit.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, RuntimeDebug)]
#[scale_info(skip_type_params(T))]
pub struct Deposit {
	/// Deposit ID.
	pub id: DepositId,
	/// Deposited RING.
	pub value: Balance,
	/// Expired timestamp.
	pub expired_time: Timestamp,
	/// Deposit state.
	pub in_used: bool,
}

#[frame_support::pallet]
pub mod pallet {
	// darwinia
	use crate::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Override the [`frame_system::Config::RuntimeEvent`].
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Unix timestamp.
		type UnixTime: UnixTime;

		/// Maximum deposit count.
		#[pallet::constant]
		type MaxDeposits: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Dummy.
		Dummy,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Exceed maximum deposit count.
		ExceedMaxDeposits,
	}

	/// All deposits.
	#[pallet::storage]
	#[pallet::getter(fn deposit_of)]
	pub type Deposits<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<Deposit, T::MaxDeposits>,
		ValueQuery,
	>;

	#[derive(Default)]
	#[pallet::genesis_config]
	pub struct GenesisConfig {
		// TODO
	}
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
			Default::default()
		}
	}
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// TODO
		#[pallet::weight(0)]
		pub fn lock(origin: OriginFor<T>, amount: Balance, month: u8) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// TODO: transfer to pallet account

			Ok(())
		}

		/// TODO
		#[pallet::weight(0)]
		pub fn claim(origin: OriginFor<T>, deposit_id: DepositId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let d =
				<Deposits<T>>::get(&who).into_iter().find(|d| d.id == deposit_id).ok_or("TODO")?;

			if d.expired_time < time(T::UnixTime::now()) {
				Err("")?;
			}

			// TODO: withdraw from pallet account

			Ok(())
		}
	}
	impl<T> Pallet<T> where T: Config {}

	fn time(duration: Duration) -> Timestamp {
		duration.as_millis()
	}
}
pub use pallet::*;

impl<T> darwinia_staking::Stake for Pallet<T>
where
	T: Config,
{
	type AccountId = T::AccountId;
	type Item = DepositId;

	fn stake(who: &Self::AccountId, item: Self::Item) -> DispatchResult {
		<Deposits<T>>::try_mutate(who, |ds| {
			let Some(d) = ds.iter_mut().find(|d| d.id == item) else {
			    return DispatchResult::Err("TODO".into());
			};

			if d.in_used {
				Err("TODO".into())
			} else {
				d.in_used = true;

				Ok(())
			}
		})
	}

	fn unstake(who: &Self::AccountId, item: Self::Item) -> DispatchResult {
		<Deposits<T>>::try_mutate(who, |ds| {
			let Some(d) = ds.iter_mut().find(|d| d.id == item) else {
			    return DispatchResult::Err("TODO".into());
			};

			if d.in_used {
				d.in_used = false;

				Ok(())
			} else {
				Err("TODO".into())
			}
		})
	}
}
impl<T> darwinia_staking::StakeExt for Pallet<T>
where
	T: Config,
{
	type Amount = Balance;

	fn amount(who: &Self::AccountId, item: Self::Item) -> Self::Amount {
		<Deposits<T>>::get(who)
			.into_iter()
			.find_map(|d| if d.id == item { Some(d.value) } else { None })
			.unwrap_or_default()
	}
}
