// darwinia
use crate::*;

impl pallet_multisig::Config for Runtime {
	type Currency = Balances;
	// One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
	type DepositBase = ConstU128<{ darwinia_deposit(1, 88) }>;
	// Additional storage item size of 32 bytes.
	type DepositFactor = ConstU128<{ darwinia_deposit(0, 32) }>;
	type RuntimeEvent = RuntimeEvent;
	type MaxSignatories = ConstU16<100>;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = ();
}
