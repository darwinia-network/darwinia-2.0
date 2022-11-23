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
// cumulus
use cumulus_primitives_core::ParaId;
// darwinia
use super::*;
use darwinia_runtime::{AuraId, DarwiniaPrecompiles, EvmConfig, Runtime};
use dc_primitives::*;
// frontier
use fp_evm::GenesisAccount;
// substrate
use sc_service::ChainType;
use sp_core::H160;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<darwinia_runtime::GenesisConfig, Extensions>;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn session_keys(keys: AuraId) -> darwinia_runtime::SessionKeys {
	darwinia_runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "RING".into());
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
						array_bytes::hex_n_into_unchecked(COLLATOR_A),
						// Make `--alice` available for testnet.
						get_collator_keys_from_seed("Alice"),
					),
					(
						array_bytes::hex_n_into_unchecked(COLLATOR_B),
						// Make `--bob` available for testnet.
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![
					array_bytes::hex_n_into_unchecked(COLLATOR_A),
					array_bytes::hex_n_into_unchecked(COLLATOR_B),
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
		"Darwinia Local Testnet",
		// ID
		"darwinia_local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// initial collators.
				vec![
					(
						array_bytes::hex_n_into_unchecked(COLLATOR_A),
						// Make `--alice` available for testnet.
						get_collator_keys_from_seed("Alice"),
					),
					(
						array_bytes::hex_n_into_unchecked(COLLATOR_B),
						// Make `--bob` available for testnet.
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![
					array_bytes::hex_n_into_unchecked(COLLATOR_A),
					array_bytes::hex_n_into_unchecked(COLLATOR_B),
				],
				1000.into(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("darwinia"),
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

pub fn config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "RING".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 18.into());

	ChainSpec::from_genesis(
		// Name
		"Darwinia",
		// ID
		"darwinia",
		ChainType::Live,
		move || {
			darwinia_runtime::GenesisConfig {
				system: darwinia_runtime::SystemConfig {
					code: darwinia_runtime::WASM_BINARY
						.expect("WASM binary was not build, please build it!")
						.to_vec(),
				},
				balances: Default::default(),
				parachain_info: darwinia_runtime::ParachainInfoConfig { parachain_id: 2046.into() },
				// TODO: update this before final release
				collator_selection: darwinia_runtime::CollatorSelectionConfig {
					invulnerables: vec![array_bytes::hex_n_into_unchecked(COLLATOR_A)],
					..Default::default()
				},
				// TODO: update this before final release
				session: darwinia_runtime::SessionConfig {
					keys: vec![(
						array_bytes::hex_n_into_unchecked(COLLATOR_A),
						array_bytes::hex_n_into_unchecked(COLLATOR_A),
						session_keys(get_collator_keys_from_seed("Alice")),
					)],
				},
				// no need to pass anything to aura, in fact it will panic if we do. Session will
				// take care of this.
				aura: Default::default(),
				aura_ext: Default::default(),
				parachain_system: Default::default(),
				polkadot_xcm: darwinia_runtime::PolkadotXcmConfig {
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
		Some("darwinia"),
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
) -> darwinia_runtime::GenesisConfig {
	darwinia_runtime::GenesisConfig {
		system: darwinia_runtime::SystemConfig {
			code: darwinia_runtime::WASM_BINARY.unwrap().to_vec(),
		},
		balances: darwinia_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 100_000_000 * UNIT)).collect(),
		},
		parachain_info: darwinia_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: darwinia_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: UNIT,
			..Default::default()
		},
		session: darwinia_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc,                // account id
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
		polkadot_xcm: darwinia_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
		ethereum: Default::default(),
		evm: EvmConfig {
			accounts: {
				BTreeMap::from_iter(
					DarwiniaPrecompiles::<Runtime>::used_addresses()
						.iter()
						.map(|p| {
							(
								p.to_owned(),
								GenesisAccount {
									nonce: Default::default(),
									balance: Default::default(),
									storage: Default::default(),
									code: REVERT_BYTECODE.to_vec(),
								},
							)
						})
						.chain([
							// Testing account.
							(
								H160::from_str("0x6be02d1d3665660d22ff9624b7be0551ee1ac91b")
									.unwrap(),
								GenesisAccount {
									balance: (10_000_000 * UNIT).into(),
									code: Default::default(),
									nonce: Default::default(),
									storage: Default::default(),
								},
							),
							// Benchmarking account.
							(
								H160::from_str("1000000000000000000000000000000000000001").unwrap(),
								GenesisAccount {
									nonce: 1.into(),
									balance: (10_000_000 * UNIT).into(),
									storage: Default::default(),
									code: vec![0x00],
								},
							),
						]),
				)
			},
		},
		base_fee: Default::default(),
	}
}

pub fn genesis_config() -> ChainSpec {
	unimplemented!("TODO")
}
