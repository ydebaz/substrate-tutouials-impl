use super::*;

#[test]
fn error_works(){
    new_test_ext().execute_with(|| {
        assert_err!(
            TestingPallet::(Origin::signed(1), 51),
            "value must be <= maximum add amount constant"
        );
    })
}