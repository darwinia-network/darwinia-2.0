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
use array_bytes::hex2bytes;
use codec::{Decode, Encode, MaxEncodedLen};
use ethereum::{TransactionAction, TransactionSignature};
use rlp::RlpStream;
use sha3::{Digest, Keccak256};
// frontier
use fp_evm::{Precompile, PrecompileSet};
use pallet_ethereum::IntermediateStateRoot;
use pallet_evm::IdentityAddressMapping;
// substrate
use frame_support::{
	dispatch::RawOrigin,
	ensure,
	pallet_prelude::Weight,
	traits::{ConstU32, Everything},
	StorageHasher, Twox128,
};
use sp_core::{H160, H256, U256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, DispatchInfoOf, IdentifyAccount, IdentityLookup, Verify},
	transaction_validity::{InvalidTransaction, TransactionValidity, TransactionValidityError},
};
use sp_std::{marker::PhantomData, prelude::*};
// darwinia
use crate::*;
use bp_message_dispatch::{CallValidate, IntoDispatchOrigin as IntoDispatchOriginT};

pub type Block = frame_system::mocking::MockBlock<TestRuntime>;
pub type Balance = u64;
pub type AccountId = H160;
pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;

#[derive(
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Clone,
	Encode,
	Decode,
	Debug,
	MaxEncodedLen,
	scale_info::TypeInfo,
)]
pub enum Account {
	Alice,
	Bob,
	Charlie,
	Bogus,
	Precompile,
}

impl Default for Account {
	fn default() -> Self {
		Self::Bogus
	}
}

impl Into<H160> for Account {
	fn into(self) -> H160 {
		match self {
			Account::Alice => H160::repeat_byte(0xAA),
			Account::Bob => H160::repeat_byte(0xBB),
			Account::Charlie => H160::repeat_byte(0xCC),
			Account::Bogus => H160::repeat_byte(0xDD),
			Account::Precompile => H160::from_low_u64_be(1),
		}
	}
}

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

impl pallet_ethereum::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
	type StateRoot = IntermediateStateRoot<Self>;
}

pub struct MockAccountIdConverter;
impl sp_runtime::traits::Convert<H256, AccountId> for MockAccountIdConverter {
	fn convert(hash: H256) -> AccountId {
		hash.into()
	}
}

#[derive(Decode, Encode, Clone)]
pub struct MockEncodedCall(pub Vec<u8>);
impl From<MockEncodedCall> for Result<RuntimeCall, ()> {
	fn from(call: MockEncodedCall) -> Result<RuntimeCall, ()> {
		RuntimeCall::decode(&mut &call.0[..]).map_err(drop)
	}
}

pub struct MockCallValidator;
impl CallValidate<AccountId, RuntimeOrigin, RuntimeCall> for MockCallValidator {
	fn check_receiving_before_dispatch(
		relayer_account: &AccountId,
		call: &RuntimeCall,
	) -> Result<(), &'static str> {
		match call {
			RuntimeCall::MessageTransact(crate::Call::message_transact { transaction: tx }) => {
				let total_payment = crate::total_payment::<TestRuntime>(tx.into());
				let relayer = pallet_evm::Pallet::<TestRuntime>::account_basic(&relayer_account).0;

				ensure!(relayer.balance >= total_payment, "Insufficient balance");
				Ok(())
			},
			_ => Ok(()),
		}
	}

	fn call_validate(
		relayer_account: &AccountId,
		origin: &RuntimeOrigin,
		call: &RuntimeCall,
	) -> Result<(), TransactionValidityError> {
		match call {
			RuntimeCall::MessageTransact(crate::Call::message_transact { transaction: tx }) =>
				match origin.caller {
					OriginCaller::MessageTransact(LcmpEthOrigin::MessageTransact(id)) => {
						let total_payment = crate::total_payment::<TestRuntime>(tx.into());
						pallet_balances::Pallet::<TestRuntime>::transfer(
							RawOrigin::Signed(*relayer_account).into(),
							id,
							total_payment.as_u64(),
						)
						.map_err(|_| {
							TransactionValidityError::Invalid(InvalidTransaction::Payment)
						})?;

						Ok(())
					},
					_ => Err(TransactionValidityError::Invalid(InvalidTransaction::BadSigner)),
				},
			_ => Ok(()),
		}
	}
}
pub struct MockIntoDispatchOrigin;
impl IntoDispatchOriginT<AccountId, RuntimeCall, RuntimeOrigin> for MockIntoDispatchOrigin {
	fn into_dispatch_origin(id: &AccountId, call: &RuntimeCall) -> RuntimeOrigin {
		match call {
			RuntimeCall::MessageTransact(crate::Call::message_transact { .. }) =>
				crate::LcmpEthOrigin::MessageTransact(*id).into(),
			_ => frame_system::RawOrigin::Signed(*id).into(),
		}
	}
}
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
pub struct MockAccountPublic(AccountId);
impl IdentifyAccount for MockAccountPublic {
	type AccountId = AccountId;

