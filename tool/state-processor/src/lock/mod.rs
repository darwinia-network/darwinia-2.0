// darwinia
use crate::*;

impl State {
	pub fn process_lock(
		self,
		ring_locks: &mut Map<BalanceLock>,
		kton_locks: &mut Map<BalanceLock>,
	) -> Self {
		let s = self
			.take::<BalanceLock, _>(b"Balances", b"Locks", ring_locks, get_blake2_256_concat_suffix)
			.take::<BalanceLock, _>(b"Kton", b"Locks", kton_locks, get_blake2_256_concat_suffix);

		ring_locks.iter_mut().for_each(|(_, v)| v.amount *= GWEI);
		kton_locks.iter_mut().for_each(|(_, v)| v.amount *= GWEI);

		s
	}
}
