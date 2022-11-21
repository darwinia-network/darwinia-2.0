// darwinia
use crate::*;

impl State {
	pub fn process_lock(
		self,
		ring_locks: &mut Map<BalanceLock>,
		kton_locks: &mut Map<BalanceLock>,
	) -> Self {
		let s = self
			.take::<BalanceLock>(b"Balances", b"Locks", ring_locks)
			.take::<BalanceLock>(b"Kton", b"Locks", kton_locks);

		ring_locks.iter_mut().for_each(|(_, v)| v.amount *= GWEI);
		kton_locks.iter_mut().for_each(|(_, v)| v.amount *= GWEI);

		s
	}
}
