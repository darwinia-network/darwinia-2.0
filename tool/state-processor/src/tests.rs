// crates.io
use array_bytes::hex_n_into_unchecked;
use parity_scale_codec::Encode;
use primitive_types::H256;
// darwinia
use crate::*;

struct Tester {
	// solo chain
	solo_accounts: Map<AccountInfo>,
	solo_remaining_ring: Map<u128>,
	solo_remaining_kton: Map<u128>,
	solo_evm_codes: Map<Vec<u8>>,
	// para chain
	para_accounts: Map<AccountInfo>,
	// processed
	migration_accounts: Map<AccountInfo>,
	migration_kton_accounts: Map<AssetAccount>,
	shell_system_accounts: Map<AccountInfo>,
	shell_evm_codes: Map<Vec<u8>>,

	solo_state: State,
	para_state: State,
	shell_state: State,
}

fn get_last_64(key: &str, _: &str) -> String {
	format!("0x{}", &key[key.len() - 64..])
}

fn get_last_40(key: &str, _: &str) -> String {
	format!("0x{}", &key[key.len() - 40..])
}

fn twox64_concat_to_string<D>(data: D) -> String
where
	D: AsRef<[u8]>,
{
	array_bytes::bytes2hex("", subhasher::twox64_concat(data))
}

impl Tester {
	fn new() -> Self {
		// This test is only used to ensure the correctness of  the state processor and is only
		// applicable to Crab, Crab Parachain.
		let mut solo_state = State::from_file("test-data/crab.json").unwrap();
		let mut para_state = State::from_file("test-data/crab-parachain.json").unwrap();
		let mut shell_state = State::from_file("test-data/processed.json").unwrap();

		// solo chain
		let mut solo_accounts = <Map<AccountInfo>>::default();
		let mut solo_remaining_ring = <Map<u128>>::default();
		let mut solo_remaining_kton = <Map<u128>>::default();
		let mut solo_evm_codes = <Map<Vec<u8>>>::default();
		solo_state
			.take_map(b"System", b"Account", &mut solo_accounts, get_last_64)
			.take_map(b"Ethereum", b"RemainingRingBalance", &mut solo_remaining_ring, get_last_64)
			.take_map(b"Ethereum", b"RemainingKtonBalance", &mut solo_remaining_kton, get_last_64)
			.take_map(b"EVM", b"AccountCodes", &mut solo_evm_codes, get_last_40);

		// para chain
		let mut para_accounts = <Map<AccountInfo>>::default();
		para_state.take_map(b"System", b"Account", &mut para_accounts, get_last_64);

		// processed
		let mut shell_system_accounts = <Map<AccountInfo>>::default();
		let mut shell_evm_codes = <Map<Vec<u8>>>::default();
		let mut migration_accounts = <Map<AccountInfo>>::default();
		let mut migration_kton_accounts = <Map<AssetAccount>>::default();
		shell_state
			.take_map(b"System", b"Account", &mut shell_system_accounts, get_last_40)
			.take_map(
				b"AccountMigration",
				b"KtonAccounts",
				&mut migration_kton_accounts,
				get_last_64,
			)
			.take_map(b"AccountMigration", b"Accounts", &mut migration_accounts, get_last_64)
			.take_map(b"Evm", b"AccountCodes", &mut shell_evm_codes, get_last_40);

		Self {
			solo_accounts,
			solo_remaining_ring,
			solo_remaining_kton,
			solo_evm_codes,

			para_accounts,

			migration_accounts,
			migration_kton_accounts,
			shell_system_accounts,
			shell_evm_codes,

			solo_state,
			para_state,
			shell_state,
		}
	}
}

fn run_test<T>(test: T)
where
	T: FnOnce(&Tester) + std::panic::UnwindSafe,
{
	let tester = Tester::new();
	let result = std::panic::catch_unwind(|| test(&tester));
	assert!(result.is_ok())
}

// --- System & Balances & Assets ---

