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

// darwinia
use crate::*;

pub struct TODO;
impl darwinia_staking::Stake for TODO {
	type AccountId = AccountId;
	type Item = Balance;

	fn stake(_: &Self::AccountId, _: Self::Item) -> frame_support::pallet_prelude::DispatchResult {
		Ok(())
	}

	fn unstake(
		_: &Self::AccountId,
		_: Self::Item,
	) -> frame_support::pallet_prelude::DispatchResult {
		Ok(())
	}
}
impl darwinia_staking::StakeExt for TODO {
	type Amount = Balance;

	fn amount(_: &Self::AccountId, _: Self::Item) -> Self::Amount {
		0
	}
}

frame_support::parameter_types! {
	pub const PayoutFraction: sp_runtime::Perbill = sp_runtime::Perbill::from_percent(20);
}

impl darwinia_staking::Config for Runtime {
	type Currency = Balances;
	type Deposit = TODO;
	type Kton = TODO;
	type MaxDeposits = ConstU32<16>;
	type MaxUnstakings = ConstU32<16>;
	type PayoutFraction = PayoutFraction;
	type RewardRemainder = Treasury;
	type Ring = TODO;
	type RuntimeEvent = RuntimeEvent;
	type StakeAtLeast = ConstU32<{ 14 * DAYS }>;
	type UnixTime = pallet_timestamp::Pallet<Self>;
}
