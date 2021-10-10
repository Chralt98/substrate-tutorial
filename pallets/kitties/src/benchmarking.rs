use super::*;

use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create {
		let caller = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	breed {
		let caller = whitelisted_caller();

		let mut kitty = Kitty(Default::default());
		let kitty_id = orml_nft::Pallet::<T>::mint(&caller, Pallet::<T>::class_id(), Vec::new(), kitty.clone())?;

		kitty.0[0] = 1;
		let kitty_id2 = orml_nft::Pallet::<T>::mint(&caller, Pallet::<T>::class_id(), Vec::new(), kitty)?;

	}: _(RawOrigin::Signed(caller), kitty_id, kitty_id2)

	transfer {
		let caller = whitelisted_caller();
		let to = account("to", 0, 0);

		let kitty_id = orml_nft::Pallet::<T>::mint(&caller, Pallet::<T>::class_id(), Vec::new(), Kitty(Default::default()))?;

	}: _(RawOrigin::Signed(caller), to, kitty_id)

	set_price {
		let caller = whitelisted_caller();

		let kitty_id = orml_nft::Pallet::<T>::mint(&caller, Pallet::<T>::class_id(), Vec::new(), Kitty(Default::default()))?;

	}: _(RawOrigin::Signed(caller), kitty_id, Some(100u32.into()))

	buy {
		let caller = whitelisted_caller();
		let seller = account("seller", 0, 0);

		let _ = T::Currency::make_free_balance_be(&caller, 1000u32.into());

		let kitty_id = orml_nft::Pallet::<T>::mint(&seller, Pallet::<T>::class_id(), Vec::new(), Kitty(Default::default()))?;
		Pallet::<T>::set_price(RawOrigin::Signed(seller.clone()).into(), kitty_id, Some(500u32.into()))?;

	}: _(RawOrigin::Signed(caller), seller, kitty_id, 500u32.into())
}

impl_benchmark_test_suite!(Pallet, crate::tests::new_test_ext(), crate::tests::Test,);
