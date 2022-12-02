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
use crate::{self as darwinia_deposit, *};
use dc_types::UNIT;

impl frame_system::Config for Runtime {
	type AccountData = pallet_balances::AccountData<Balance>;
	type AccountId = u32;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockHashCount = ();
	type BlockLength = ();
	type BlockNumber = u64;
	type BlockWeights = ();
	type DbWeight = ();
	type Hash = sp_core::H256;
	type Hashing = sp_runtime::traits::BlakeTwo256;
	type Header = sp_runtime::testing::Header;
	type Index = u64;
	type Lookup = sp_runtime::traits::IdentityLookup<Self::AccountId>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type PalletInfo = PalletInfo;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SS58Prefix = ();
	type SystemWeightInfo = ();
	type Version = ();
}

impl pallet_balances::Config for Runtime {
	type AccountStore = System;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = frame_support::traits::ConstU128<0>;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}
impl pallet_balances::Config<pallet_balances::Instance1> for Runtime {
	type AccountStore = frame_support::traits::StorageMapShim<
		pallet_balances::Account<Runtime>,
		frame_system::Provider<Runtime>,
		u32,
		pallet_balances::AccountData<Balance>,
	>;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = frame_support::traits::ConstU128<0>;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

frame_support::parameter_types! {
	pub static Time: core::time::Duration = core::time::Duration::new(0, 0);
}
impl Time {
	pub(crate) fn run(milli_secs: Timestamp) {
		TIME.with(|v| *v.borrow_mut() += core::time::Duration::from_millis(milli_secs as _));
	}
}
impl UnixTime for Time {
	fn now() -> core::time::Duration {
		Time::get()
	}
}
pub enum KtonMinting {}
impl Minting for KtonMinting {
	type AccountId = u32;

	fn mint(beneficiary: &Self::AccountId, amount: Balance) -> DispatchResult {
		let _ = Kton::deposit_creating(beneficiary, amount);

		Ok(())
	}
}
impl darwinia_deposit::Config for Runtime {
	type Kton = KtonMinting;
	type MaxDeposits = frame_support::traits::ConstU32<16>;
	type MinLockAmount = frame_support::traits::ConstU128<UNIT>;
	type Ring = Balances;
	type RuntimeEvent = RuntimeEvent;
	type UnixTime = Time;
}

frame_support::construct_runtime!(
	pub enum Runtime where
		Block = frame_system::mocking::MockBlock<Runtime>,
		NodeBlock = frame_system::mocking::MockBlock<Runtime>,
		UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>,
	{
		System: frame_system,
		Balances: pallet_balances,
		Kton: pallet_balances::<Instance1>,
		Deposit: darwinia_deposit,
	}
);

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: (1..=2).map(|i| (i, (i as Balance) * 1_000 * UNIT)).collect(),
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	storage.into()
}
