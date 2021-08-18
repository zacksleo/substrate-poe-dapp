use super::*;
use crate::{mock::{new_test_ext, Origin, System, Test, Poe, Event as TestEvent}, Error};
use frame_support::{assert_noop, assert_ok};

const CLAIM_HASH: &str = "claim hash";
const TEST_SENDER: u64 = 1;
const TEST_RECEIVER: u64 = 3;

#[test]
fn create_claim_test() {
	new_test_ext().execute_with(|| {
		let hash = CLAIM_HASH.as_bytes().to_vec();
		let sender = TEST_SENDER;
		assert_ok!(Poe::create_claim(Origin::signed(sender), hash.clone()));

		assert_eq!(Proofs::<Test>::get(&hash), (sender, frame_system::Pallet::<Test>::block_number()));

		// Event is raised
		System::assert_has_event(TestEvent::Poe(Event::ClaimCreated(sender, hash.clone())));
	});
}

#[test]
fn create_claim_test_with_large_hash() {
	new_test_ext().execute_with(|| {
		let hash = "larger claim hash";
		assert_noop!(
			Poe::create_claim(Origin::signed(1), hash.as_bytes().to_vec()),
			Error::<Test>::ProofOutOfMaxSize,
		);
	});
}

#[test]
fn recreate_claim_test() {
	new_test_ext().execute_with(|| {
		let hash = CLAIM_HASH.as_bytes().to_vec();
		let sender = TEST_SENDER;
		assert_ok!(Poe::create_claim(Origin::signed(sender), hash.clone()));
		assert_noop!(
			Poe::create_claim(Origin::signed(sender), hash.clone()),
			Error::<Test>::ProofAlreadyClaimed
		);
	});
}

#[test]
fn revoke_claim_test() {
	new_test_ext().execute_with(|| {
		let hash = CLAIM_HASH.as_bytes().to_vec();
		let invalid_sender = 2;
		let sender = TEST_SENDER;
		assert_noop!(
			Poe::revoke_claim(Origin::signed(sender), hash.clone()),
			Error::<Test>::NoSuchProof
		);
		assert_ok!(Poe::create_claim(Origin::signed(sender), hash.clone()));
		assert_noop!(
			Poe::revoke_claim(Origin::signed(invalid_sender), hash.clone()),
			Error::<Test>::NotProofOfOwner
		);
		assert_ok!(Poe::revoke_claim(Origin::signed(sender), hash.clone()));
		assert_eq!(Proofs::<Test>::contains_key(hash.clone()), false);
		System::assert_has_event(TestEvent::Poe(Event::ClaimRevoked(sender, hash.clone())));
	});
}

#[test]
fn transfer_test() {
	new_test_ext().execute_with(|| {
		let hash = CLAIM_HASH.as_bytes().to_vec();
		let sender = TEST_SENDER;
		let receiver = TEST_RECEIVER;

		assert_noop!(
			Poe::revoke_claim(Origin::signed(sender), hash.clone()),
			Error::<Test>::NoSuchProof
		);
		assert_ok!(Poe::create_claim(Origin::signed(sender), hash.clone()));
		assert_noop!(
			Poe::revoke_claim(Origin::signed(receiver), hash.clone()),
			Error::<Test>::NotProofOfOwner
		);

		assert_ok!(Poe::transfer(Origin::signed(sender), hash.clone(), receiver));
		System::assert_has_event(TestEvent::Poe(Event::ClaimTransfered(sender, receiver.clone(), hash.clone())));

		assert_noop!(
			Poe::revoke_claim(Origin::signed(sender), hash.clone()),
			Error::<Test>::NotProofOfOwner
		);
		assert_ok!(Poe::revoke_claim(Origin::signed(receiver), hash.clone()));
	});
}
