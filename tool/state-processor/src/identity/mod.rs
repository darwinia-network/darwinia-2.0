// darwinia
use crate::*;

// TODO: Note this value is different between pangolin/crab and darwinia network.
// Pangolin: https://github.com/darwinia-network/darwinia-common/blob/main/node/runtime/pangolin/src/pallets/identity.rs#L10
// Crab: https://github.com/darwinia-network/darwinia/blob/main/runtime/crab/src/pallets/identity.rs#L10
// Darwinia: https://github.com/darwinia-network/darwinia/blob/main/runtime/darwinia/src/pallets/identity.rs#L10
const SUB_ACCOUNT_DEPOSIT: u128 = 2 * GWEI;

impl<S> Processor<S> {
	/// Only care about the solo chain, since parachains don't have identity now.
	pub fn process_identity(&mut self) -> &mut Self {
		let mut identities = <Map<Registration>>::default();
		let mut registrars = Vec::<Option<RegistrarInfo>>::default();
		let mut super_of = Map::<([u8; 32], Data)>::default();
		let mut subs_of = Map::<(u128, Vec<[u8; 32]>)>::default();

		log::info!("take `Identity::IdentityOf`, `Identity::Registrars`, `Identity::SuperOf` and `Identity::SuperOf`.");
		self.solo_state
			.take_map(b"Identity", b"IdentityOf", &mut identities, get_hashed_key)
			.take_value(b"Identity", b"Registrars", "", &mut registrars)
			.take_map(b"Identity", b"SuperOf", &mut super_of, get_last_64_key)
			.take_map(b"Identity", b"SubsOf", &mut subs_of, get_hashed_key);

		log::info!("update identities's deposit and judgement decimal.");
		identities.iter_mut().for_each(|(_k, v)| {
			v.adjust();
		});
		log::info!("update registrars fee decimal.");
		registrars.iter_mut().for_each(|registar_info| match registar_info {
			Some(info) => {
				info.adjust();
			},
			None => {
				log::error!(
					"This gonna not matched, no request_judgement or provide_judgement in this case."
				);
			},
		});

		log::info!("analyze `Identity::SuperOf` and `Identity::SubsOf` and update identity's reserved balance.");
		subs_of.iter_mut().for_each(|(_, (subs_deposit, sub_ids))| {
			for sub_id in sub_ids {
				if let Some((super_id, _)) = super_of.get(&array_bytes::bytes2hex("0x", sub_id)) {

					// `Identity::SubsOf` use `Twox64Concat` hash, calc the key manually.
					self.shell_state
						.map
						.entry(full_key(
							b"AccountMigration",
							b"Accounts",
							&blake2_128_concat_to_string(&super_id),
						))
						.and_modify(|v| {
							let mut info = decode::<AccountInfo>(v).expect("Never happened!");
							let deposit = SUB_ACCOUNT_DEPOSIT.min(*subs_deposit);
							*subs_deposit -= deposit;

							info.data.reserved -= deposit * GWEI;
						});
				}
			}
		});

		log::info!("set `AccountMigration::IdentityOf` and`AccountMigration::Registrars`.");
		self.shell_state
			.insert_map(identities, |k| full_key(b"AccountMigration", b"IdentityOf", k));
		self.shell_state.insert_value(b"AccountMigration", b"Registrars", "", registrars);

		self
	}
}
