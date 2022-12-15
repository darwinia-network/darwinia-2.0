// darwinia
use crate::*;

impl Processor {
	pub fn process_staking(&mut self) -> &mut Self {
		// Storage items.
		// https://github.dev/darwinia-network/darwinia-common/blob/6a9392cfb9fe2c99b1c2b47d0c36125d61991bb7/frame/staking/src/lib.rs#L611
		let mut ledgers = <Map<StakingLedger>>::default();
		let mut ring_pool = u128::default();
		let mut kton_pool = u128::default();

		log::info!("take solo `Staking::Ledger`, `Staking::RingPool` and `Staking::KtonPool`");
		self.solo_state
			.take_map(b"Staking", b"Ledger", &mut ledgers, get_hashed_key)
			.take_value(b"Staking", b"RingPool", &mut ring_pool)
			.take_value(b"Staking", b"KtonPool", &mut kton_pool);

		log::info!("set `AccountMigration::Ledgers`");
		// self.shell_state
		// .insert_map(ledgers, |k| format!("{}{k}", item_key(b"AccountMigration", b"Ledgers")));

		log::info!("adjust `ring_pool` and `kton_pool` balance decimals");
		ring_pool *= GWEI;
		kton_pool *= GWEI;

		log::info!("{ring_pool}");
		log::info!("{kton_pool}");

		self
	}
}
