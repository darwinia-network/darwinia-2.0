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

// std
use std::iter;
// crates.io
use libsecp256k1::{Message, PublicKey, SecretKey};
// darwinia
use darwinia_ecdsa_authority::{primitives::*, *};
use dc_primitives::AccountId;
// substrate
use frame_support::traits::{GenesisBuild, OnInitialize};
use sp_io::{hashing, TestExternalities};

frame_support::parameter_types! {
	pub Version: sp_version::RuntimeVersion = sp_version::RuntimeVersion {
		spec_name: sp_runtime::RuntimeString::Owned("Darwinia".into()),
		..Default::default()
	};
}
impl frame_system::Config for Runtime {
	type AccountData = ();
	type AccountId = AccountId;
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
	type Version = Version;
}

frame_support::parameter_types! {
	pub const ChainId: &'static [u8] = b"46";
	pub const MaxAuthorities: u32 = 3;
	pub const MaxPendingPeriod: u32 = 5;
	pub const SignThreshold: sp_runtime::Perbill = sp_runtime::Perbill::from_percent(60);
	pub const SyncInterval: u32 = 3;
	pub static MessageRoot: Option<darwinia_ecdsa_authority::primitives::Hash> = Some(Default::default());
}
impl Config for Runtime {
	type ChainId = ChainId;
	type MaxAuthorities = MaxAuthorities;
	type MaxPendingPeriod = MaxPendingPeriod;
	type MessageRoot = MessageRoot;
	type RuntimeEvent = RuntimeEvent;
	type SignThreshold = SignThreshold;
	type SyncInterval = SyncInterval;
	type WeightInfo = ();
}

frame_support::construct_runtime! {
	pub enum Runtime
	where
		Block = frame_system::mocking::MockBlock<Runtime>,
		NodeBlock = frame_system::mocking::MockBlock<Runtime>,
		UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>,
	{
		System: frame_system,
		EcdsaAuthority: darwinia_ecdsa_authority,
	}
}

#[derive(Default)]
pub struct ExtBuilder {
	authorities: Vec<AccountId>,
}
impl ExtBuilder {
	pub fn authorities(mut self, authorities: Vec<AccountId>) -> Self {
		self.authorities = authorities;

		self
	}

	pub fn build(self) -> TestExternalities {
		let Self { authorities } = self;
		let mut storage =
			frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

		darwinia_ecdsa_authority::GenesisConfig::<Runtime> { authorities }
			.assimilate_storage(&mut storage)
			.unwrap();

		let mut ext = TestExternalities::from(storage);

		ext.execute_with(|| {
			System::set_block_number(1);
			<EcdsaAuthority as OnInitialize<_>>::on_initialize(1);
		});

		ext
	}
}

pub fn gen_pair(byte: u8) -> (SecretKey, AccountId) {
	let seed = iter::repeat(byte).take(32).collect::<Vec<_>>();
	let secret_key = SecretKey::parse_slice(&seed).unwrap();
	let public_key = PublicKey::from_secret_key(&secret_key).serialize();
	let address =
		array_bytes::slice_n_into_unchecked(&hashing::keccak_256(&public_key[1..65])[12..]);

	(secret_key, address)
}

pub fn sign(secret_key: &SecretKey, message: &[u8; 32]) -> Signature {
	let (sig, recovery_id) = libsecp256k1::sign(&Message::parse(message), secret_key);
	let mut signature = [0u8; 65];

	signature[0..64].copy_from_slice(&sig.serialize()[..]);
	signature[64] = recovery_id.serialize();

	Signature(signature)
}

pub fn presume_authority_change_succeed() {
	EcdsaAuthority::apply_next_authorities();
}

pub fn message_root_of(byte: u8) -> Hash {
	Hash::repeat_byte(byte)
}

pub fn new_message_root(byte: u8) {
	MESSAGE_ROOT.with(|v| *v.borrow_mut() = Some(message_root_of(byte)));
}

pub fn run_to_block(n: u64) {
	for b in System::block_number() + 1..=n {
		System::set_block_number(b);
		<EcdsaAuthority as OnInitialize<_>>::on_initialize(b);
	}
}

pub fn ecdsa_authority_events() -> Vec<Event<Runtime>> {
	fn events() -> Vec<RuntimeEvent> {
		let events = System::events().into_iter().map(|evt| evt.event).collect::<Vec<_>>();

		System::reset_events();

		events
	}

	events()
		.into_iter()
		.filter_map(|e| match e {
			RuntimeEvent::EcdsaAuthority(e) => Some(e),
			_ => None,
		})
		.collect::<Vec<_>>()
}
