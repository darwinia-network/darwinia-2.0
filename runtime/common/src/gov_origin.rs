// paritytech
use frame_support::traits::EitherOfDiverse;
use frame_system::EnsureRoot;
use pallet_collective::{EnsureProportionAtLeast, EnsureProportionMoreThan};
// darwinia
use dc_primitives::AccountId;

pub type Root = EnsureRoot<AccountId>;

pub type RootOrAtLeastHalf<Collective> =
	EitherOfDiverse<Root, EnsureProportionAtLeast<AccountId, Collective, 1, 2>>;

pub type RootOrMoreThanHalf<Collective> =
	EitherOfDiverse<Root, EnsureProportionMoreThan<AccountId, Collective, 1, 2>>;

pub type RootOrAtLeastTwoThird<Collective> =
	EitherOfDiverse<Root, EnsureProportionAtLeast<AccountId, Collective, 2, 3>>;

pub type RootOrAtLeastThreeFifth<Collective> =
	EitherOfDiverse<Root, EnsureProportionAtLeast<AccountId, Collective, 3, 5>>;

pub type RootOrAll<Collective> =
	EitherOfDiverse<Root, EnsureProportionAtLeast<AccountId, Collective, 1, 1>>;
