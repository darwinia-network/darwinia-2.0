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

mod balances;
mod frontier;
mod system;

mod type_registry;
use type_registry::*;

// std
use std::{
	env,
	fs::File,
	io::{Read, Write},
};
// crates.io
use anyhow::Result;
use fxhash::FxHashMap;
use parity_scale_codec::{Decode, Encode};
use serde::de::DeserializeOwned;
// hack-ink
use subspector::ChainSpec;

type Map<V> = FxHashMap<String, V>;

fn main() -> Result<()> {
	env::set_var("RUST_LOG", "state_processor");
	pretty_env_logger::init();

	Processor::new()?.process()?;

	Ok(())
}

struct Processor {
	solo_state: State,
	para_state: State,
	shell_chain_spec: ChainSpec,
}
impl Processor {
	fn new() -> Result<Self> {
		Ok(Self {
			solo_state: State::from_file("test-data/pangolin.json")?,
			para_state: State::from_file("test-data/darwinia-parachain.json")?,
			shell_chain_spec: from_file("test-data/shell.json")?,
		})
	}

	fn process(mut self) -> Result<()> {
		log::info!("bear: --- state process system.");
		self.process_system();
		log::info!("bear: --- state frontier's storage.");
		self.process_ethereum();

		self.save()
	}

	fn save(self) -> Result<()> {
		log::info!("save processed chain spec");

		let mut f = File::create("test-data/processed.json")?;
		let v = serde_json::to_vec(&self.shell_chain_spec)?;

		f.write_all(&v)?;

		Ok(())
	}
}

struct State(Map<String>);
impl State {
	fn from_file(path: &str) -> Result<Self> {
		Ok(Self(from_file::<ChainSpec>(path)?.genesis.raw.top))
	}

	#[allow(unused)]
	fn prune(&mut self, pallet: &[u8], items: Option<&[&[u8]]>) -> &mut Self {
		// Prune specific storages.
		if let Some(items) = items {
			for item in items {
				let k = item_key(pallet, item);

				self.0.remove(&k).or_else(|| {
					log::warn!(
						"`{}::{}: {k}` not found",
						String::from_utf8_lossy(pallet),
						String::from_utf8_lossy(item)
					);

					None
				});
			}
		}
		// Prune entire pallet.
		else {
			let prefix = pallet_key(pallet);
			let mut pruned = false;

			self.0.retain(|full_key, _| {
				if full_key.starts_with(&prefix) {
					pruned = true;

					false
				} else {
					true
				}
			});

			if !pruned {
				log::warn!("`{}: {prefix}` not found", String::from_utf8_lossy(pallet));
			}
		}

		self
	}

	fn take<D, F>(
		&mut self,
		pallet: &[u8],
		item: &[u8],
		buffer: &mut Map<D>,
		preprocess_key: F,
	) -> &mut Self
	where
		D: Decode,
		F: Fn(&str, &str) -> String,
	{
		let item_key = item_key(pallet, item);

		self.0.retain(|full_key, v| {
			if full_key.starts_with(&item_key) {
				match decode(v) {
					Ok(v) => {
						buffer.insert(preprocess_key(full_key, &item_key), v);
					},
					Err(e) => log::warn!("failed to decode `{full_key}:{v}`, due to `{e}`"),
				}

				false
			} else {
				true
			}
		});

		self
	}
}

fn from_file<D>(path: &str) -> Result<D>
where
	D: DeserializeOwned,
{
	log::info!("load data from {path:?}");

	let mut f = File::open(path)?;
	let mut v = Vec::new();

	f.read_to_end(&mut v)?;

	Ok(serde_json::from_slice(&v)?)
}

fn pallet_key(pallet: &[u8]) -> String {
	let prefix = subhasher::twox128(pallet);

	array_bytes::bytes2hex("0x", &prefix)
}

fn item_key(pallet: &[u8], item: &[u8]) -> String {
	let k = substorager::storage_key(pallet, item);

	array_bytes::bytes2hex("0x", &k.0)
}

fn encode_value<V>(v: V) -> String
where
	V: Encode,
{
	array_bytes::bytes2hex("0x", &v.encode())
}

fn decode<D>(hex: &str) -> Result<D>
where
	D: Decode,
{
	let v = array_bytes::hex2bytes(hex).map_err(|e| anyhow::anyhow!("{e:?}"))?;

	Ok(D::decode(&mut &*v)?)
}

// twox128(pallet) + twox128(item) + blake2_256_concat(item_key) -> blake2_256_concat(item_key)
fn get_blake2_128_concat_suffix(full_key: &str, item_key: &str) -> String {
	full_key.trim_start_matches(item_key).into()
}

// twox128(pallet) + twox128(item) + blake2_256_concat(account_id_32) -> account_id_32
#[allow(unused)]
fn get_concat_suffix(full_key: &str, _: &str) -> String {
	format!("0x{}", &full_key[full_key.len() - 64..])
}
