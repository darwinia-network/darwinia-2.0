// darwinia
use crate::*;

impl Processor {
	pub fn process_balances(&mut self) -> &mut Self {
		let mut ring_locks = Map::default();
		let mut kton_locks = Map::default();

		log::info!("take balance locks");
		self.solo_state
			.take::<Vec<BalanceLock>, _>(b"Balances", b"Locks", &mut ring_locks, get_hashed_key)
			.take::<Vec<BalanceLock>, _>(b"Kton", b"Locks", &mut kton_locks, get_hashed_key);

		log::info!("prune ring locks");
		prune(ring_locks);
		log::info!("prune kton locks");
		prune(kton_locks);

		self
	}
}

fn prune(locks: Map<Vec<BalanceLock>>) {
	// https://github.dev/darwinia-network/darwinia-common/blob/6a9392cfb9fe2c99b1c2b47d0c36125d61991bb7/frame/staking/src/primitives.rs#L39
	const STAKING: [u8; 8] = *b"da/staki";
	// https://github.dev/darwinia-network/darwinia/blob/2d1c1436594b2c397d450e317c35eb16c71105d6/runtime/crab/src/pallets/elections_phragmen.rs#L8
	const PHRAGMEN_ELECTION: [u8; 8] = *b"phrelect";
	// https://github.dev/paritytech/substrate/blob/19162e43be45817b44c7d48e50d03f074f60fbf4/frame/democracy/src/lib.rs#L190
	const DEMOCRACY: [u8; 8] = *b"democrac";
	// https://github.dev/paritytech/substrate/blob/19162e43be45817b44c7d48e50d03f074f60fbf4/frame/vesting/src/lib.rs#L86
	const VESTING: [u8; 8] = *b"vesting ";
	// https://github.dev/darwinia-network/darwinia/blob/2d1c1436594b2c397d450e317c35eb16c71105d6/runtime/crab/src/pallets/fee_market.rs#L35
	const FEE_MARKET_0: [u8; 8] = *b"da/feelf";
	// https://github.dev/darwinia-network/darwinia/blob/2d1c1436594b2c397d450e317c35eb16c71105d6/runtime/crab/src/pallets/fee_market.rs#L36
	const FEE_MARKET_1: [u8; 8] = *b"da/feecp";
	// https://github.dev/darwinia-network/darwinia/blob/2d1c1436594b2c397d450e317c35eb16c71105d6/runtime/darwinia/src/pallets/fee_market.rs#L37
	const FEE_MARKET_2: [u8; 8] = *b"da/feedp";

	locks.into_iter().for_each(|(k, v)| {
		v.into_iter().for_each(|l| match l.id {
			STAKING | PHRAGMEN_ELECTION | DEMOCRACY | VESTING | FEE_MARKET_0 | FEE_MARKET_1
			| FEE_MARKET_2 => (),
			id => log::error!(
				"Encountered unknown lock id({}) of account({})",
				String::from_utf8_lossy(&id),
				get_last_64(&k)
			),
		})
	});
}
