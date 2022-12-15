// darwinia
use crate::*;

use array_bytes::bytes2hex;
use frame_support::{traits::ConstU32, BoundedVec};
use pallet_identity::{Data, Judgement, RegistrarInfo, Registration};
use sp_core::crypto::AccountId32;
use subhasher::blake2_128_concat;

// TODO: Note this value is different between pangolin/crab and darwinia network.
// Pangolin: https://github.com/darwinia-network/darwinia-common/blob/main/node/runtime/pangolin/src/pallets/identity.rs#L10
// Crab: https://github.com/darwinia-network/darwinia/blob/main/runtime/crab/src/pallets/identity.rs#L10
// Darwinia: https://github.com/darwinia-network/darwinia/blob/main/runtime/darwinia/src/pallets/identity.rs#L10
const SUB_ACCOUNT_DEPOSIT: u128 = 2 * GWEI;

impl Processor {
	/// Only care about the solo chain, since parachains don't have identity now.
	pub fn process_identity(&mut self) -> &mut Self {
		let mut identities = <Map<Registration<u128, ConstU32<20>, ConstU32<100>>>>::default();
		let mut registrars =
			BoundedVec::<Option<RegistrarInfo<u128, AccountId32>>, ConstU32<20>>::default();
		let mut super_of = Map::<(AccountId32, Data)>::default();
		let mut subs_of = Map::<(u128, BoundedVec<AccountId32, ConstU32<100>>)>::default();

		log::info!("take `Identity::IdentityOf`, `Identity::Registrars`, `Identity::SuperOf` and `Identity::SuperOf`.");
		self.solo_state
			.take_map(b"Identity", b"IdentityOf", &mut identities, get_hashed_key)
			.take_value(b"Identity", b"Registrars", &mut registrars)
			.take_map(b"Identity", b"SuperOf", &mut super_of, get_hashed_key)
			.take_map(b"Identity", b"SubsOf", &mut subs_of, get_hashed_key);

		log::info!("update identities's deposit and judgement decimal.");
		identities.iter_mut().for_each(|(_k, v)| {
			v.deposit = v.deposit * GWEI;
			v.judgements.iter_mut().for_each(|(_, judgement)| {
				if let Judgement::FeePaid(amount) = judgement {
					*amount = *amount * GWEI;
				}
			});
		});
		log::info!("update registrars fee decimal.");
		registrars.iter_mut().for_each(|registar_info| match registar_info {
			Some(info) => {
				info.fee = info.fee * GWEI;
			},
			None => {
				log::error!("This gonna not matched, no request_judgement or provide_judgement in this case.");
			},
		});
		log::info!("analyze `Identity::SuperOf` and `Identity::SubsOf` and update identity's reserved balance.");
		subs_of.iter_mut().for_each(|(_, (subs_deposit, sub_ids))| {
			for sub_id in sub_ids {
				let sub_hash = &bytes2hex("", &blake2_128_concat(&sub_id.encode()));
				if let Some((super_id, _)) = super_of.get(sub_hash) {
					// SubsOf item use Twox64Concat hash, have to calculate the hash key manually
					let super_hash = &bytes2hex("", &blake2_128_concat(&super_id.encode()));

					self.shell_state
						.0
						.entry(full_key(b"AccountMigration", b"Accounts", &super_hash))
						.and_modify(|v| {
							let mut info =
								decode::<AccountInfo>(v).expect("The account should existed!");
							let deposit = SUB_ACCOUNT_DEPOSIT.min(*subs_deposit);
							*subs_deposit -= deposit;

							info.data.reserved -= deposit * GWEI;
						});
				}
			}
		});

		log::info!("set `AccountMigration::IdentityOf` and`AccountMigration::Registrars`.");
		identities.iter().for_each(|(k, v)| {
			self.shell_state
				.0
				.insert(full_key(b"AccountMigration", b"IdentityOf", &k), encode_value(v));
		});
		self.shell_state
			.0
			.insert(item_key(b"AccountMigration", b"Registrars"), encode_value(registrars));

		self
	}
}
