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

// darwinia
use crate::*;

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	codec::Encode,
	codec::Decode,
	codec::MaxEncodedLen,
	scale_info::TypeInfo,
	sp_runtime::RuntimeDebug,
)]
pub enum ProxyType {
	Any,
	NonTransfer,
	Governance,
	Staking,
	IdentityJudgement,
	CancelProxy,
	EcdsaBridge,
	SubstrateBridge,
}
impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}
impl frame_support::traits::InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => !matches!(
				c,
				RuntimeCall::Balances(..)
					| RuntimeCall::Assets(..)
					| RuntimeCall::Vesting(pallet_vesting::Call::vested_transfer { .. })
					| RuntimeCall::Deposit(..)
					| RuntimeCall::DarwiniaStaking(..)
					// Might contains transfer {
					| RuntimeCall::Utility(..)
					| RuntimeCall::Proxy(..)
					| RuntimeCall::PolkadotXcm(..)
					| RuntimeCall::Ethereum(..) // }
			),
			ProxyType::Governance => matches!(
				c,
				RuntimeCall::Democracy(..)
					| RuntimeCall::Council(..)
					| RuntimeCall::TechnicalCommittee(..)
					| RuntimeCall::PhragmenElection(..)
					| RuntimeCall::Treasury(..)
					| RuntimeCall::Tips(..)
			),
			ProxyType::Staking => {
				matches!(
					c,
					RuntimeCall::Session(..)
						| RuntimeCall::Deposit(..)
						| RuntimeCall::DarwiniaStaking(..)
				)
			},
			ProxyType::IdentityJudgement =>
				matches!(c, RuntimeCall::Identity(pallet_identity::Call::provide_judgement { .. })),
			ProxyType::CancelProxy => {
				matches!(c, RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. }))
			},
			ProxyType::EcdsaBridge => {
				matches!(c, RuntimeCall::EcdsaAuthority(..))
			},
			ProxyType::SubstrateBridge => {
				matches!(
					c,
					RuntimeCall::BridgePolkadotGrandpa(..)
						| RuntimeCall::BridgePolkadotParachain(..)
						| RuntimeCall::BridgeDarwiniaMessages(..)
						| RuntimeCall::BridgeDarwiniaDispatch(..)
						| RuntimeCall::DarwiniaFeeMarket(..)
				)
			},
		}
	}

	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			(ProxyType::NonTransfer, _) => true,
			_ => false,
		}
	}
}

impl pallet_proxy::Config for Runtime {
	type AnnouncementDepositBase = ConstU128<{ darwinia_deposit(1, 8) }>;
	type AnnouncementDepositFactor = ConstU128<{ darwinia_deposit(0, 66) }>;
	type CallHasher = Hashing;
	type Currency = Balances;
	type MaxPending = ConstU32<32>;
	type MaxProxies = ConstU32<32>;
	// One storage item; key size 32, value size 8; .
	type ProxyDepositBase = ConstU128<{ darwinia_deposit(1, 8) }>;
	// Additional storage item size of 33 bytes.
	type ProxyDepositFactor = ConstU128<{ darwinia_deposit(0, 33) }>;
	type ProxyType = ProxyType;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_proxy::WeightInfo<Self>;
}
