use crate::{ext, mock::*};
use frame_support::{assert_err, assert_ok};
use mocktopus::mocking::*;

#[test]
fn should_not_deposit_against_invalid_vault() {
    run_test(|| {
        assert_err!(
            Nomination::_deposit_collateral(ALICE, BOB, 100),
            TestError::VaultNotOptedInToNomination
        );
    })
}

#[test]
fn should_deposit_against_valid_vault() {
    run_test(|| {
        ext::vault_registry::vault_exists::<Test>.mock_safe(|_| MockResult::Return(true));
        ext::vault_registry::get_backing_collateral::<Test>.mock_safe(|_| MockResult::Return(Ok(10000)));
        ext::vault_registry::compute_collateral::<Test>.mock_safe(|_| MockResult::Return(Ok(10000)));

        assert_ok!(Nomination::_opt_in_to_nomination(&ALICE));
        assert_ok!(Nomination::_deposit_collateral(ALICE, BOB, 100));
    })
}