#[test]
fn solo_chain_substrate_account_adjust() {
	run_test(|tester| {
		let test_addr = "0xf4171e1b64c96cc17f601f28d002cb5fcd27eab8b6585e296f4652be5bf05550";

		let solo_account = tester.solo_accounts.get(test_addr).unwrap();
		assert_ne!(solo_account.nonce, 0);
		assert_ne!(solo_account.consumers, 0);
		assert_ne!(solo_account.providers, 0);
		assert_eq!(solo_account.sufficients, 0);
		assert_ne!(solo_account.data.free, 0);
		assert_ne!(solo_account.data.free_kton_or_misc_frozen, 0);

		// after migrate

		let migrated_account = tester.migration_accounts.get(test_addr).unwrap();
		assert_eq!(solo_account.consumers, migrated_account.consumers);
		assert_eq!(solo_account.providers, migrated_account.providers);
		assert_eq!(solo_account.sufficients + 1, migrated_account.sufficients);
		// nonce reset
		assert_eq!(migrated_account.nonce, 0);
		// decimal adjust
		assert_eq!(solo_account.data.free * GWEI, migrated_account.data.free);
		// the kton part has been removed.
		assert_eq!(migrated_account.data.free_kton_or_misc_frozen, 0);
		//  the kton part moved to the asset pallet
		let asset_account = tester.migration_kton_accounts.get(test_addr).unwrap();
		assert_eq!(asset_account.balance, solo_account.data.free_kton_or_misc_frozen * GWEI);
		assert!(!asset_account.is_frozen);
	});
}

#[test]
fn solo_chain_substrate_account_adjust_with_remaining_balance() {
	run_test(|tester| {
		let test_addr = "0xfe129f56cc498227acacc4231f70ae15a2f4e8f9ccfa51f4de268c75516fa350";

		let solo_account = tester.solo_accounts.get(test_addr).unwrap();
		let remaining_balance = tester.solo_remaining_ring.get(test_addr).unwrap();
		assert_ne!(*remaining_balance, 0);

		// after migrate

		let migrated_account = tester.migration_accounts.get(test_addr).unwrap();
		assert_eq!(migrated_account.data.free, solo_account.data.free * GWEI + remaining_balance);
	});
}

#[test]
fn combine_solo_account_with_para_account() {
	run_test(|tester| {
		let test_addr = "0x2a997fbf3423723ab73fae76567b320de6979664cb3287c0e6ce24099d0eff68";

		// solo
		let solo_account = tester.solo_accounts.get(test_addr).unwrap();
		let remaining_balance = tester.solo_remaining_ring.get(test_addr).unwrap();
		assert_ne!(solo_account.nonce, 0);
		// para
		let para_account = tester.para_accounts.get(test_addr).unwrap();
		assert_ne!(para_account.nonce, 0);

		// after migrate

		let migrated_account = tester.migration_accounts.get(test_addr).unwrap();
		assert_eq!(
			migrated_account.data.free,
			solo_account.data.free * GWEI + remaining_balance + para_account.data.free
		);
		// reset the nonce
		assert_eq!(migrated_account.nonce, 0);
	});
}

#[test]
fn evm_account_adjust() {
	run_test(|tester| {
		let test_addr = "0x64766d3a00000000000000aef71b03670f1c52cd3d8efc2ced3ad68ad91e33f3";

		let solo_account = tester.solo_accounts.get(test_addr).unwrap();
		assert_ne!(solo_account.nonce, 0);
		assert_ne!(solo_account.data.free, 0);
		assert_ne!(solo_account.data.free_kton_or_misc_frozen, 0);
		let solo_remaining_kton = tester.solo_remaining_kton.get(test_addr).unwrap();
		assert_ne!(*solo_remaining_kton, 0);

		// after migrate

		let migrate_addr = "0xaef71b03670f1c52cd3d8efc2ced3ad68ad91e33";
		let migrated_account = tester.shell_system_accounts.get(migrate_addr).unwrap();
		// nonce doesn't changed.
		assert_eq!(migrated_account.nonce, solo_account.nonce);
		assert_eq!(migrated_account.consumers, solo_account.consumers);
		assert_eq!(migrated_account.providers, solo_account.providers);
		// sufficient increase by one because of the asset pallet.
		assert_eq!(migrated_account.sufficients, solo_account.sufficients + 1);
		assert_eq!(migrated_account.data.free, solo_account.data.free * GWEI);
		assert_eq!(migrated_account.data.free_kton_or_misc_frozen, 0);

		//  the kton part moved to the asset pallet
		let mut asset_account = AssetAccount::default();
		let migrate_addr: [u8; 20] =
			hex_n_into_unchecked::<_, _, 20>("0xaef71b03670f1c52cd3d8efc2ced3ad68ad91e33");
		tester.shell_state.get_value(
			b"Assets",
			b"Account",
			&format!(
				"{}{}",
				blake2_128_concat_to_string(KTON_ID.encode()),
				blake2_128_concat_to_string(migrate_addr.encode()),
			),
			&mut asset_account,
		);
		assert_eq!(
			asset_account.balance,
			solo_account.data.free_kton_or_misc_frozen * GWEI + solo_remaining_kton
		);
		assert!(!asset_account.is_frozen);
	});
}

