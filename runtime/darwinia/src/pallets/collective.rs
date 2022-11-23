pub use pallet_collective::{Instance1 as CouncilCollective, Instance2 as TechnicalCollective};

// darwinia
use crate::*;

pub const COLLECTIVE_DESIRED_MEMBERS: u32 = 7;
const COLLECTIVE_MAX_MEMBERS: u32 = 100;

// Make sure that there are no more than `COLLECTIVE_MAX_MEMBERS` members elected via phragmen.
static_assertions::const_assert!(COLLECTIVE_DESIRED_MEMBERS <= COLLECTIVE_MAX_MEMBERS);

impl pallet_collective::Config<CouncilCollective> for Runtime {
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type MaxMembers = ConstU32<COLLECTIVE_MAX_MEMBERS>;
	type MaxProposals = ConstU32<100>;
	type MotionDuration = ConstU32<{ 3 * DAYS }>;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type WeightInfo = ();
}
impl pallet_collective::Config<TechnicalCollective> for Runtime {
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type MaxMembers = ConstU32<COLLECTIVE_MAX_MEMBERS>;
	type MaxProposals = ConstU32<100>;
	type MotionDuration = ConstU32<{ 3 * DAYS }>;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type WeightInfo = ();
}
