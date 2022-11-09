use crate::*;

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type ChannelInfo = ParachainSystem;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type VersionWrapper = ();
	type WeightInfo = weights::cumulus_pallet_xcmp_queue::WeightInfo<Self>;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}