#[test]
fn evm_contract_account_adjust_sufficients() {
	run_test(|tester| {
		let test_addr = "0x64766d3a000000000000000050f880c35c31c13bfd9cbb7d28aafaeca3abd2d0";
		let solo_account = tester.solo_accounts.get(test_addr).unwrap();
		assert_eq!(solo_account.sufficients, 0);

		// after migrated

		let migrate_addr = "0x0050f880c35c31c13bfd9cbb7d28aafaeca3abd2";
		let migrated_account = tester.shell_system_accounts.get(migrate_addr).unwrap();
		assert_eq!(migrated_account.sufficients, 1);
	});
}

#[test]
fn ring_total_issuance() {
	run_test(|tester| {
		let mut solo_issuance = u128::default();
		let mut para_issuance = u128::default();

		tester.solo_state.get_value(b"Balances", b"TotalIssuance", "", &mut solo_issuance);
		assert_ne!(solo_issuance, 0);
		tester.para_state.get_value(b"Balances", b"TotalIssuance", "", &mut para_issuance);
		assert_ne!(para_issuance, 0);

		// after migrate
		let mut migrated_total_issuance = u128::default();
		tester.shell_state.get_value(
			b"Balances",
			b"TotalIssuance",
			"",
			&mut migrated_total_issuance,
		);

		assert!(migrated_total_issuance - (solo_issuance * GWEI + para_issuance) < 200 * GWEI);
	});
}

#[test]
fn kton_total_issuance() {
	run_test(|tester| {
		let mut total_issuance = u128::default();
		tester.solo_state.get_value(b"Kton", b"TotalIssuance", "", &mut total_issuance);
		assert_ne!(total_issuance, 0);

		// after migrate
		let mut migrated_total_issuance = u128::default();
		tester.shell_state.get_value(
			b"Balances",
			b"TotalIssuance",
			"",
			&mut migrated_total_issuance,
		);

		let mut details = AssetDetails::default();
		tester.shell_state.get_value(
			b"Assets",
			b"Asset",
			&blake2_128_concat_to_string(KTON_ID.encode()),
			&mut details,
		);
		assert!(details.supply - total_issuance * GWEI < 200 * GWEI);
	});
}

#[test]
fn asset_creation() {
	run_test(|tester| {
		let mut details = AssetDetails::default();
		tester.shell_state.get_value(
			b"Assets",
			b"Asset",
			&blake2_128_concat_to_string(KTON_ID.encode()),
			&mut details,
		);
		assert!(details.accounts > 0);
		assert!(details.supply != 0);
		assert_eq!(details.min_balance, 1);
		assert_eq!(details.sufficients, details.accounts);
	});
}

#[test]
fn asset_metadata() {
	run_test(|tester| {
		let mut metadata = AssetMetadata::default();
		tester.shell_state.get_value(
			b"Assets",
			b"Metadata",
			&blake2_128_concat_to_string(KTON_ID.encode()),
			&mut metadata,
		);
		assert_eq!(metadata.decimals, 18);
		assert_eq!(metadata.symbol, b"KTON".to_vec());
		assert_eq!(metadata.name, b"Darwinia Commitment Token".to_vec());
	});
}

