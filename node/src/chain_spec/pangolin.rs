// This file is part of Darwinia.
//
// Copyright (C) 2018-2022 Darwinia Network
// SPDX-License-Identifier: GPL-3.0
//
// Darwinia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Darwinia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

#![allow(clippy::derive_partial_eq_without_eq)]

// std
use std::{collections::BTreeMap, str::FromStr};
// crates.io
use serde::{Deserialize, Serialize};
// cumulus
use cumulus_primitives_core::ParaId;
// darwinia
use super::*;
use dc_primitives::*;
use pangolin_runtime::{AuraId, EvmConfig, PangolinPrecompiles, Runtime};
// frontier
use fp_evm::GenesisAccount;
// substrate
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public, H160, U256};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<pangolin_runtime::GenesisConfig, Extensions>;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn session_keys(keys: AuraId) -> pangolin_runtime::SessionKeys {
	pangolin_runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "PRING".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 18.into());

	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 1000,
		},
	)
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "RING".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 18.into());

	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				1000.into(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("pangolin"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 1000,
		},
	)
}

pub fn shell_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "RING".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 18.into());

	ChainSpec::from_genesis(
		// Name
		"Pangolin",
		// ID
		"pangolin",
		ChainType::Live,
		move || {
			pangolin_runtime::GenesisConfig {
				system: pangolin_runtime::SystemConfig {
					code: pangolin_runtime::WASM_BINARY
						.expect("WASM binary was not build, please build it!")
						.to_vec(),
				},
				balances: Default::default(),
				parachain_info: pangolin_runtime::ParachainInfoConfig { parachain_id: 2046.into() },
				collator_selection: pangolin_runtime::CollatorSelectionConfig {
					invulnerables: vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
					..Default::default()
				},
				session: pangolin_runtime::SessionConfig {
					keys: vec![(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						session_keys(get_collator_keys_from_seed("Alice")),
					)],
				},
				// no need to pass anything to aura, in fact it will panic if we do. Session will
				// take care of this.
				aura: Default::default(),
				aura_ext: Default::default(),
				parachain_system: Default::default(),
				polkadot_xcm: pangolin_runtime::PolkadotXcmConfig {
					safe_xcm_version: Some(SAFE_XCM_VERSION),
				},
				ethereum: Default::default(),
				evm: Default::default(),
				base_fee: Default::default(),
			}
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("pangolin"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "polkadot".into(), // You MUST set this to the correct network!
			para_id: 2046,
		},
	)
}

fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> pangolin_runtime::GenesisConfig {
	pangolin_runtime::GenesisConfig {
		system: pangolin_runtime::SystemConfig {
			code: pangolin_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: pangolin_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		parachain_info: pangolin_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: pangolin_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: UNIT,
			..Default::default()
		},
		session: pangolin_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),        // account id
						acc,                // validator id
						session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: pangolin_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
		ethereum: Default::default(),
		evm: EvmConfig {
			accounts: {
				let mut map = BTreeMap::new();
				map.insert(
					// Testing account.
					H160::from_str("0x6be02d1d3665660d22ff9624b7be0551ee1ac91b")
						.expect("internal `H160` is valid; qed"),
					GenesisAccount {
						balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
							.expect("internal `U256` is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// Benchmarking account.
					H160::from_str("1000000000000000000000000000000000000001")
						.expect("internal `H160` is valid; qed"),
					GenesisAccount {
						nonce: U256::from(1),
						balance: U256::from(1_000_000_000_000_000_000_000_000_u128),
						storage: Default::default(),
						code: vec![0x00],
					},
				);

				for precompile in PangolinPrecompiles::<Runtime>::used_addresses() {
					map.insert(
						precompile,
						GenesisAccount {
							nonce: Default::default(),
							balance: Default::default(),
							storage: Default::default(),
							code: REVERT_BYTECODE.to_vec(),
						},
					);
				}
				map
			},
		},
		base_fee: Default::default(),
	}
}
