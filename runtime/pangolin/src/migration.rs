// darwinia
#[allow(unused_imports)]
use crate::*;
// substrate
use codec::{Decode, Encode};
use frame_support::{
	migration::{
		get_storage_value, put_storage_value, storage_key_iter_with_suffix, take_storage_value,
	},
	Blake2_128Concat, StorageHasher,
};

#[derive(Encode, Decode)]
pub struct AssetAccount {
	balance: Balance,
	is_frozen: bool,
	reason: ExistenceReason,
	extra: (),
}
#[derive(Encode, Decode)]
pub enum ExistenceReason {
	#[codec(index = 0)]
	Consumer,
	#[codec(index = 1)]
	Sufficient,
	#[codec(index = 2)]
	DepositHeld(Balance),
	#[codec(index = 3)]
	DepositRefunded,
}
#[derive(Encode, Decode)]
struct AssetDetails {
	owner: AccountId,
	issuer: AccountId,
	admin: AccountId,
	freezer: AccountId,
	supply: Balance,
	deposit: Balance,
	min_balance: Balance,
	is_sufficient: bool,
	accounts: u32,
	sufficients: u32,
	approvals: u32,
	status: AssetStatus,
}
#[derive(Encode, Decode)]
enum AssetStatus {
	Live,
	Frozen,
	Destroying,
}

pub struct CustomOnRuntimeUpgrade;
impl frame_support::traits::OnRuntimeUpgrade for CustomOnRuntimeUpgrade {
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		Ok(Vec::new())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), &'static str> {
		let actual_accounts =
			storage_key_iter_with_suffix::<AccountId, AssetAccount, Blake2_128Concat>(
				b"Assets",
				b"Account",
				&Blake2_128Concat::hash(&(AssetIds::PKton as u64).encode()),
			)
			.count();

		let asset_detail = migration::get_storage_value::<AssetDetails>(
			b"Assets",
			b"Asset",
			&Blake2_128Concat::hash(&(AssetIds::PKton as u64).encode()),
		)
		.unwrap();

		assert_eq!(actual_accounts as u32, asset_detail.accounts);
		assert_eq!(actual_accounts as u32, asset_detail.sufficients);
		Ok(())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		migrate()
	}
}

fn migrate() -> frame_support::weights::Weight {
	let actual_accounts =
		storage_key_iter_with_suffix::<AccountId, AssetAccount, Blake2_128Concat>(
			b"Assets",
			b"Account",
			&Blake2_128Concat::hash(&(AssetIds::PKton as u64).encode()),
		)
		.count();

	if let Some(mut asset_details) = take_storage_value::<AssetDetails>(
		b"Assets",
		b"Asset",
		&Blake2_128Concat::hash(&(AssetIds::PKton as u64).encode()),
	) {
		asset_details.accounts = actual_accounts as u32;
		asset_details.sufficients = actual_accounts as u32;

		put_storage_value(
			b"Assets",
			b"Asset",
			&Blake2_128Concat::hash(&(AssetIds::PKton as u64).encode()),
			asset_details,
		);
	}

	// frame_support::weights::Weight::zero()
	RuntimeBlockWeights::get().max_block
}
