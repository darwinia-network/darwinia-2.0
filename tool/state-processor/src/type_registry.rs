// crates.io
use parity_scale_codec::{Decode, Encode};

pub const GWEI: u128 = 1_000_000_000;
pub const KTON_ID: u64 = 1026;

#[derive(Debug, Encode, Decode)]
pub struct AccountInfo {
	pub nonce: u32,
	pub consumers: u32,
	pub providers: u32,
	pub sufficients: u32,
	pub data: AccountData,
}

#[derive(Debug, Encode, Decode)]
pub struct AccountData {
	pub free: u128,
	pub reserved: u128,
	pub free_kton_or_misc_frozen: u128,
	pub reserved_kton_or_fee_frozen: u128,
}

#[derive(Debug, Encode, Decode)]
pub struct BalanceLock {
	pub id: [u8; 8],
	pub amount: u128,
	pub reasons: Reasons,
}

#[allow(clippy::unnecessary_cast)]
#[derive(Debug, PartialEq, Eq, Encode, Decode)]
pub enum Reasons {
	Fee = 0,
	Misc = 1,
	All = 2,
}

#[derive(Debug, Encode, Decode)]
pub struct Deposit {
	pub id: u8,
	pub value: u128,
	pub expired_time: u128,
	pub in_use: bool,
}

#[derive(Debug, Encode, Decode)]
pub struct StakingLedger {
	pub stash: [u8; 32],
	#[codec(compact)]
	pub active: u128,
	#[codec(compact)]
	pub active_deposit_ring: u128,
	#[codec(compact)]
	pub active_kton: u128,
	pub deposit_items: Vec<TimeDepositItem>,
	pub ring_staking_lock: StakingLock,
	pub kton_staking_lock: StakingLock,
	pub claimed_rewards: Vec<u32>,
}

#[derive(Debug, Encode, Decode)]
pub struct TimeDepositItem {
	#[codec(compact)]
	pub value: u128,
	#[codec(compact)]
	pub start_time: u64,
	#[codec(compact)]
	pub expire_time: u64,
}

#[derive(Debug, Encode, Decode)]
pub struct StakingLock {
	pub staking_amount: u128,
	pub unbondings: Vec<Unbonding>,
}

#[derive(Debug, Encode, Decode)]
pub struct Unbonding {
	pub amount: u128,
	pub until: u32,
}

#[derive(Debug, Encode, Decode)]
pub struct Ledger {
	pub staked_ring: u128,
	pub staked_kton: u128,
	pub staked_deposits: Vec<u8>,
	pub unstaking_ring: Vec<(u128, u32)>,
	pub unstaking_kton: Vec<(u128, u32)>,
	pub unstaking_deposits: Vec<(u8, u32)>,
}

// https://github.dev/paritytech/substrate/blob/polkadot-v0.9.30/frame/assets/src/types.rs#L33
#[derive(Debug, Encode, Decode)]
pub struct AssetDetails {
	pub owner: [u8; 20],
	pub issuer: [u8; 20],
	pub admin: [u8; 20],
	pub freezer: [u8; 20],
	pub supply: u128,
	pub deposit: u128,
	pub min_balance: u128,
	pub is_sufficient: bool,
	pub accounts: u32,
	pub sufficients: u32,
	pub approvals: u32,
	pub is_frozen: bool,
}

// https://github.dev/paritytech/substrate/blob/polkadot-v0.9.30/frame/assets/src/types.rs#L115
#[derive(Debug, Encode, Decode)]
pub struct AssetAccount {
	pub balance: u128,
	pub is_frozen: bool,
	pub reason: ExistenceReason,
	pub extra: (),
}

// https://github.dev/paritytech/substrate/blob/polkadot-v0.9.30/frame/assets/src/types.rs#L88
#[derive(Debug, Encode, Decode)]
pub enum ExistenceReason {
	#[codec(index = 0)]
	Consumer,
	#[codec(index = 1)]
	Sufficient,
	#[codec(index = 2)]
	DepositHeld(u128),
	#[codec(index = 3)]
	DepositRefunded,
}

// https://github.dev/paritytech/substrate/blob/polkadot-v0.9.30/frame/assets/src/types.rs#L73
#[derive(Debug, Encode, Decode)]
pub struct Approval {
	pub amount: u128,
	pub deposit: u128,
}

// https://github.dev/paritytech/substrate/blob/polkadot-v0.9.30/frame/assets/src/types.rs#L127
#[derive(Clone, Encode, Decode)]
pub struct AssetMetadata {
	pub deposit: u128,
	pub name: Vec<u8>,
	pub symbol: Vec<u8>,
	pub decimals: u8,
	pub is_frozen: bool,
}
