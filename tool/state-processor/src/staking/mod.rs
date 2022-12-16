// darwinia
use crate::*;

impl Processor {
	pub fn process_staking(&mut self) -> &mut Self {
		let mut solo_block_number = u32::default();
		// Storage items.
		// https://github.dev/darwinia-network/darwinia-common/blob/6a9392cfb9fe2c99b1c2b47d0c36125d61991bb7/frame/staking/src/lib.rs#L611
		let mut ledgers = <Map<StakingLedger>>::default();
		let mut ring_pool = u128::default();
		let mut kton_pool = u128::default();

		log::info!("take solo `Staking::Ledger`, `Staking::RingPool` and `Staking::KtonPool`");
		self.solo_state
			.take_map(b"Staking", b"Ledger", &mut ledgers, get_identity_key)
			.take_value(b"Staking", b"RingPool", "", &mut ring_pool)
			.take_value(b"Staking", b"KtonPool", "", &mut kton_pool);

		log::info!("get solo `System::Number`");
		self.solo_state.get_value(b"System", b"Number", "", &mut solo_block_number);

		log::info!("adjust decimals, convert ledger, adjust unstaking duration then set `AccountMigration::Ledgers`");
		{
			let staking_ik = item_key(b"Staking", b"Ledgers");
			let deposit_ik = item_key(b"Deposit", b"Deposits");

			ledgers.into_iter().for_each(|(_, v)| {
				let hash_k = array_bytes::bytes2hex("", subhasher::blake2_128_concat(v.stash));
				let deposit_k = format!("{deposit_ik}{hash_k}");
				let staking_k = format!("{staking_ik}{hash_k}");
				let mut staked_deposits = Vec::default();

				if !v.deposit_items.is_empty() {
					// TODO: account references
					self.shell_state.insert_raw_key_value(
						deposit_k,
						v.deposit_items
							.into_iter()
							.enumerate()
							.map(|(i, d)| {
								let i = i as _;

								staked_deposits.push(i);

								Deposit {
									id: i,
									value: adjust_decimals(d.value),
									expired_time: d.expire_time as _,
									in_use: true,
								}
							})
							.collect::<Vec<_>>(),
					);
				}
				self.shell_state.insert_raw_key_value(
					staking_k,
					Ledger {
						staked_ring: v.active,
						staked_kton: v.active_kton,
						staked_deposits,
						unstaking_ring: v
							.ring_staking_lock
							.unbondings
							.into_iter()
							.map(|u| {
								(
									adjust_decimals(u.amount),
									adjust_block_time(solo_block_number, u.until),
								)
							})
							.collect(),
						unstaking_kton: v
							.kton_staking_lock
							.unbondings
							.into_iter()
							.map(|u| {
								(
									adjust_decimals(u.amount),
									adjust_block_time(solo_block_number, u.until),
								)
							})
							.collect(),
						unstaking_deposits: Default::default(),
					},
				);
			});
		}

		log::info!("adjust `ring_pool` and `kton_pool` balance decimals");
		ring_pool = adjust_decimals(ring_pool);
		kton_pool = adjust_decimals(kton_pool);

		log::info!("{ring_pool}");
		log::info!("{kton_pool}");

		self
	}
}
