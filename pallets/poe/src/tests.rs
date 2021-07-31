use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

const CLAIM_HASH: &str = "claim hash";
const TEST_SENDER: &str = "Alice";
const TEST_RECEIVER: &str = "Bob";

#[test]
fn create_claim_test() {
	new_test_ext().execute_with(|| {
		let hash = CLAIM_HASH.as_bytes().to_vec();
		let sender = account_key(TEST_SENDER);
		assert_ok!(PoeModule::create_claim(Origin::signed(sender), hash.clone()));
		let (account_id, _) = <Proofs<Test>>::get(&hash);
		assert_eq!(account_id, sender);

		assert!(System::events()
			.iter()
			.any(|er| er.event
				== TestEvent::pallet(crate::Event::ClaimCreated(sender, hash.clone()))));
	});
}

#[test]
fn recreate_claim_test() {
	new_test_ext().execute_with(|| {
		let hash = CLAIM_HASH.as_bytes().to_vec();
		let sender = account_key(TEST_SENDER);
		assert_ok!(PoeModule::create_claim(Origin::signed(sender), hash.clone()));
		assert_noop!(
			PoeModule::create_claim(Origin::signed(sender), hash.clone()),
			Error::<Test>::ProofAlreadyClaimed
		);
	});
}

#[test]
fn revoke_claim_test() {
	new_test_ext().execute_with(|| {
		let hash = CLAIM_HASH.as_bytes().to_vec();
		let invalid_sender = account_key("Bob");
		let sender = account_key(TEST_SENDER);
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(sender), hash.clone()),
			Error::<Test>::NoSuchProof
		);
		assert_ok!(PoeModule::create_claim(Origin::signed(sender), hash.clone()));
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(invalid_sender), hash.clone()),
			Error::<Test>::NotProofOfOwner
		);
		assert_ok!(PoeModule::revoke_claim(Origin::signed(sender), hash.clone()));
	});
}

#[test]
fn transfer_test() {
	new_test_ext().execute_with(|| {
		let hash = CLAIM_HASH.as_bytes().to_vec();
		let sender = account_key(TEST_SENDER);
		let receiver = account_key(TEST_RECEIVER);

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(sender), hash.clone()),
			Error::<Test>::NoSuchProof
		);
		assert_ok!(PoeModule::create_claim(Origin::signed(sender), hash.clone()));
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(receiver), hash.clone()),
			Error::<Test>::NotProofOfOwner
		);

		assert_ok!(PoeModule::transfer(Origin::signed(sender), hash.clone(), receiver));
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(sender), hash.clone()),
			Error::<Test>::NotProofOfOwner
		);
		assert_ok!(PoeModule::revoke_claim(Origin::signed(receiver), hash.clone()));
	});
}
