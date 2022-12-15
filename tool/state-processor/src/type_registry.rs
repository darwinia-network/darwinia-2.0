// crates.io
use parity_scale_codec::{Decode, Encode};

pub const GWEI: u128 = 1_000_000_000;

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
pub struct StakingLedger {
	stash: [u8; 32],
	#[codec(compact)]
	active: u128,
	#[codec(compact)]
	active_deposit_ring: u128,
	#[codec(compact)]
	active_kton: u128,
	deposit_items: Vec<TimeDepositItem>,
	ring_staking_lock: StakingLock,
	kton_staking_lock: StakingLock,
	claimed_rewards: Vec<u32>,
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
	staking_amount: u128,
	unbondings: Vec<Unbonding>,
}
#[derive(Debug, Encode, Decode)]
pub struct Unbonding {
	amount: u128,
	until: u32,
}

#[derive(Debug, Encode, Decode)]
pub struct Ledger {
	/// Staked RING.
	pub staked_ring: Balance,
	/// Staked KTON.
	pub staked_kton: Balance,
	/// Staked deposits.
	pub staked_deposits: BoundedVec<DepositId<T>, T::MaxDeposits>,
	/// The RING in unstaking process.
	pub unstaking_ring: BoundedVec<(Balance, T::BlockNumber), T::MaxUnstakings>,
	/// The KTON in unstaking process.
	pub unstaking_kton: BoundedVec<(Balance, T::BlockNumber), T::MaxUnstakings>,
	/// The deposit in unstaking process.
	pub unstaking_deposits: BoundedVec<(DepositId<T>, T::BlockNumber), T::MaxUnstakings>,
}