	fn into_account(self) -> AccountId {
		self.0
	}
}
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
pub struct MockSignature(AccountId);
impl Verify for MockSignature {
	type Signer = MockAccountPublic;

	fn verify<L: sp_runtime::traits::Lazy<[u8]>>(&self, _msg: L, signer: &AccountId) -> bool {
		self.0 == *signer
	}
}

pub(crate) type MockBridgeMessageId = [u8; 4];

impl pallet_bridge_dispatch::Config for TestRuntime {
	type AccountIdConverter = MockAccountIdConverter;
	type BridgeMessageId = MockBridgeMessageId;
	type CallValidator = MockCallValidator;
	type EncodedCall = MockEncodedCall;
	type IntoDispatchOrigin = MockIntoDispatchOrigin;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type SourceChainAccountId = AccountId;
	type TargetChainAccountPublic = MockAccountPublic;
	type TargetChainSignature = MockSignature;
}

impl crate::Config for TestRuntime {
	type LcmpEthOrigin = crate::EnsureLcmpEthOrigin;
	type ValidatedTransaction = pallet_ethereum::ValidatedTransaction<Self>;
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
		Ethereum: pallet_ethereum::{Pallet, Call, Storage, Event, Origin},
		MessageTransact: crate::{Pallet, Call, Origin},
		Dispatch: pallet_bridge_dispatch::{Pallet, Call, Event<T>},
	}
}

pub struct AccountInfo {
	pub address: H160,
	pub private_key: H256,
}

fn address_build(seed: u8) -> AccountInfo {
	let raw_private_key = [seed + 1; 32];
	let secret_key = libsecp256k1::SecretKey::parse_slice(&raw_private_key).unwrap();
	let raw_public_key = &libsecp256k1::PublicKey::from_secret_key(&secret_key).serialize()[1..65];
	let raw_address = {
		let mut s = [0; 20];
		s.copy_from_slice(&Keccak256::digest(raw_public_key)[12..]);
		s
	};

	AccountInfo { private_key: raw_private_key.into(), address: raw_address.into() }
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext(accounts_len: usize) -> (Vec<AccountInfo>, sp_io::TestExternalities) {
	let mut t = frame_system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();

	let pairs = (0..accounts_len).map(|i| address_build(i as u8)).collect::<Vec<_>>();

	let balances: Vec<_> =
		(0..accounts_len).map(|i| (pairs[i].address.clone(), 100_000_000_000)).collect();

	pallet_balances::GenesisConfig::<TestRuntime> { balances }.assimilate_storage(&mut t).unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));

	(pairs, ext.into())
}

impl fp_self_contained::SelfContainedCall for RuntimeCall {
	type SignedInfo = H160;

	fn is_self_contained(&self) -> bool {
		match self {
			RuntimeCall::Ethereum(call) => call.is_self_contained(),
			_ => false,
		}
	}

	fn check_self_contained(&self) -> Option<Result<Self::SignedInfo, TransactionValidityError>> {
		match self {
			RuntimeCall::Ethereum(call) => call.check_self_contained(),
			_ => None,
		}
	}

	fn validate_self_contained(
		&self,
		info: &Self::SignedInfo,
		dispatch_info: &DispatchInfoOf<RuntimeCall>,
		len: usize,
	) -> Option<TransactionValidity> {
		match self {
			RuntimeCall::Ethereum(call) => call.validate_self_contained(info, dispatch_info, len),
			_ => None,
		}
	}

	fn pre_dispatch_self_contained(
		&self,
		info: &Self::SignedInfo,
		dispatch_info: &DispatchInfoOf<RuntimeCall>,
		len: usize,
	) -> Option<Result<(), TransactionValidityError>> {
		match self {
			RuntimeCall::Ethereum(call) =>
				call.pre_dispatch_self_contained(info, dispatch_info, len),
			_ => None,
		}
	}

	fn apply_self_contained(
		self,
		info: Self::SignedInfo,
	) -> Option<sp_runtime::DispatchResultWithInfo<sp_runtime::traits::PostDispatchInfoOf<Self>>> {
		use sp_runtime::traits::Dispatchable as _;
		match self {
			call @ RuntimeCall::Ethereum(pallet_ethereum::Call::transact { .. }) =>
				Some(call.dispatch(RuntimeOrigin::from(
					pallet_ethereum::RawOrigin::EthereumTransaction(info),
				))),
			_ => None,
		}
	}
}

pub struct LegacyUnsignedTransaction {
	pub nonce: U256,
	pub gas_price: U256,
	pub gas_limit: U256,
	pub action: TransactionAction,
	pub value: U256,
	pub input: Vec<u8>,
}
impl LegacyUnsignedTransaction {
	fn signing_rlp_append(&self, s: &mut RlpStream) {
		s.begin_list(9);
		s.append(&self.nonce);
		s.append(&self.gas_price);
		s.append(&self.gas_limit);
		s.append(&self.action);
		s.append(&self.value);
		s.append(&self.input);
		s.append(&ChainId::get());
		s.append(&0u8);
		s.append(&0u8);
	}

