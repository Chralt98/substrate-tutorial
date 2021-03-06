use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

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

		assert_noop!(
			RewardCoin::transfer(Origin::signed(2), 3, 50),
			Error::<Test>::InsufficientBalance
		);

		// when 2 would send 41 coins to 4 then 2 would have not at least 10 coins minimum balance anymore
		assert_noop!(
			RewardCoin::transfer(Origin::signed(2), 4, 41),
			Error::<Test>::BelowMinBalance
		);

		// when 2 would send 1 coin to 4 then 4 would not have at least 10 coins minimum balance
		assert_noop!(
			RewardCoin::transfer(Origin::signed(2), 4, 1),
			Error::<Test>::BelowMinBalance
		);

		// 3 would have more than the MinBalance of 10 (and the 2 would have 15 to send, but more then MinBalance after the transfer), so it is successful 
		assert_ok!(RewardCoin::transfer(Origin::signed(2), 3, 15));
	});
}

#[test]
fn transfer_works_atomic() {
	new_test_ext().execute_with(|| {
		MetaDataStore::<Test>::put(MetaData {
			issuance: 0,
			minter: 1,
			burner: 1,
		});
		// Dispatch a signed extrinsic.
		assert_ok!(RewardCoin::mint(Origin::signed(1), 2, 42));
		System::assert_last_event(Event::RewardCoin(crate::Event::<Test>::Minted(
			2, 42,
		)));

		assert_eq!(RewardCoin::account(&2), 42);

		// this would fail for 3 because 3 hasn't got the MinBalance of 10
		// the sender would have lost the 9 tokens
		assert_noop!(RewardCoin::transfer(Origin::signed(2), 3, 9), Error::<Test>::BelowMinBalance);

		// Account 2 got still a Balance of 42, so the storage seems reverted
		// it is because the #[transactional] annotation 
		assert_eq!(RewardCoin::account(&2), 42);
	});
}
