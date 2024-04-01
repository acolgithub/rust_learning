use test_organization;  // needed to bring library into this test crate's scope

mod common;  // include module corresponding to mod.rs

#[test]  // do not need #[cfg(test)] configuration annotation
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organization::add_two(2));
}


