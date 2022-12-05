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
use dc_types::{AssetId, Balance, Moment, UNIT};
// substrate
use sp_runtime::RuntimeAppPublic;

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

impl pallet_timestamp::Config for Runtime {
	type MinimumPeriod = ();
	type Moment = Moment;
	type OnTimestampSet = ();
	type WeightInfo = ();
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

impl pallet_assets::Config for Runtime {
	type ApprovalDeposit = ();
	type AssetAccountDeposit = ();
	type AssetDeposit = ();
	type AssetId = AssetId;
	type Balance = Balance;
	type Currency = Balances;
	type Extra = ();
	type ForceOrigin = frame_system::EnsureRoot<u32>;
	type Freezer = ();
	type MetadataDepositBase = ();
	type MetadataDepositPerByte = ();
	type RuntimeEvent = RuntimeEvent;
	type StringLimit = frame_support::traits::ConstU32<4>;
	type WeightInfo = ();
}

frame_support::parameter_types! {
	pub static Time: core::time::Duration = core::time::Duration::new(0, 0);
}
impl Time {
	pub fn run(milli_secs: Moment) {
		Time::mutate(|t| *t += core::time::Duration::from_millis(milli_secs as _));
	}
}
impl frame_support::traits::UnixTime for Time {
	fn now() -> core::time::Duration {
		Time::get()
	}
}
pub enum KtonMinting {}
impl darwinia_deposit::Minting for KtonMinting {
	type AccountId = u32;

	fn mint(beneficiary: &Self::AccountId, amount: Balance) -> sp_runtime::DispatchResult {
		Assets::mint(RuntimeOrigin::signed(0), 0, *beneficiary, amount)
	}
}
impl darwinia_deposit::Config for Runtime {
	type Kton = KtonMinting;
	type MaxDeposits = frame_support::traits::ConstU32<16>;
	type MinLockingAmount = frame_support::traits::ConstU128<UNIT>;
	type Ring = Balances;
	type RuntimeEvent = RuntimeEvent;
	type UnixTime = Time;
}

pub enum RingStaking {}
impl darwinia_staking::Stake for RingStaking {
	type AccountId = u32;
	type Item = Balance;

	fn stake(who: &Self::AccountId, item: Self::Item) -> sp_runtime::DispatchResult {
		<Balances as frame_support::traits::Currency<_>>::transfer(
			who,
			&darwinia_staking::account_id(),
			item,
			frame_support::traits::ExistenceRequirement::KeepAlive,
		)
	}

	fn unstake(who: &Self::AccountId, item: Self::Item) -> sp_runtime::DispatchResult {
		<Balances as frame_support::traits::Currency<_>>::transfer(
			&darwinia_staking::account_id(),
			who,
			item,
			frame_support::traits::ExistenceRequirement::AllowDeath,
		)
	}
}

frame_support::parameter_types! {
	pub static SessionHandlerCollators: Vec<u32> = Vec::new();
	pub static SessionChangeBlock: u64 = 0;
}
sp_runtime::impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: sp_runtime::testing::UintAuthorityId,
	}
}
type Period = frame_support::traits::ConstU64<3>;
pub struct TestSessionHandler;
impl pallet_session::SessionHandler<u32> for TestSessionHandler {
	const KEY_TYPE_IDS: &'static [sp_runtime::KeyTypeId] =
		&[sp_runtime::testing::UintAuthorityId::ID];

	fn on_genesis_session<Ks: sp_runtime::traits::OpaqueKeys>(keys: &[(u32, Ks)]) {
		SessionHandlerCollators::set(keys.into_iter().map(|(a, _)| *a).collect::<Vec<_>>())
	}

	fn on_new_session<Ks: sp_runtime::traits::OpaqueKeys>(
		_: bool,
		keys: &[(u32, Ks)],
		_: &[(u32, Ks)],
	) {
		SessionChangeBlock::set(System::block_number());
		SessionHandlerCollators::set(keys.into_iter().map(|(a, _)| *a).collect::<Vec<_>>())
	}

	fn on_before_session_ending() {}

	fn on_disabled(_: u32) {}
}
impl pallet_session::Config for Runtime {
	type Keys = SessionKeys;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, ()>;
	type RuntimeEvent = RuntimeEvent;
	type SessionHandler = pallet_session::TestSessionHandler;
	type SessionManager = Staking;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, ()>;
	type ValidatorId = Self::AccountId;
	type ValidatorIdOf = darwinia_staking::IdentityCollator;
	type WeightInfo = ();
}

pub enum KtonStaking {}
impl darwinia_staking::Stake for KtonStaking {
	type AccountId = u32;
	type Item = Balance;

	fn stake(who: &Self::AccountId, item: Self::Item) -> sp_runtime::DispatchResult {
		Assets::transfer(RuntimeOrigin::signed(*who), 0, darwinia_staking::account_id(), item)
	}

	fn unstake(who: &Self::AccountId, item: Self::Item) -> sp_runtime::DispatchResult {
		Assets::transfer(RuntimeOrigin::signed(darwinia_staking::account_id()), 0, *who, item)
	}
}
frame_support::parameter_types! {
	pub const PayoutFraction: sp_runtime::Perbill = sp_runtime::Perbill::from_percent(20);
}
impl darwinia_staking::Config for Runtime {
	type Deposit = Deposit;
	type Kton = KtonStaking;
	type MaxDeposits = frame_support::traits::ConstU32<16>;
	type MaxUnstakings = frame_support::traits::ConstU32<16>;
	type MinStakingDuration = frame_support::traits::ConstU64<3>;
	type PayoutFraction = PayoutFraction;
	type RewardRemainder = ();
	type Ring = RingStaking;
	type RingCurrency = Balances;
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
		Timestamp: pallet_timestamp,
		Balances: pallet_balances,
		Assets: pallet_assets,
		Deposit: darwinia_deposit,
		Session: pallet_session,
		Staking: darwinia_staking,
	}
);

pub enum Efflux {}
impl Efflux {
	pub fn time(milli_secs: Moment) {
		Timestamp::set_timestamp(Timestamp::now() + milli_secs);
	}

	pub fn block(number: u64) {
		for _ in 0..number {
			initialize_block(System::block_number() + 1)
		}
	}
}

fn initialize_block(number: u64) {
	System::set_block_number(number);
	Efflux::time(1);
	<AllPalletsWithSystem as frame_support::traits::OnInitialize<u64>>::on_initialize(number);
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	// substrate
	use frame_support::traits::GenesisBuild;

	let mut storage = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: (1..=4).map(|i| (i, (i as Balance) * 1_000 * UNIT)).collect(),
	}
	.assimilate_storage(&mut storage)
	.unwrap();
	pallet_assets::GenesisConfig::<Runtime> {
		assets: vec![(0, 0, true, 1)],
		metadata: vec![(0, b"KTON".to_vec(), b"KTON".to_vec(), 18)],
		accounts: (1..=4).map(|i| (0, i, (i as Balance) * 1_000 * UNIT)).collect(),
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	let mut ext = sp_io::TestExternalities::from(storage);

	ext.execute_with(|| initialize_block(1));

	ext
}
