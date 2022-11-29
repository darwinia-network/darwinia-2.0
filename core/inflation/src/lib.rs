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

//! Darwinia economic inflation mechanism implementation.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

// crates.io
use primitive_types::U256;

// github
use substrate_fixed::{
	transcendental,
	types::{I95F33, U94F34},
};

/// Milliseconds per year for the Julian year (365.25 days).
pub const MILLISECS_PER_YEAR: u128 = (366 * 24 * 60 * 60) * 1000;

/// Compute the inflation of a period.
///
/// Use `U94F34` here, because `2^94 > MAX_RING * 10^9`.
pub fn in_period(unminted: u128, period: u128, living_time: u128) -> Option<u128> {
	let unminted_per_millisecs = U94F34::from_num(unminted) / MILLISECS_PER_YEAR;
	let x = (unminted_per_millisecs * period).floor().to_num();
	let years = (living_time / MILLISECS_PER_YEAR + 1) as _;

	inflate(x, years)
}

// Compute the inflation.
//
// Formula:
// ```
// x * (1 - (99 / 100) ^ sqrt(years));
// ```
//
// Use `I95F33` here, because `2^94 > MAX_RING * 10^9`.
fn inflate(x: u128, years: u8) -> Option<u128> {
	let sqrt = transcendental::sqrt::<I95F33, I95F33>(years.into()).ok()?;
	let ninety_nine = I95F33::from_num(99) / 100;
	let pow = transcendental::pow::<I95F33, I95F33>(ninety_nine, sqrt).ok()?;
	let ratio = I95F33::from_num(1) - pow;
	let inflation = I95F33::from_num(x) * ratio;

	Some(inflation.floor().to_num())
}

/// Compute the reward of a deposit.
///
/// Reference(s):
/// - <https://github.com/evolutionlandorg/bank/blob/master/contracts/GringottsBank.sol#L280>
pub fn deposit_interest(amount: u128, months: u8) -> u128 {
	let amount = U256::from(amount);
	let months = U256::from(months);
	let n = U256::from(67_u8).pow(months);
	let d = U256::from(66_u8).pow(months);
	let quot = n / d;
	let rem = n % d;
	let precision = U256::from(1_000_u16);

	// The result of `((quot - 1) * precision + rem * precision / d)` is `197` when months is `12`.
	//
	// The default interest is `1_000`.
	// So, we directly use `1_970_000` here instead `interest * 197 * 10^7`.
	(amount * (precision * (quot - 1_u8) + precision * rem / d) / 1_970_000_u32).as_u128()
}

#[cfg(test)]
mod test {
	// darwinia
	use crate::*;

	const UNIT: u128 = 1_000_000_000_000_000_000;

	#[test]
	fn inflate_should_work() {
		let max = 10_000_000_000_u128 * UNIT;
		let init = 2_000_000_000_u128 * UNIT;
		let rates = [
			4_f64, 5.37, 6.15, 6.56, 6.74, 6.76, 6.66, 6.5, 6.28, 6.04, 5.79, 5.52, 5.26, 4.99,
			4.74, 4.49, 4.25, 4.03, 3.81, 3.6, 3.4, 3.21, 3.04, 2.87, 2.71, 2.55, 2.41, 2.27, 2.14,
			2.02, 1.91, 1.8, 1.69, 1.59, 1.5, 1.41, 1.33, 1.25, 1.17, 1.1, 1.04, 0.97, 0.91, 0.86,
			0.8, 0.75, 0.71, 0.66, 0.62, 0.58, 0.54, 0.51, 0.47, 0.44, 0.41, 0.38, 0.36, 0.33,
			0.31, 0.29, 0.27, 0.25, 0.23, 0.21, 0.2, 0.18, 0.17, 0.16, 0.15, 0.14, 0.13, 0.12,
			0.11, 0.1, 0.09, 0.08, 0.08, 0.07, 0.07, 0.06, 0.06, 0.05, 0.05, 0.04, 0.04, 0.04,
			0.03, 0.03, 0.03, 0.03, 0.02, 0.02, 0.02, 0.02, 0.02, 0.01, 0.01, 0.01, 0.01, 0.01,
		];
		let mut unminted = max - init;

		for (&rate, years) in rates.iter().zip(1..) {
			let inflation =
				in_period(unminted, MILLISECS_PER_YEAR, (years - 1) as u128 * MILLISECS_PER_YEAR)
					.unwrap();

			sp_arithmetic::assert_eq_error_rate!(
				inflation as f64 / (max - unminted) as f64,
				rate / 100_f64,
				0.0001_f64
			);

			unminted -= inflation;
		}
	}

	#[test]
	fn deposit_interest_should_work() {
		let precision = 10_000_f64;

		for (&expect_interest, months) in [
			0.0761_f64, 0.1522, 0.2335, 0.3096, 0.3959, 0.4771, 0.5634, 0.6446, 0.7309, 0.8223,
			0.9086, 1.0000, 1.0913, 1.1878, 1.2842, 1.3807, 1.4771, 1.5736, 1.6751, 1.7766, 1.8832,
			1.9898, 2.0964, 2.2030, 2.3147, 2.4263, 2.5380, 2.6548, 2.7715, 2.8934, 3.0101, 3.1370,
			3.2588, 3.3857, 3.5126, 3.6446,
		]
		.iter()
		.zip(1_u8..)
		{
			let interest = deposit_interest(10_000_u128 * UNIT, months) as f64 / UNIT as f64;
			let interest = (interest * precision).floor() / precision;

			assert_eq!(interest, expect_interest);
		}
	}
}
