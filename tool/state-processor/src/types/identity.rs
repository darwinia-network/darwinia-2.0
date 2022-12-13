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

// crates.io
use parity_scale_codec::{Decode, Encode};

pub type RegistrarIndex = u32;
pub type Balance = u128;

#[derive(Clone, Encode, Decode, Debug)]
pub struct Registration {
	pub judgements: Vec<(RegistrarIndex, Judgement)>,
	pub deposit: Balance,
	pub info: IdentityInfo,
}

#[derive(Clone, Encode, Decode, Debug)]
pub enum Judgement {
	Unknown,
	FeePaid(Balance),
	Reasonable,
	KnownGood,
	OutOfDate,
	LowQuality,
	Erroneous,
}

#[derive(Clone, Encode, Decode, Debug)]
pub enum Data {
	None,
	Raw(Vec<u8>),
	BlakeTwo256([u8; 32]),
	Sha256([u8; 32]),
	Keccak256([u8; 32]),
	ShaThree256([u8; 32]),
}

#[derive(Clone, Encode, Decode, Debug)]
pub struct IdentityInfo {
	pub additional: Vec<(Data, Data)>,
	pub display: Data,
	pub legal: Data,
	pub web: Data,
	pub riot: Data,
	pub email: Data,
	pub pgp_fingerprint: Option<[u8; 20]>,
	pub image: Data,
	pub twitter: Data,
}
