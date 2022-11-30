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
// Darwinia is distributed in_use the hope that it will be useful,
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

// darwinia
use dc_inflation::MILLISECS_PER_YEAR;
use dc_types::{Balance, Timestamp};

// substrate
use frame_support::{pallet_prelude::*, traits::UnixTime};
use frame_system::pallet_prelude::*;

type DepositId = u8;

/// Deposit.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, RuntimeDebug)]
pub struct Deposit {
	/// Deposit ID.
	pub id: DepositId,
	/// Deposited RING.
	pub value: Balance,
	/// Expired timestamp.
	pub expired_time: Timestamp,
	/// Deposit state.
	pub in_use: bool,
}

#[frame_support::pallet]
pub mod pallet {
	// darwinia
	use crate::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Override the [`frame_system::Config::RuntimeEvent`].
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Unix time getter.
		type UnixTime: UnixTime;

		/// Maximum deposit count.
		#[pallet::constant]
		type MaxDeposits: Get<u32>;
	}

	#[pallet::event]
	// TODO: event?
	// #[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		/// Lock at least for one month.
		LockAtLeastOneMonth,
		/// Exceed maximum deposit count.
		ExceedMaxDeposits,
		/// Deposit not found.
		DepositNotFound,
		/// Deposit is in use.
		DepositInUse,
		/// Deposit is not in use.
		DepositNotInUse,
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
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Lock the RING for some KTON profit/interest.
		#[pallet::weight(0)]
		pub fn lock(origin: OriginFor<T>, amount: Balance, months: u8) -> DispatchResult {
			let who = ensure_signed(origin)?;

			if month == 0 {
				Err(<Error<T>>::LockAtLeastOneMonth)?;
			}

			<Deposits<T>>::try_mutate(&who, |ds| {
				ds.try_push(Deposit {
					// TODO: unique identifier
					id: 0,
					value: amount,
					expired_time: MILLISECS_PER_YEAR / months as Timestamp,
					in_use: false,
				})
				.map_err(|_| <Error<T>>::ExceedMaxDeposits)
			})?;

			// TODO: transfer

			// TODO: mint
			let interest = dc_inflation::deposit_interest(amount, months);

			// TODO: event?

			Ok(())
		}

		/// Claim the expired-locked RING.
		#[pallet::weight(0)]
		pub fn claim(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let now = T::UnixTime::now().as_millis();
			let mut claimed = 0;

			<Deposits<T>>::mutate(&who, |ds| {
				ds.retain(|d| {
					if d.expired_time >= now && !d.in_use {
						claimed += d.value;

						false
					} else {
						true
					}
				});
			});

			// TODO: withdraw

			// TODO: event?

			Ok(())
		}

		// TODO: claim_with_penalty
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
			    return DispatchResult::Err(<Error<T>>::DepositNotFound.into());
			};

			if d.in_use {
				Err(<Error<T>>::DepositInUse.into())
			} else {
				d.in_use = true;

				Ok(())
			}
		})
	}

	fn unstake(who: &Self::AccountId, item: Self::Item) -> DispatchResult {
		<Deposits<T>>::try_mutate(who, |ds| {
			let Some(d) = ds.iter_mut().find(|d| d.id == item) else {
			    return DispatchResult::Err(<Error<T>>::DepositNotFound.into());
			};

			if d.in_use {
				d.in_use = false;

				Ok(())
			} else {
				Err(<Error<T>>::DepositNotInUse.into())
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
