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

impl pallet_multisig::Config for Runtime {
	type Currency = Balances;
	// One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
	type DepositBase = ConstU128<{ darwinia_deposit(1, 88) }>;
	// Additional storage item size of 32 bytes.
	type DepositFactor = ConstU128<{ darwinia_deposit(0, 32) }>;
	type RuntimeEvent = RuntimeEvent;
	type MaxSignatories = ConstU16<100>;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = ();
}