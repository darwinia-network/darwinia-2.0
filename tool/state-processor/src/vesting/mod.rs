// darwinia
use crate::*;

impl<S> Processor<S> {
	pub fn process_vesting(&mut self) -> &mut Self {
		// Storage items.
		// https://github.dev/darwinia-network/substrate/blob/darwinia-v0.12.5/frame/vesting/src/lib.rs#L188
		let mut vestings = <Map<VestingInfo>>::default();

		log::info!("take solo `Vesting::Vesting`");
		self.solo_state.take_map(b"Vesting", b"Vesting", &mut vestings, get_hashed_key);

		log::info!("adjust solo `VestingInfo`s");
		vestings.iter_mut().for_each(|(_, v)| v.adjust());

		log::info!("set `AccountMigration::Vestings`");
		vestings.into_iter().for_each(|(k, v)| {
			let a = get_last_64(&k);

			self.shell_state.inc_consumers(a);
			self.shell_state.insert_value(b"AccountMigration", b"Vestings", &k, v);
		});

		self
	}
}
