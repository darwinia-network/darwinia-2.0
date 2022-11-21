// crates.io
use parity_scale_codec::Decode;

pub type Nonce = u32;
pub type Balance = u128;

pub type RefCount = u32;

pub const GWEI: Balance = 1_000_000_000;

#[derive(Debug, Decode)]
pub struct AccountInfo {
	pub nonce: Nonce,
	pub consumers: RefCount,
	pub providers: RefCount,
	pub sufficients: RefCount,
	pub data: AccountData,
}
#[derive(Debug, Decode)]
pub struct AccountData {
	pub free: Balance,
	pub reserved: Balance,
	pub free_kton: Balance,
	pub reserved_kton: Balance,
}
