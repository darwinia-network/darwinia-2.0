use crate::*;

frame_support::parameter_types! {
	/// Relay Chain `TransactionByteFee` / 10
	pub const TransactionByteFee: Balance = 10 * MICROUNIT;
	pub const OperationalFeeMultiplier: u8 = 5;
}

impl pallet_transaction_payment::Config for Runtime {
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type RuntimeEvent = RuntimeEvent;
	type WeightToFee = WeightToFee;
}
