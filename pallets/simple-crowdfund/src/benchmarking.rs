//! Benchmarking setup for simple_crowdfund

use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller, account, impl_benchmark_test_suite};
#[allow(unused)]
use crate::Pallet as SimpleCrowdfund;

benchmarks! {
	test_create_crowdfunds {
		let caller: T::AccountId = whitelisted_caller();
		let beneficiary: T::AccountId = account("bob", 0, 0);
		let goal: u32 = 0;
		let s in 0 .. 100;
	}: create(RawOrigin::Signed(caller), beneficiary, BalanceOf::<T>::from(goal), T::BlockNumber::from(s))
	verify {
		assert_eq!(<FundCount<T>>::get(), s);
	}
}

impl_benchmark_test_suite!(
	Pallet,
	crate::mock::new_test_ext(),
	crate::mock::Test
);
