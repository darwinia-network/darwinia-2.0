// darwinia
use crate::*;

impl pallet_identity::Config for Runtime {
	// Minimum 100 bytes/UNIT deposited (1 MILLIUNIT/byte).
	// 258 bytes on-chain.
	type BasicDeposit = ConstU128<{ darwinia_deposit(1, 258) }>;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	// 66 bytes on-chain.
	type FieldDeposit = ConstU128<{ darwinia_deposit(0, 66) }>;
	type ForceOrigin = RootOrMoreThanHalf<CouncilCollective>;
	type MaxAdditionalFields = ConstU32<100>;
	type MaxRegistrars = ConstU32<20>;
	type MaxSubAccounts = ConstU32<100>;
	type RegistrarOrigin = RootOrMoreThanHalf<CouncilCollective>;
	type Slashed = Treasury;
	// 53 bytes on-chain.
	type SubAccountDeposit = ConstU128<{ darwinia_deposit(1, 53) }>;
	type WeightInfo = ();
}