// --- EVM & Ethereum ---

#[test]
fn evm_code_migrate() {
	run_test(|tester| {
		{
			let test_addr = "0x0050f880c35c31c13bfd9cbb7d28aafaeca3abd2";

			let code = tester.solo_evm_codes.get(test_addr).unwrap();
			assert_ne!(code.len(), 0);

			// after migrate

			let migrated_code = tester.shell_evm_codes.get(test_addr).unwrap();
			assert_eq!(*code, *migrated_code);
		}

		{
			assert_eq!(tester.solo_evm_codes, tester.shell_evm_codes);
		}
	});
}

#[test]
fn evm_account_storage_migrate() {
	run_test(|tester| {
		// https://crab.subscan.io/account/0x0050f880c35c31c13bfd9cbb7d28aafaeca3abd2
		let test_addr: [u8; 20] =
			hex_n_into_unchecked::<_, _, 20>("0x0050f880c35c31c13bfd9cbb7d28aafaeca3abd2");

		let storage_item_len = tester.solo_state.0.iter().fold(0u32, |sum, (k, _)| {
			if k.starts_with(&full_key(
				b"EVM",
				b"AccountStorages",
				&blake2_128_concat_to_string(test_addr.encode()),
			)) {
				sum + 1
			} else {
				sum
			}
		});
		assert_ne!(storage_item_len, 0);

		let storage_key: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0x2093bcd1218dc1519493ee712ddfee3f4ced2d74096331d39d4247147baf17e2",
		);
		let mut storage_value = H256::zero();
		tester.solo_state.get_value(
			b"EVM",
			b"AccountStorages",
			&format!(
				"{}{}",
				&blake2_128_concat_to_string(test_addr.encode()),
				&blake2_128_concat_to_string(storage_key),
			),
			&mut storage_value,
		);
		assert_ne!(storage_value, H256::zero());

		// after migrate
		let migrated_storage_item_len = tester.shell_state.0.iter().fold(0u32, |sum, (k, _)| {
			if k.starts_with(&full_key(
				b"Evm",
				b"AccountStorages",
				&blake2_128_concat_to_string(test_addr.encode()),
			)) {
				sum + 1
			} else {
				sum
			}
		});
		assert_eq!(storage_item_len, migrated_storage_item_len);

		let mut migrated_storage_value = H256::zero();
		tester.shell_state.get_value(
			b"Evm",
			b"AccountStorages",
			&format!(
				"{}{}",
				&blake2_128_concat_to_string(test_addr.encode()),
				&blake2_128_concat_to_string(storage_key),
			),
			&mut migrated_storage_value,
		);
		assert_eq!(storage_value, migrated_storage_value);
	});
}

// --- Staking ---

#[test]
fn bonded_migrate() {
	run_test(|tester| {
		// https://crab.subscan.io/account/5FxS8ugbXi4WijFuNS45Wg3Z5QsdN8hLZMmo71afoW8hJP67
		let test_addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0xac288b0d41a3dcb69b025f51d9ad76ee088339f1c27708e164f9b019c584897d",
		);

		let mut controller = [0u8; 32];
		tester.solo_state.get_value(
			b"Staking",
			b"Bonded",
			&twox64_concat_to_string(test_addr.encode()),
			&mut controller,
		);
		assert_ne!(controller, [0u8; 32]);

		// after migrate
		let mut migrated_controller = [0u8; 32];
		tester.shell_state.get_value(
			b"AccountMigration",
			b"Bonded",
			&twox64_concat_to_string(test_addr.encode()),
			&mut migrated_controller,
		);
		assert_eq!(migrated_controller, controller);
	});
}

