use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_kitty_works() {
	new_test_ext().execute_with(|| {
        assert_ok!(SubstrateKitties::create_kitty(Origin::signed(1)));
        assert_eq!(SubstrateKitties::kitty_cnt(), 1);
        });
}

#[test]
fn breed_kitty_works() {
	new_test_ext().execute_with(|| {
        let mut sender = 1;
        assert_ok!(SubstrateKitties::create_kitty(Origin::signed(sender)));
        let hash1 = SubstrateKitties::kitties_owned(sender)[0];  
        assert_ok!(SubstrateKitties::create_kitty(Origin::signed(sender)));
        let hash2 = SubstrateKitties::kitties_owned(sender)[1];        

        sender = 3;
        assert_ok!(SubstrateKitties::create_kitty(Origin::signed(sender)));
        let hash3 = SubstrateKitties::kitties_owned(sender)[0];   
        
        assert_noop!(SubstrateKitties::breed_kitty(Origin::signed(1), hash1, hash3), Error::<Test>::NotKittyOwner);
        assert_ok!(SubstrateKitties::breed_kitty(Origin::signed(1), hash1, hash2));
        assert_eq!(SubstrateKitties::kitty_cnt(), 4);

        //test the kitty bred by 1
        let hash = SubstrateKitties::kitties_owned(1)[2]; 
        let kitty = SubstrateKitties::kitties(hash).expect("kitty not exists");
        assert_eq!(kitty.owner, 1);

        });
}

#[test]
fn transfer_kitty_not_owner() {
	new_test_ext().execute_with(|| {
        assert_ok!(SubstrateKitties::create_kitty(Origin::signed(1)));
        let hash = sp_core::H256([1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]);
        assert_noop!(SubstrateKitties::transfer(Origin::signed(1), 2, hash), Error::<Test>::KittyNotExist);
        });
}

#[test]
fn transfer_kitty_works() {
	new_test_ext().execute_with(|| {
        let sender = 1;
        assert_ok!(SubstrateKitties::create_kitty(Origin::signed(sender)));
        assert_eq!(SubstrateKitties::kitty_cnt(), 1);
        let hash = SubstrateKitties::kitties_owned(sender)[0];
        assert_ok!(SubstrateKitties::transfer(Origin::signed(1), 2, hash));
	let kitty = SubstrateKitties::kitties(hash).expect("kitty not exists");
        assert_eq!(kitty.owner, 2);
        });
}

#[test]
fn set_price_works() {
	new_test_ext().execute_with(|| {
        let sender = 1;
        assert_ok!(SubstrateKitties::create_kitty(Origin::signed(sender)));
        let hash = SubstrateKitties::kitties_owned(sender)[0];
        assert_ok!(SubstrateKitties::set_price(Origin::signed(sender), hash, Some(3)));
        let kitty = SubstrateKitties::kitties(hash).expect("kitty not exists");
        assert_eq!(kitty.price, Some(3));
	});
}


#[test]
fn buy_kitty_works() {
	new_test_ext().execute_with(|| {
        let mut sender = 1;
        assert_ok!(SubstrateKitties::create_kitty(Origin::signed(sender)));
        let hash1 = SubstrateKitties::kitties_owned(sender)[0];        
        assert_ok!(SubstrateKitties::set_price(Origin::signed(sender), hash1, Some(3)));

        sender = 2;
        assert_ok!(SubstrateKitties::create_kitty(Origin::signed(sender)));
        let hash2 = SubstrateKitties::kitties_owned(sender)[0];

        let kitty1 = SubstrateKitties::kitties(hash1).expect("kitty not exists");
        assert_eq!(kitty1.price, Some(3));

        assert_noop!(SubstrateKitties::buy_kitty(Origin::signed(5), hash1, 2), Error::<Test>::KittyBidPriceTooLow);

        assert_noop!(SubstrateKitties::buy_kitty(Origin::signed(5), hash2, 2), Error::<Test>::KittyNotForSale);

        assert_noop!(SubstrateKitties::buy_kitty(Origin::signed(5), hash1, 100), Error::<Test>::NotEnoughBalance);

        assert_noop!(SubstrateKitties::buy_kitty(Origin::signed(1), hash1, 2), Error::<Test>::BuyerIsKittyOwner);

        assert_ok!(SubstrateKitties::buy_kitty(Origin::signed(5), hash1, 3));

        let kitty = SubstrateKitties::kitties(hash1).expect("kitty not exists");
        assert_eq!(kitty.owner, 5);

        });
}

