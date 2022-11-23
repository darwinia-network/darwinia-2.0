// darwinia
use crate::*;

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	codec::Encode,
	codec::Decode,
	codec::MaxEncodedLen,
	scale_info::TypeInfo,
	sp_runtime::RuntimeDebug,
)]
pub enum ProxyType {
	Any,
	NonTransfer,
	Governance,
	IdentityJudgement,
	EthereumBridge,
}
impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}
impl frame_support::traits::InstanceFilter<RuntimeCall> for ProxyType {
	// TODO: configure filter
	fn filter(&self, _c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => true,
			ProxyType::Governance => true,
			ProxyType::IdentityJudgement => true,
			ProxyType::EthereumBridge => true,
		}
	}

	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			(ProxyType::NonTransfer, _) => true,
			_ => false,
		}
	}
}

impl pallet_proxy::Config for Runtime {
	type AnnouncementDepositBase = ConstU128<{ darwinia_deposit(1, 8) }>;
	type AnnouncementDepositFactor = ConstU128<{ darwinia_deposit(0, 66) }>;
	type CallHasher = Hashing;
	type Currency = Balances;
	type MaxPending = ConstU32<32>;
	type MaxProxies = ConstU32<32>;
	// One storage item; key size 32, value size 8; .
	type ProxyDepositBase = ConstU128<{ darwinia_deposit(1, 8) }>;
	// Additional storage item size of 33 bytes.
	type ProxyDepositFactor = ConstU128<{ darwinia_deposit(0, 33) }>;
	type ProxyType = ProxyType;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}
