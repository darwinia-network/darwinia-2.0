// darwinia
use crate::*;

const MAX_PENDING_PERIOD: BlockNumber = 100;
const SYNC_INTERVAL: BlockNumber = 10;

frame_support::parameter_types! {
	pub const SignThreshold: sp_runtime::Perbill = sp_runtime::Perbill::from_percent(60);
}
static_assertions::const_assert!(MAX_PENDING_PERIOD > SYNC_INTERVAL);

impl darwinia_ecdsa_authority::Config for Runtime {
	type ChainId = <Self as pallet_evm::Config>::ChainId;
	type MaxAuthorities = ConstU32<3>;
	type MaxPendingPeriod = ConstU32<MAX_PENDING_PERIOD>;
	type MessageRoot = darwinia_message_gadget::MessageRootGetter<Self>;
	type RuntimeEvent = RuntimeEvent;
	type SignThreshold = SignThreshold;
	type SyncInterval = ConstU32<SYNC_INTERVAL>;
	type WeightInfo = ();
}
