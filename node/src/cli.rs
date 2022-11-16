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

// std
use std::path::PathBuf;

/// Sub-commands supported by the collator.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// Remove the whole chain.
	PurgeChain(cumulus_client_cli::PurgeChainCmd),

	/// Export the genesis state of the parachain.
	ExportGenesisState(cumulus_client_cli::ExportGenesisStateCommand),

	/// Export the genesis wasm of the parachain.
	ExportGenesisWasm(cumulus_client_cli::ExportGenesisWasmCommand),

	/// Db meta columns information.
	FrontierDb(fc_cli::FrontierDbCmd),

	/// Sub-commands concerned with benchmarking.
	/// The pallet benchmarking moved to the `pallet` sub-command.
	#[clap(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Try some testing command against a specified runtime state.
	TryRuntime(try_runtime_cli::TryRuntimeCmd),
}

#[derive(Debug, clap::Parser)]
#[clap(
	propagate_version = true,
	args_conflicts_with_subcommands = true,
	subcommand_negates_reqs = true
)]
pub struct Cli {
	#[clap(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[clap(flatten)]
	pub run: cumulus_client_cli::RunCmd,

	/// Disable automatic hardware benchmarks.
	///
	/// By default these benchmarks are automatically ran at startup and measure
	/// the CPU speed, the memory bandwidth and the disk speed.
	///
	/// The results are then printed out in the logs, and also sent as part of
	/// telemetry, if telemetry is enabled.
	#[clap(long)]
	pub no_hardware_benchmarks: bool,

	/// Relay chain arguments
	#[clap(raw = true)]
	pub relay_chain_args: Vec<String>,

	#[clap(flatten)]
	pub eth_args: EthArgs,
}

#[derive(Debug)]
pub struct RelayChainCli {
	/// The actual relay chain cli object.
	pub base: polkadot_cli::RunCmd,

	/// Optional chain id that should be passed to the relay chain.
	pub chain_id: Option<String>,

	/// The base path that should be used by the relay chain.
	pub base_path: Option<PathBuf>,
}
impl RelayChainCli {
	/// Parse the relay chain CLI parameters using the para chain `Configuration`.
	pub fn new<'a>(
		para_config: &sc_service::Configuration,
		relay_chain_args: impl Iterator<Item = &'a String>,
	) -> Self {
		let extension = crate::chain_spec::Extensions::try_get(&*para_config.chain_spec);
		let chain_id = extension.map(|e| e.relay_chain.clone());
		let base_path = para_config.base_path.as_ref().map(|x| x.path().join("polkadot"));
		Self { base_path, chain_id, base: clap::Parser::parse_from(relay_chain_args) }
	}
}

#[derive(Debug, clap::Parser)]
pub struct EthArgs {
	/// Size in bytes of the LRU cache for block data.
	#[clap(long, default_value = "300000000")]
	pub eth_log_block_cache: usize,

	/// Size of the LRU cache for block data and their transaction statuses.
	#[clap(long, default_value = "300000000")]
	pub eth_statuses_cache: usize,

	/// Maximum number of logs in a query.
	#[clap(long, default_value = "10000")]
	pub max_past_logs: u32,

	/// Maximum fee history cache size.
	#[clap(long, default_value = "2048")]
	pub fee_history_limit: u64,

	/// Choose sealing method.
	#[cfg(feature = "manual-seal")]
	#[clap(long, arg_enum, ignore_case = true)]
	pub sealing: Sealing,
}

impl EthArgs {
	pub fn build_eth_rpc_config(&self) -> EthRpcConfig {
		EthRpcConfig {
			eth_statuses_cache: self.eth_statuses_cache,
			eth_log_block_cache: self.eth_log_block_cache,
			max_past_logs: self.max_past_logs,
			fee_history_limit: self.fee_history_limit,
			#[cfg(feature = "manual-seal")]
			sealing: self.sealing,
		}
	}
}

pub struct EthRpcConfig {
	/// Size in bytes of the LRU cache for block data.
	pub eth_log_block_cache: usize,

	/// Size in bytes of the LRU cache for transactions statuses data.
	pub eth_statuses_cache: usize,

	/// Maximum fee history cache size.
	pub fee_history_limit: u64,

	/// Maximum fee history cache size.
	pub max_past_logs: u32,

	/// Choose sealing method.
	#[cfg(feature = "manual-seal")]
	pub sealing: Sealing,
}

/// Available Sealing methods.
#[cfg(feature = "manual-seal")]
#[derive(Debug, Copy, Clone, clap::ArgEnum)]
pub enum Sealing {
	// Seal using rpc method.
	Manual,
	// Seal when transaction is executed.
	Instant,
}

#[cfg(feature = "manual-seal")]
impl Default for Sealing {
	fn default() -> Sealing {
		Sealing::Manual
	}
}