#[test]
fn deposit_items_migrate() {
	run_test(|tester| {
		// https://crab.subscan.io/account/5Dfh9agy74KFmdYqxNGEWae9fE9pdzYnyCUJKqK47Ac64zqM
		let test_addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0x46eb701bdc7f74ffda9c4335d82b3ae8d4e52c5ac630e50d68ab99822e29b3f6",
		);

		let mut ledger = StakingLedger::default();
		tester.solo_state.get_value(
			b"Staking",
			b"Ledger",
			&blake2_128_concat_to_string(test_addr.encode()),
			&mut ledger,
		);
		assert_ne!(ledger.deposit_items.len(), 0);
		let deposits_sum: u128 = ledger.deposit_items.iter().map(|i| i.value).sum();

		// after migrate
		let mut migrated_deposits = Vec::<Deposit>::new();
		tester.shell_state.get_value(
			b"AccountMigration",
			b"Deposits",
			&blake2_128_concat_to_string(test_addr.encode()),
			&mut migrated_deposits,
		);
		assert_eq!(migrated_deposits.len(), ledger.deposit_items.len());
		ledger.deposit_items.iter().zip(migrated_deposits.iter()).for_each(|(old, new)| {
			assert_eq!(new.value, old.value * GWEI);
			assert_eq!(new.expired_time, old.expire_time as u128);
			assert!(new.in_use);
		});
		let migrated_deposits_sum: u128 = migrated_deposits.iter().map(|i| i.value).sum();
		assert_eq!(migrated_deposits_sum, deposits_sum * GWEI);
	});
}

#[test]
fn ledgers_staked_value_migrate() {
	run_test(|tester| {
		// https://crab.subscan.io/account/5Dfh9agy74KFmdYqxNGEWae9fE9pdzYnyCUJKqK47Ac64zqM
		let test_addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0x46eb701bdc7f74ffda9c4335d82b3ae8d4e52c5ac630e50d68ab99822e29b3f6",
		);

		let mut ledger = StakingLedger::default();
		tester.solo_state.get_value(
			b"Staking",
			b"Ledger",
			&blake2_128_concat_to_string(test_addr.encode()),
			&mut ledger,
		);
		assert_ne!(ledger.active, 0);
		assert_ne!(ledger.active_kton, 0);

		// after migrate
		let mut migrated_ledger = Ledger::default();
		tester.shell_state.get_value(
			b"AccountMigration",
			b"Ledgers",
			&blake2_128_concat_to_string(test_addr.encode()),
			&mut migrated_ledger,
		);
		assert_eq!(migrated_ledger.staked_ring, ledger.active * GWEI);
		assert_eq!(migrated_ledger.staked_kton, ledger.active_kton * GWEI);
	});
}

#[test]
fn ledgers_unbondings_migrate() {
	run_test(|tester| {
		// https://crab.subscan.io/account/5FGL7pMZFZK4zWX2y3CRABeqMpMjBq77LhfYipWoBAT9gJsa
		let test_addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0x8d92774046fd3dc60d41825023506ad5ad91bd0d66e9c1df325fc3cf89c2d317",
		);

		let mut ledger = StakingLedger::default();
		tester.solo_state.get_value(
			b"Staking",
			b"Ledger",
			&blake2_128_concat_to_string(test_addr.encode()),
			&mut ledger,
		);
		assert_ne!(ledger.ring_staking_lock.unbondings.len(), 0);

		// after migrate
		let mut migrated_ledger = Ledger::default();
		tester.shell_state.get_value(
			b"AccountMigration",
			b"Ledgers",
			&blake2_128_concat_to_string(test_addr.encode()),
			&mut migrated_ledger,
		);
		ledger
			.ring_staking_lock
			.unbondings
			.iter()
			.zip(migrated_ledger.unstaking_ring.iter())
			.for_each(|(old, (amount, util))| {
				assert_eq!(*amount, old.amount * GWEI);
				assert!(*util < old.until);
			});
	});
}

#[test]
fn ring_pool_migrate() {
	run_test(|tester| {
		let mut ring_pool = u128::default();
		tester.solo_state.get_value(b"Staking", b"RingPool", "", &mut ring_pool);
		assert_ne!(ring_pool, 0);

		// after migrate
		let mut migrated_ring_pool = u128::default();
		tester.shell_state.get_value(b"Staking", b"RingPool", "", &mut migrated_ring_pool);
		assert_eq!(migrated_ring_pool, ring_pool * GWEI);
	});
}

