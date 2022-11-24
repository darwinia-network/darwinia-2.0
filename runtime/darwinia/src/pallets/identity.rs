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

impl pallet_identity::Config for Runtime {
	// Minimum 100 bytes/UNIT deposited (1 MILLIUNIT/byte).
	// 258 bytes on-chain.
	type BasicDeposit = ConstU128<{ darwinia_deposit(1, 258) }>;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	// 66 bytes on-chain.
	type FieldDeposit = ConstU128<{ darwinia_deposit(0, 66) }>;
	type ForceOrigin = RootOrMoreThanHalf<CouncilCollective>;
	type MaxAdditionalFields = ConstU32<100>;
	type MaxRegistrars = ConstU32<20>;
	type MaxSubAccounts = ConstU32<100>;
	type RegistrarOrigin = RootOrMoreThanHalf<CouncilCollective>;
	type Slashed = Treasury;
	// 53 bytes on-chain.
	type SubAccountDeposit = ConstU128<{ darwinia_deposit(1, 53) }>;
	type WeightInfo = ();
}
