// crates.io
use parity_scale_codec::Decode;

pub const GWEI: u128 = 1_000_000_000;

#[derive(Debug, Decode)]
pub struct AccountInfo {
	pub nonce: u32,
	pub consumers: u32,
	pub providers: u32,
	pub sufficients: u32,
	pub data: AccountData,
}
#[derive(Debug, Decode)]
pub struct AccountData {
	pub free: u128,
	pub reserved: u128,
	pub free_kton: u128,
	pub reserved_kton: u128,
}

#[derive(Debug, Decode)]
pub struct BalanceLock {
	pub id: [u8; 8],
	pub amount: u128,
	pub reasons: Reasons,
}
#[derive(Debug, Decode)]
pub enum Reasons {
	Fee = 0,
	Misc = 1,
	All = 2,
}
