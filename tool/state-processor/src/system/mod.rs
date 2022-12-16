// darwinia
use crate::*;
// parity
use array_bytes::bytes2hex;
use sp_core::{H160, U256};

#[derive(Debug)]
pub struct AccountAll {
	pub key: String,
	pub nonce: u32,
	pub consumers: u32,
	pub providers: u32,
	pub sufficients: u32,
	pub ring: u128,
	pub ring_reserved: u128,
	pub ring_locks: Vec<BalanceLock>,
	pub kton: u128,
	pub kton_reserved: u128,
	pub kton_locks: Vec<BalanceLock>,
}

impl Processor {
	// System storage items.
	// https://github.com/paritytech/substrate/blob/polkadot-v0.9.16/frame/system/src/lib.rs#L545-L639
	// Balances storage items.
	// https://github.com/paritytech/substrate/blob/polkadot-v0.9.16/frame/balances/src/lib.rs#L486-L535
	pub fn process_system(&mut self) -> &mut Self {
		let solo_account_infos = self.process_solo_account_infos();
		let para_account_infos = self.process_para_account_infos();
		let (ring_total_issuance_storage, kton_total_issuance_storage) = self.process_balances();
		let mut accounts = Map::default();
		let mut ring_total_issuance = u128::default();
		let mut kton_total_issuance = u128::default();

		log::info!("build accounts");
		log::info!("calculate total issuance");
		solo_account_infos.into_iter().for_each(|(k, v)| {
			accounts.insert(
				k.clone(),
				AccountAll {
					key: k,
					nonce: v.nonce,
					// ---
					// TODO: check if we could ignore para's.
					consumers: v.consumers,
					providers: v.providers,
					sufficients: v.sufficients,
					// ---
					ring: v.data.free,
					ring_reserved: v.data.reserved,
					ring_locks: Default::default(),
					kton: v.data.free_kton_or_misc_frozen,
					kton_reserved: v.data.reserved_kton_or_fee_frozen,
					kton_locks: Default::default(),
				},
			);

			ring_total_issuance += v.data.free;
			ring_total_issuance += v.data.reserved;
			kton_total_issuance += v.data.free_kton_or_misc_frozen;
			kton_total_issuance += v.data.reserved_kton_or_fee_frozen;
		});
		para_account_infos.into_iter().for_each(|(k, v)| {
			accounts
				.entry(k.clone())
				.and_modify(|a| {
					a.nonce = v.nonce.max(a.nonce);
					a.ring += v.data.free;
					a.ring_reserved += v.data.reserved;
				})
				.or_insert(AccountAll {
					key: k,
					nonce: v.nonce,
					consumers: v.consumers,
					providers: v.providers,
					sufficients: v.sufficients,
					ring: v.data.free,
					ring_reserved: v.data.reserved,
					ring_locks: Default::default(),
					kton: 0,
					kton_reserved: 0,
					kton_locks: Default::default(),
				});

			ring_total_issuance += v.data.free;
			ring_total_issuance += v.data.reserved;
		});

		log::info!("set `Balances::TotalIssuance`");
		log::info!("ring_total_issuance({ring_total_issuance})");
		log::info!("ring_total_issuance_storage({ring_total_issuance_storage})");
		self.shell_state
			.0
			.insert(item_key(b"Balances", b"TotalIssuance"), encode_value(ring_total_issuance));

		let mut kton_details = AssetDetails {
			owner: H160::from_low_u64_be(999),   // TODO: update this
			issuer: H160::from_low_u64_be(999),  // TODO: update this
			admin: H160::from_low_u64_be(999),   // TODO: update this
			freezer: H160::from_low_u64_be(999), // TODO: update this
			supply: kton_total_issuance,
			deposit: 0,
			min_balance: 0,
			is_sufficient: true,
			sufficients: 0,
			accounts: 0,
			approvals: 0,
			is_frozen: false,
		};

		log::info!("update ring misc frozen and fee frozen");
		log::info!("set `System::Account`");
		log::info!("set `Balances::Locks`");
		accounts.into_iter().for_each(|(k, v)| {
			let mut a = AccountInfo {
				nonce: v.nonce,
				consumers: v.consumers,
				providers: v.providers,
				sufficients: v.sufficients,
				data: AccountData {
					free: v.ring,
					reserved: v.ring_reserved,
					free_kton_or_misc_frozen: Default::default(),
					reserved_kton_or_fee_frozen: Default::default(),
				},
			};

			if is_evm_address(&k) {
				log::info!("set `Assets::Account` and `Assets::Approvals`");
				if v.kton != 0 || v.kton_reserved != 0 {
					let aa = AssetAccount {
						balance: v.kton,
						is_frozen: false,
						reason: ExistenceReason::Sufficient,
						extra: (),
					};

					a.sufficients += 1;
					kton_details.accounts += 1;
					kton_details.sufficients += 1;
					// Note: this is double map structure in the pallet-assets.
					self.shell_state.0.insert(
						full_key(
							b"Assets",
							b"Account",
							&format!(
								"{}{}",
								bytes2hex("", subhasher::blake2_128_concat(&1026u64.encode())),
								&k
							),
						),
						encode_value(&aa),
					);

					let mut approves = Map::<U256>::default();
					self.solo_state.take_map(
						b"KtonERC20",
						b"Approves",
						&mut approves,
						get_hashed_key,
					);
					approves.iter().for_each(|(k, v)| {
						self.shell_state.0.insert(
							full_key(b"Assets", b"Approvals", &k),
							encode_value(Approval { amount: v.as_u128(), deposit: 0 }.encode()),
						);
					});
				}

				self.shell_state.0.insert(full_key(b"System", b"Account", &k), encode_value(a));
			} else {
				a.nonce = 0;

				if v.kton != 0 || v.kton_reserved != 0 {
					let aa = AssetAccount {
						balance: v.kton,
						is_frozen: false,
						reason: ExistenceReason::Sufficient,
						extra: (),
					};

					self.shell_state.0.insert(
						full_key(b"AccountMigration", b"KtonAccounts", &k),
						encode_value(&aa),
					);
				}

				self.shell_state
					.0
					.insert(full_key(b"AccountMigration", b"Accounts", &k), encode_value(a));
			}
		});

		log::info!("set `Assets::Asset`");
		log::info!("kton_total_issuance({kton_total_issuance})");
		log::info!("kton_total_issuance_storage({kton_total_issuance_storage})");
		self.shell_state.0.insert(
			item_key(b"Assets", b"Asset"),
			encode_value(
				AssetDetails {
					owner: H160::from_low_u64_be(999),   // TODO: update this
					issuer: H160::from_low_u64_be(999),  // TODO: update this
					admin: H160::from_low_u64_be(999),   // TODO: update this
					freezer: H160::from_low_u64_be(999), // TODO: update this
					supply: kton_total_issuance,
					deposit: 0,
					min_balance: 0,
					is_sufficient: true,
					sufficients: 0,
					accounts: 0,
					approvals: 0,
					is_frozen: false,
				}
				.encode(),
			),
		);

		self
	}

