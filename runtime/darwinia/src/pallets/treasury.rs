// darwinia
use crate::*;

frame_support::parameter_types! {
	pub const TreasuryPalletId: frame_support::PalletId = frame_support::PalletId(*b"da/trsry");
	pub const ProposalBond: sp_runtime::Permill = sp_runtime::Permill::from_percent(5);
	pub const Burn: sp_runtime::Permill = sp_runtime::Permill::from_percent(1);
}

// In order to use `Tips`, which bounded by `pallet_treasury::Config` rather
// `pallet_treasury::Config<I>` Still use `DefaultInstance` here instead `Instance1`
impl pallet_treasury::Config for Runtime {
	type ApproveOrigin = RootOrAtLeastThreeFifth<CouncilCollective>;
	type Burn = Burn;
	type BurnDestination = ();
	type Currency = Balances;
	type MaxApprovals = ConstU32<100>;
	type OnSlash = Treasury;
	type PalletId = TreasuryPalletId;
	type ProposalBond = ProposalBond;
	type ProposalBondMaximum = ();
	type ProposalBondMinimum = ConstU128<DARWINIA_PROPOSAL_REQUIREMENT>;
	type RejectOrigin = RootOrMoreThanHalf<CouncilCollective>;
	type RuntimeEvent = RuntimeEvent;
	type SpendFunds = ();
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
	type SpendPeriod = ConstU32<{ 24 * DAYS }>;
	type WeightInfo = ();
}
