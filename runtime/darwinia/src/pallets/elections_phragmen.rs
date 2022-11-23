// darwinia
use crate::*;

const MAX_CANDIDATES: u32 = 30;

frame_support::parameter_types! {
	pub const PhragmenElectionPalletId: frame_support::traits::LockIdentifier = *b"phrelect";
}

impl pallet_elections_phragmen::Config for Runtime {
	type CandidacyBond = ConstU128<{ 100 * MILLIUNIT }>;
	type ChangeMembers = Council;
	type Currency = Balances;
	type CurrencyToVote = frame_support::traits::U128CurrencyToVote;
	type DesiredMembers = ConstU32<COLLECTIVE_DESIRED_MEMBERS>;
	type DesiredRunnersUp = ConstU32<7>;
	type InitializeMembers = Council;
	type KickedMember = Treasury;
	type LoserCandidate = Treasury;
	type MaxCandidates = ConstU32<MAX_CANDIDATES>;
	type MaxVoters = ConstU32<{ 10 * MAX_CANDIDATES }>;
	type PalletId = PhragmenElectionPalletId;
	type RuntimeEvent = RuntimeEvent;
	// Daily council elections.
	type TermDuration = ConstU32<{ 7 * DAYS }>;
	// 1 storage item created, key size is 32 bytes, value size is 16+16.
	type VotingBondBase = ConstU128<{ darwinia_deposit(1, 64) }>;
	// Additional data per vote is 32 bytes (account id).
	type VotingBondFactor = ConstU128<{ darwinia_deposit(0, 32) }>;
	type WeightInfo = ();
}