	fn process_solo_account_infos(&mut self) -> Map<AccountInfo> {
		let mut account_infos = <Map<AccountInfo>>::default();
		let mut remaining_ring = <Map<u128>>::default();
		let mut remaining_kton = <Map<u128>>::default();

		log::info!("take solo `System::Account`, `Ethereum::RemainingRingBalance` and `Ethereum::RemainingKtonBalance`");
		self.solo_state
			.take_map(b"System", b"Account", &mut account_infos, get_hashed_key)
			.take_map(b"Ethereum", b"RemainingRingBalance", &mut remaining_ring, get_hashed_key)
			.take_map(b"Ethereum", b"RemainingKtonBalance", &mut remaining_kton, get_hashed_key);

		log::info!("adjust solo balance decimals");
		account_infos.iter_mut().for_each(|(_, v)| {
			v.data.free *= GWEI;
			v.data.reserved *= GWEI;
			v.data.free_kton_or_misc_frozen *= GWEI;
			v.data.reserved_kton_or_fee_frozen *= GWEI;
		});

		log::info!("merge solo remaining balances");
		remaining_ring.into_iter().for_each(|(k, v)| {
			if let Some(a) = account_infos.get_mut(&k) {
				a.data.free += v;
			} else {
				log::error!(
					"`Account({})` not found while merging `RemainingRingBalance`",
					get_last_64(&k)
				);
			}
		});
		remaining_kton.into_iter().for_each(|(k, v)| {
			if let Some(a) = account_infos.get_mut(&k) {
				a.data.free_kton_or_misc_frozen += v;
			} else {
				log::error!(
					"`Account({})` not found while merging `RemainingKtonBalance`",
					get_last_64(&k)
				);
			}
		});

		account_infos
	}

	fn process_para_account_infos(&mut self) -> Map<AccountInfo> {
		let mut account_infos = <Map<AccountInfo>>::default();

		log::info!("take para `System::Account`");
		self.para_state.take_map(b"System", b"Account", &mut account_infos, get_hashed_key);

		account_infos
	}
}

fn is_evm_address(address: &str) -> bool {
	let address = array_bytes::hex2bytes_unchecked(address);

	address.starts_with(b"dvm:")
		&& address[1..31].iter().fold(address[0], |checksum, &byte| checksum ^ byte) == address[31]
}

#[test]
fn verify_evm_address_checksum_should_work() {
	// subalfred key 5ELRpquT7C3mWtjerpPfdmaGoSh12BL2gFCv2WczEcv6E1zL
	// sub-seed
	// public-key 0x64766d3a00000000000000b7de7f8c52ac75e036d05fda53a75cf12714a76973
	// Substrate 5ELRpquT7C3mWtjerpPfdmaGoSh12BL2gFCv2WczEcv6E1zL
	assert!(is_evm_address("0x64766d3a00000000000000b7de7f8c52ac75e036d05fda53a75cf12714a76973"));
}

#[test]
fn test_hash() {
	assert_eq!(
		array_bytes::bytes2hex("", subhasher::blake2_128_concat(&1026u64.encode())),
		"15ffd708b25d8ed5477f01d3f9277c360204000000000000"
	);
}
