// darwinia
use crate::*;

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
