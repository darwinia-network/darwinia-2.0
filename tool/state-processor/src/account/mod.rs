// darwinia
use crate::*;

#[derive(Debug)]
pub struct FlatAccount {
	pub public_key: String,
	pub nonce: u32,
	pub consumers: u32,
	pub providers: u32,
	pub sufficients: u32,
	pub ring: u128,
	pub ring_reserved: u128,
	pub ring_locks: Option<BalanceLock>,
	pub kton: u128,
	pub kton_reserved: u128,
	pub kton_locks: Option<BalanceLock>,
}

impl State {
	pub fn process_account(self, account_infos: &mut Map<AccountInfo>) -> Self {
		let mut remaining_ring = Map::default();
		let mut remaining_kton = Map::default();
		let s = self
			.take::<AccountInfo>(b"System", b"Account", account_infos)
			.take::<u128>(b"Ethereum", b"RemainingRingBalance", &mut remaining_ring)
			.take::<u128>(b"Ethereum", b"RemainingKtonBalance", &mut remaining_kton);

		account_infos.iter_mut().for_each(|(_, v)| {
			v.data.free *= GWEI;
			v.data.reserved *= GWEI;
			v.data.free_kton *= GWEI;
			v.data.reserved_kton *= GWEI;
		});
		remaining_ring.into_iter().for_each(|(k, v)| {
			if let Some(a) = account_infos.get_mut(&k) {
				a.data.free += v;
			} else {
				log::warn!("`RemainingRingBalance({k})` not found");
			}
		});
		remaining_kton.into_iter().for_each(|(k, v)| {
			if let Some(a) = account_infos.get_mut(&k) {
				a.data.free_kton += v;
			} else {
				log::warn!("`RemainingKtonBalance({k})` not found");
			}
		});

		s
	}
}

pub fn flatten(
	account_infos: Map<AccountInfo>,
	mut ring_locks: Map<BalanceLock>,
	mut kton_locks: Map<BalanceLock>,
) -> Vec<FlatAccount> {
	let account_details = account_infos
		.into_iter()
		.map(|(k, v)| {
			let ring_locks = ring_locks.remove(&k);
			let kton_locks = kton_locks.remove(&k);

			FlatAccount {
				public_key: k,
				nonce: v.nonce,
				consumers: v.consumers,
				providers: v.providers,
				sufficients: v.sufficients,
				ring: v.data.free,
				ring_reserved: v.data.reserved,
				ring_locks,
				kton: v.data.free_kton,
				kton_reserved: v.data.reserved_kton,
				kton_locks,
			}
		})
		.collect();

	ring_locks.into_iter().for_each(|(k, _)| log::warn!("ring_locks' owner({k}) dropped"));
	kton_locks.into_iter().for_each(|(k, _)| log::warn!("kton_locks' owner({k}) dropped"));

	account_details
}
