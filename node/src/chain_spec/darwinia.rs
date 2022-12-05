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
use std::{
	collections::BTreeMap,
	str::FromStr,
	time::{SystemTime, UNIX_EPOCH},
};
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
		"Darwinia2 Development",
		// ID
		"darwinia-dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// Initial collators.
				vec![
					// Bind the `Alice` to `Alith` to make `--alice` available for testnet.
					(
						array_bytes::hex_n_into_unchecked(ALITH),
						get_collator_keys_from_seed("Alice"),
					),
					// Bind the `Bob` to `Balthar` to make `--bob` available for testnet.
					(
						array_bytes::hex_n_into_unchecked(BALTATHAR),
						get_collator_keys_from_seed("Bob"),
					),
					// Bind the `Charlie` to `CHARLETH` to make `--charlie` available for testnet.
					(
						array_bytes::hex_n_into_unchecked(CHARLETH),
						get_collator_keys_from_seed("Charlie"),
					),
				],
				vec![
					array_bytes::hex_n_into_unchecked(ALITH),
					array_bytes::hex_n_into_unchecked(BALTATHAR),
					array_bytes::hex_n_into_unchecked(CHARLETH),
					array_bytes::hex_n_into_unchecked(DOROTHY),
					array_bytes::hex_n_into_unchecked(ETHAN),
					array_bytes::hex_n_into_unchecked(FAITH),
				],
				2046.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 2046,
		},
	)
}
fn testnet_genesis(
	collators: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> darwinia_runtime::GenesisConfig {
	darwinia_runtime::GenesisConfig {
		// System stuff.
		system: darwinia_runtime::SystemConfig {
			code: darwinia_runtime::WASM_BINARY.unwrap().to_vec(),
		},
		parachain_system: Default::default(),
		parachain_info: darwinia_runtime::ParachainInfoConfig { parachain_id: id },

		// Monetary stuff.
		balances: darwinia_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 100_000_000 * UNIT)).collect(),
		},
		transaction_payment: Default::default(),
		assets: Default::default(),

		// Consensus stuff.
		staking: darwinia_runtime::StakingConfig {
			now: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
			elapsed_time: 0,
			collator_count: collators.len() as _,
			collators: collators.iter().map(|(a, _)| (a.to_owned(), UNIT)).collect(),
		},
		session: darwinia_runtime::SessionConfig {
			keys: collators
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
		aura: Default::default(),
		aura_ext: Default::default(),

		// Governance stuff.
		democracy: Default::default(),
		council: Default::default(),
		technical_committee: Default::default(),
		phragmen_election: Default::default(),
		technical_membership: Default::default(),
		treasury: Default::default(),

		// Utility stuff.
		sudo: Default::default(),
		vesting: Default::default(),

		// XCM stuff.
		polkadot_xcm: darwinia_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},

		// EVM stuff.
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

		// S2S stuff
		bridge_crab_grandpa: Default::default(),
		bridge_crab_messages: Default::default(),
		crab_fee_market: Default::default(),
	}
}

pub fn genesis_config() -> ChainSpec {
	unimplemented!("TODO")
}

pub fn config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "RING".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 18.into());

	// TODO: update this before final release
	ChainSpec::from_genesis(
		// Name
		"Darwinia2",
		// ID
		"darwinia",
		ChainType::Live,
		move || {
			darwinia_runtime::GenesisConfig {
				// System stuff.
				system: darwinia_runtime::SystemConfig {
					code: darwinia_runtime::WASM_BINARY
						.expect("WASM binary was not build, please build it!")
						.to_vec(),
				},
				parachain_system: Default::default(),
				parachain_info: darwinia_runtime::ParachainInfoConfig { parachain_id: 2046.into() },

				// Monetary stuff.
				balances: Default::default(),
				transaction_payment: Default::default(),
				assets: Default::default(),

				// Consensus stuff.
				staking: darwinia_runtime::StakingConfig {
					now: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
					elapsed_time: 0,
					collator_count: 3,
					collators: Vec::new(),
				},
				session: darwinia_runtime::SessionConfig {
					keys: vec![(
						array_bytes::hex_n_into_unchecked(ALITH),
						array_bytes::hex_n_into_unchecked(ALITH),
						session_keys(get_collator_keys_from_seed("Alice")),
					)],
				},
				aura: Default::default(),
				aura_ext: Default::default(),

				// Governance stuff.
				democracy: Default::default(),
				council: Default::default(),
				technical_committee: Default::default(),
				phragmen_election: Default::default(),
				technical_membership: Default::default(),
				treasury: Default::default(),

				// Utility stuff.
				sudo: Default::default(),
				vesting: Default::default(),

				// XCM stuff.
				polkadot_xcm: darwinia_runtime::PolkadotXcmConfig {
					safe_xcm_version: Some(SAFE_XCM_VERSION),
				},

				// EVM stuff.
				ethereum: Default::default(),
				evm: Default::default(),
				base_fee: Default::default(),

				// S2S stuff
				bridge_crab_grandpa: Default::default(),
				bridge_crab_messages: Default::default(),
				crab_fee_market: Default::default(),
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