#[test]
fn kton_pool_migrate() {
	run_test(|tester| {
		let mut kton_pool = u128::default();
		tester.solo_state.get_value(b"Staking", b"KtonPool", "", &mut kton_pool);
		assert_ne!(kton_pool, 0);

		// after migrate
		let mut migrated_kton_pool = u128::default();
		tester.shell_state.get_value(b"Staking", b"KtonPool", "", &mut migrated_kton_pool);
		assert_eq!(migrated_kton_pool, kton_pool * GWEI);
	});
}

#[test]
fn elapsed_time_migrate() {
	run_test(|tester| {
		let mut elapsed_time = u64::default();
		tester.solo_state.get_value(b"Staking", b"LivingTime", "", &mut elapsed_time);
		assert_ne!(elapsed_time, 0);

		// after migrate
		let mut migrated_elapsed_time = u128::default();
		tester.shell_state.get_value(b"Staking", b"ElapsedTime", "", &mut migrated_elapsed_time);
		assert_eq!(migrated_elapsed_time, elapsed_time as u128);
	});
}

// --- Vesting ---
#[test]
fn vesting_info_adjust() {
	run_test(|tester| {
		// https://crab.subscan.io/account/5EFJA3K6uRfkLxqjhHyrkJoQjfhmhyVyVEG5XtPPBM6yCCxM
		let test_addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0x608c62275934b164899ca6270c4b89c5d84b2390d4316fda980cd1b3acfad525",
		);

		let mut vesting_info = VestingInfo::default();
		tester.solo_state.get_value(
			b"Vesting",
			b"Vesting",
			&blake2_128_concat_to_string(test_addr.encode()),
			&mut vesting_info,
		);
		assert_ne!(vesting_info.locked, 0);
		assert_ne!(vesting_info.starting_block, 0);

		// after migrate
		let mut migrated_vesting_info = VestingInfo::default();
		tester.shell_state.get_value(
			b"AccountMigration",
			b"Vestings",
			&blake2_128_concat_to_string(test_addr.encode()),
			&mut migrated_vesting_info,
		);

		assert_eq!(migrated_vesting_info.locked, vesting_info.locked * GWEI);
		assert_eq!(migrated_vesting_info.per_block, vesting_info.per_block * GWEI * 2);
		assert!(migrated_vesting_info.starting_block < vesting_info.starting_block);
	});
}

// --- Indices ---

#[test]
#[ignore]
fn indices_adjust_evm_account() {
	run_test(|tester| {
		let test_addr = "0x64766d3a00000000000000c7912465c55be41bd09325b393f4fbea73f26d473b";

		let solo_account = tester.solo_accounts.get(test_addr).unwrap();
		let remaining_balance = tester.solo_remaining_ring.get(test_addr).unwrap();
		assert_ne!(solo_account.data.reserved, 0);

		// after migrated

		let migrated_account = "0xc7912465c55be41bd09325b393f4fbea73f26d47";
		let migrated_account = tester.shell_system_accounts.get(migrated_account).unwrap();

		// TODO: https://github.com/darwinia-network/darwinia-2.0/issues/166
		assert_eq!(
			migrated_account.data.free,
			(solo_account.data.free + solo_account.data.reserved) * GWEI + remaining_balance
		);
		assert_eq!(migrated_account.data.reserved, 0);
	});
}

#[test]
#[ignore]
fn indices_adjust_substrate_account() {
	run_test(|tester| {
		let test_addr = "0xf83ee607164969887eaecab7e058ab3ba0f64c0cfe3f0b575fe45562cfc36bd5";

		let solo_account = tester.solo_accounts.get(test_addr).unwrap();
		assert_ne!(solo_account.data.reserved, 0);

		// after migrated
		let migrated_account = tester.migration_accounts.get(test_addr).unwrap();

		// TODO: https://github.com/darwinia-network/darwinia-2.0/issues/166
		assert_eq!(
			migrated_account.data.free,
			(solo_account.data.free + solo_account.data.reserved) * GWEI
		);
		assert_eq!(migrated_account.data.reserved, 0);
	});
}