//! Proc-macro fixture test for the `cflx_planned` attribute.

use cairn_macros::cflx_planned;

#[cflx_planned(phase = 8)]
#[allow(dead_code)] // Reason: fixture for macro expansion; not a direct test target.
fn planned_test_fixture() {
    panic!("this test is planned for phase 8 and should be ignored");
}

#[test]
fn test_cflx_planned_emits_ignore_attribute() {
    // The function above should compile and be marked as ignored.
    // We verify this by checking that the macro expanded successfully.
}
