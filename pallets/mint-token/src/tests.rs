use crate::{mock::*};
use frame_support::{
	assert_ok,
};


#[test]
fn can_mint() {
	new_test_ext().execute_with(|| {
		assert_ok!(MintSupply::mint(Origin::signed(100), 21000000));
        System::assert_last_event(Event::MintSupply(crate::Event::<Test>::MintedNewSupply(100)));

        assert_ok!(MintSupply::transfer(Origin::signed(100), 101, 2000000));
        System::assert_last_event(Event::MintSupply(crate::Event::<Test>::Transferred(100, 101, 2000000)));

		assert_eq!(MintSupply::get_balance(&100), 19000000);
	});
}