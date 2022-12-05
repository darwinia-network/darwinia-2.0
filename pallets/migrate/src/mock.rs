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

//! Test utilities

// crates.io
use codec::{Decode, Encode, MaxEncodedLen};
// frontier
use pallet_evm::IdentityAddressMapping;
// parity
#[cfg(feature = "std")]
use frame_support::traits::GenesisBuild;
use frame_support::{
	pallet_prelude::Weight,
	traits::{ConstU32, Everything},
};
use frame_system::EnsureRoot;
use sp_core::{H160, H256, U256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	AccountId32, BuildStorage,
};
use sp_std::{marker::PhantomData, prelude::*};
// darwinia
use crate::{self as darwinia_migrate};

pub type Block = frame_system::mocking::MockBlock<TestRuntime>;
pub type Balance = u128;
pub type AccountId = H160;
pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;

frame_support::parameter_types! {
	pub const BlockHashCount: u64 = 250;
}
impl frame_system::Config for TestRuntime {
	type AccountData = pallet_balances::AccountData<Balance>;
	type AccountId = AccountId;
	type BaseCallFilter = Everything;
	type BlockHashCount = ();
	type BlockLength = ();
	type BlockNumber = u64;
	type BlockWeights = ();
	type DbWeight = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = ConstU32<16>;
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

frame_support::parameter_types! {
	pub const MaxLocks: u32 = 10;
	pub const ExistentialDeposit: u64 = 0;
}
impl pallet_balances::Config for TestRuntime {
	type AccountStore = System;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

frame_support::parameter_types! {
	pub const MinimumPeriod: u64 = 6000 / 2;
}
impl pallet_timestamp::Config for TestRuntime {
	type MinimumPeriod = MinimumPeriod;
	type Moment = u64;
	type OnTimestampSet = ();
	type WeightInfo = ();
}

frame_support::parameter_types! {
	pub const TransactionByteFee: u64 = 1;
	pub const ChainId: u64 = 42;
	pub const BlockGasLimit: U256 = U256::MAX;
	pub const WeightPerGas: Weight = Weight::from_ref_time(20_000);
}

impl pallet_evm::Config for TestRuntime {
	type AddressMapping = IdentityAddressMapping;
	type BlockGasLimit = BlockGasLimit;
	type BlockHashMapping = pallet_evm::SubstrateBlockHashMapping<Self>;
	type CallOrigin = pallet_evm::EnsureAddressRoot<AccountId>;
	type ChainId = ChainId;
	type Currency = Balances;
	type FeeCalculator = ();
	type FindAuthor = ();
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type OnChargeTransaction = ();
	type PrecompilesType = ();
	type PrecompilesValue = ();
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type RuntimeEvent = RuntimeEvent;
	type WeightPerGas = WeightPerGas;
	type WithdrawOrigin = pallet_evm::EnsureAddressNever<AccountId>;
}

impl darwinia_migrate::Config for TestRuntime {
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
}

frame_support::construct_runtime! {
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		EVM: pallet_evm::{Pallet, Call, Storage, Config, Event<T>},
		Migrate: darwinia_migrate::{Pallet, Call, Storage, Config, Event},
	}
}

#[derive(Default)]
pub(crate) struct ExtBuilder {
	migrated_accounts: Vec<(AccountId32, Balance)>,
}

impl ExtBuilder {
	pub(crate) fn with_migrated_accounts(mut self, accounts: Vec<(AccountId32, Balance)>) -> Self {
		self.migrated_accounts = accounts;
		self
	}

	pub(crate) fn build(self) -> sp_io::TestExternalities {
		let t = GenesisConfig {
			system: Default::default(),
			balances: Default::default(),
			evm: Default::default(),
			migrate: darwinia_migrate::GenesisConfig { migrated_accounts: self.migrated_accounts },
		}
		.build_storage()
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