	fn signing_hash(&self) -> H256 {
		let mut stream = RlpStream::new();
		self.signing_rlp_append(&mut stream);
		H256::from_slice(&Keccak256::digest(&stream.out()).as_slice())
	}

	pub fn sign(&self, key: &H256) -> Transaction {
		self.sign_with_chain_id(key, ChainId::get())
	}

	pub fn sign_with_chain_id(&self, key: &H256, chain_id: u64) -> Transaction {
		let hash = self.signing_hash();
		let msg = libsecp256k1::Message::parse(hash.as_fixed_bytes());
		let s = libsecp256k1::sign(&msg, &libsecp256k1::SecretKey::parse_slice(&key[..]).unwrap());
		let sig = s.0.serialize();

		let sig = TransactionSignature::new(
			s.1.serialize() as u64 % 2 + chain_id * 2 + 35,
			H256::from_slice(&sig[0..32]),
			H256::from_slice(&sig[32..64]),
		)
		.unwrap();

		Transaction::Legacy(ethereum::LegacyTransaction {
			nonce: self.nonce,
			gas_price: self.gas_price,
			gas_limit: self.gas_limit,
			action: self.action,
			value: self.value,
			input: self.input.clone(),
			signature: sig,
		})
	}
}

pub struct EIP2930UnsignedTransaction {
	pub nonce: U256,
	pub gas_price: U256,
	pub gas_limit: U256,
	pub action: TransactionAction,
	pub value: U256,
	pub input: Vec<u8>,
}

impl EIP2930UnsignedTransaction {
	pub fn sign(&self, secret: &H256, chain_id: Option<u64>) -> Transaction {
		let secret = {
			let mut sk: [u8; 32] = [0u8; 32];
			sk.copy_from_slice(&secret[0..]);
			libsecp256k1::SecretKey::parse(&sk).unwrap()
		};
		let chain_id = chain_id.unwrap_or(ChainId::get());
		let msg = ethereum::EIP2930TransactionMessage {
			chain_id,
			nonce: self.nonce,
			gas_price: self.gas_price,
			gas_limit: self.gas_limit,
			action: self.action,
			value: self.value,
			input: self.input.clone(),
			access_list: vec![],
		};
		let signing_message = libsecp256k1::Message::parse_slice(&msg.hash()[..]).unwrap();

		let (signature, recid) = libsecp256k1::sign(&signing_message, &secret);
		let rs = signature.serialize();
		let r = H256::from_slice(&rs[0..32]);
		let s = H256::from_slice(&rs[32..64]);
		Transaction::EIP2930(ethereum::EIP2930Transaction {
			chain_id: msg.chain_id,
			nonce: msg.nonce,
			gas_price: msg.gas_price,
			gas_limit: msg.gas_limit,
			action: msg.action,
			value: msg.value,
			input: msg.input.clone(),
			access_list: msg.access_list,
			odd_y_parity: recid.serialize() != 0,
			r,

			s,
		})
	}
}

pub struct EIP1559UnsignedTransaction {
	pub nonce: U256,
	pub max_priority_fee_per_gas: U256,
	pub max_fee_per_gas: U256,
	pub gas_limit: U256,
	pub action: TransactionAction,
	pub value: U256,
	pub input: Vec<u8>,
}

impl EIP1559UnsignedTransaction {
	pub fn sign(&self, secret: &H256, chain_id: Option<u64>) -> Transaction {
		let secret = {
			let mut sk: [u8; 32] = [0u8; 32];
			sk.copy_from_slice(&secret[0..]);
			libsecp256k1::SecretKey::parse(&sk).unwrap()
		};
		let chain_id = chain_id.unwrap_or(ChainId::get());
		let msg = ethereum::EIP1559TransactionMessage {
			chain_id,
			nonce: self.nonce,
			max_priority_fee_per_gas: self.max_priority_fee_per_gas,
			max_fee_per_gas: self.max_fee_per_gas,
			gas_limit: self.gas_limit,
			action: self.action,
			value: self.value,
			input: self.input.clone(),
			access_list: vec![],
		};
		let signing_message = libsecp256k1::Message::parse_slice(&msg.hash()[..]).unwrap();

		let (signature, recid) = libsecp256k1::sign(&signing_message, &secret);
		let rs = signature.serialize();
		let r = H256::from_slice(&rs[0..32]);
		let s = H256::from_slice(&rs[32..64]);
		Transaction::EIP1559(ethereum::EIP1559Transaction {
			chain_id: msg.chain_id,
			nonce: msg.nonce,
			max_priority_fee_per_gas: msg.max_priority_fee_per_gas,
			max_fee_per_gas: msg.max_fee_per_gas,
			gas_limit: msg.gas_limit,
			action: msg.action,
			value: msg.value,
			input: msg.input.clone(),
			access_list: msg.access_list,
			odd_y_parity: recid.serialize() != 0,
			r,
			s,
		})
	}
}
