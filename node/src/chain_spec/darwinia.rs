// This file is part of Darwinia.
//
// Copyright (C) 2018-2023 Darwinia Network
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
use darwinia_runtime::*;
// frontier
use fp_evm::GenesisAccount;
// substrate
use sc_chain_spec::Properties;
use sc_service::ChainType;
use sp_core::H160;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

fn properties() -> Properties {
	super::properties("RING")
}

// Generate the session keys from individual elements.
//
// The input must be a tuple of individual keys (a single arg for now since we have just one key).
fn session_keys(keys: AuraId) -> SessionKeys {
	SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Darwinia2 Development",
		"darwinia2-development",
		ChainType::Development,
		move || {
			testnet_genesis(
				vec![
					// Bind the `Alice` to `Alith` to make `--alice` available for testnet.
					(
						array_bytes::hex_n_into_unchecked(ALITH),
						get_collator_keys_from_seed("Alice"),
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
		Some(PROTOCOL_ID),
		None,
		Some(properties()),
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 2046,
		},
	)
}

pub fn local_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Darwinia2 Local",
		"darwinia2-local",
		ChainType::Local,
		move || {
			testnet_genesis(
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
		Some(PROTOCOL_ID),
		None,
		Some(properties()),
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 2046,
		},
	)
}

pub fn genesis_config() -> ChainSpec {
	// TODO: update this before the final release
	ChainSpec::from_genesis(
		"Darwinia2",
		"darwinia2",
		ChainType::Live,
		move || {
			GenesisConfig {
				// System stuff.
				system: SystemConfig { code: WASM_BINARY.unwrap().to_vec() },
				parachain_system: Default::default(),
				parachain_info: ParachainInfoConfig { parachain_id: 2046.into() },

				// Monetary stuff.
				balances: BalancesConfig {
					balances: vec![(array_bytes::hex_n_into_unchecked(ALITH), 100_000_000 * UNIT)],
				},
				transaction_payment: Default::default(),
				assets: AssetsConfig {
					assets: vec![(AssetIds::Kton as _, ROOT, true, 1)],
					metadata: vec![(
						AssetIds::Kton as _,
						b"Darwinia Commitment Token".to_vec(),
						b"KTON".to_vec(),
						18,
					)],
					..Default::default()
				},
				vesting: Default::default(),

				// Consensus stuff.
				darwinia_staking: DarwiniaStakingConfig {
					now: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
					elapsed_time: 0,
					collator_count: 3,
					collators: vec![(array_bytes::hex_n_into_unchecked(ALITH), UNIT)],
				},
				session: SessionConfig {
					keys: vec![(
						array_bytes::hex_n_into_unchecked(ALITH),
						array_bytes::hex_n_into_unchecked(ALITH),
						session_keys(get_collator_keys_from_seed("Alice")),
					)],
				},
				aura: Default::default(),
				aura_ext: Default::default(),
				message_gadget: Default::default(),
				ecdsa_authority: Default::default(),

				// Governance stuff.
				democracy: Default::default(),
				council: Default::default(),
				technical_committee: Default::default(),
				phragmen_election: Default::default(),
				technical_membership: Default::default(),
				treasury: Default::default(),

				// Utility stuff.
				sudo: SudoConfig { key: Some(array_bytes::hex_n_into_unchecked(ALITH)) },

				// XCM stuff.
				polkadot_xcm: PolkadotXcmConfig { safe_xcm_version: Some(SAFE_XCM_VERSION) },

				// EVM stuff.
				ethereum: Default::default(),
				evm: EVMConfig {
					accounts: {
						BTreeMap::from_iter(
							DarwiniaPrecompiles::<Runtime>::used_addresses().iter().map(|p| {
								(
									p.to_owned(),
									GenesisAccount {
										nonce: Default::default(),
										balance: Default::default(),
										storage: Default::default(),
										code: REVERT_BYTECODE.to_vec(),
									},
								)
							}),
						)
					},
				},

				// S2S stuff.
				bridge_kusama_grandpa: Default::default(),
				bridge_kusama_parachain: Default::default(),
				bridge_crab_messages: Default::default(),
				crab_fee_market: Default::default(),
			}
		},
		Vec::new(),
		None,
		Some(PROTOCOL_ID),
		None,
		Some(properties()),
		Extensions {
			relay_chain: "polkadot".into(), // You MUST set this to the correct network!
			para_id: 2046,
		},
	)
}

pub fn config() -> ChainSpec {
	unimplemented!("TODO")
}

fn testnet_genesis(
	collators: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> GenesisConfig {
	GenesisConfig {
		// System stuff.
		system: SystemConfig { code: WASM_BINARY.unwrap().to_vec() },
		parachain_system: Default::default(),
		parachain_info: ParachainInfoConfig { parachain_id: id },

		// Monetary stuff.
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 100_000_000 * UNIT)).collect(),
		},
		transaction_payment: Default::default(),
		assets: AssetsConfig {
			assets: vec![(AssetIds::Kton as _, ROOT, true, 1)],
			metadata: vec![(
				AssetIds::Kton as _,
				b"Darwinia Commitment Token".to_vec(),
				b"KTON".to_vec(),
				18,
			)],
			..Default::default()
		},
		vesting: Default::default(),

		// Consensus stuff.
		darwinia_staking: DarwiniaStakingConfig {
			now: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
			elapsed_time: 0,
			collator_count: collators.len() as _,
			collators: collators.iter().map(|(a, _)| (a.to_owned(), UNIT)).collect(),
		},
		session: SessionConfig {
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
		message_gadget: Default::default(),
		ecdsa_authority: Default::default(),

		// Governance stuff.
		democracy: Default::default(),
		council: Default::default(),
		technical_committee: Default::default(),
		phragmen_election: Default::default(),
		technical_membership: Default::default(),
		treasury: Default::default(),

		// Utility stuff.
		sudo: SudoConfig { key: Some(array_bytes::hex_n_into_unchecked(ALITH)) },

		// XCM stuff.
		polkadot_xcm: PolkadotXcmConfig { safe_xcm_version: Some(SAFE_XCM_VERSION) },

		// EVM stuff.
		ethereum: Default::default(),
		evm: EVMConfig {
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

		// S2S stuff.
		bridge_kusama_grandpa: Default::default(),
		bridge_kusama_parachain: Default::default(),
		bridge_crab_messages: Default::default(),
		crab_fee_market: Default::default(),
	}
}
