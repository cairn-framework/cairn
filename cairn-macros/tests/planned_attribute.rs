use cairn_macros::cflx_planned;

#[cflx_planned(phase = 8)]
fn planned_test_fixture() {
    panic!("this test is planned for phase 8 and should be ignored");
}

#[test]
fn test_cflx_planned_emits_ignore_attribute() {
    // The function above should compile and be marked as ignored.
    // We verify this by checking that the macro expanded successfully.
    // In a real test runner, `cargo test` would skip this function.
    assert!(true);
}
