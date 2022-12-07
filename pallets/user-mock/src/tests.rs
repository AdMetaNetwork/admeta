use crate::{mock::*, Error};
use admeta_common::TargetTag;
use frame_support::{assert_noop, assert_ok};

#[test]
fn add_profile_succeeds() {
	new_test_ext().execute_with(|| {
		assert_ok!(User::add_profile(RuntimeOrigin::signed(1), 20, TargetTag::DeFi, true));
		assert_eq!(User::get_user(1).is_some(), true);
	});
}
