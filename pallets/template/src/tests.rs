use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::store_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		print!("1");
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
		print!("2");

	});
}

#[test]
fn sum_is_transactional() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::store_something(Origin::signed(1), u32::MAX));
		// Ensure the expected error is thrown when no value is present.
		print!("3");

		assert_noop!(TemplateModule::transactional_sum(Origin::signed(1), 1), Error::<Test>::Overflow);
	});
}