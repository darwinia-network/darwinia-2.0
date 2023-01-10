// This file is part of Darwinia.
//
// Copyright (C) 2018-2023 Darwinia Network
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

/// List of the assets existed in this runtime.
pub enum AssetIds {
	Kton = 1026,
}

impl pallet_assets::Config for Runtime {
	type ApprovalDeposit = ConstU128<0>;
	type AssetAccountDeposit = ConstU128<0>;
	type AssetDeposit = ConstU128<0>;
	type AssetId = AssetId;
	type Balance = Balance;
	type CreateOrigin = EnsureRoot<AccountId>;
	type Currency = Balances;
	type Extra = ();
	type ForceOrigin = EnsureRoot<AccountId>;
	type Freezer = ();
	type MetadataDepositBase = ConstU128<0>;
	type MetadataDepositPerByte = ConstU128<0>;
	type RuntimeEvent = RuntimeEvent;
	type StringLimit = ConstU32<50>;
	type WeightInfo = ();
}
