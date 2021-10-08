use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

use crate::{MetaData, MetaDataStore};

#[test]
fn minting_works() {
	new_test_ext().execute_with(|| {
		MetaDataStore::<Test>::put(MetaData {
			issuance: 0,
			minter: 1,
			burner: 1,
		});
		// Dispatch a signed extrinsic.
		assert_ok!(RewardCoin::mint(Origin::signed(1), 2, 42));
	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		MetaDataStore::<Test>::put(MetaData {
			issuance: 0,
			minter: 1,
			burner: 1,
		});
		// Dispatch a signed extrinsic.
		assert_ok!(RewardCoin::mint(Origin::signed(1), 2, 42));

		assert_noop!(RewardCoin::transfer(Origin::signed(2), 3, 50), Error::<Test>::InsufficientBalance);

		// when 2 would send 41 coins to 4 then 2 would have not at least 10 coins minimum balance anymore
		assert_noop!(RewardCoin::transfer(Origin::signed(2), 4, 41), Error::<Test>::BelowMinBalance);

		// when 2 would send 1 coin to 4 then 4 would not have at least 10 coins minimum balance
		assert_noop!(RewardCoin::transfer(Origin::signed(2), 4, 1), Error::<Test>::BelowMinBalance);

		assert_ok!(RewardCoin::transfer(Origin::signed(2), 3, 15));
	});
}