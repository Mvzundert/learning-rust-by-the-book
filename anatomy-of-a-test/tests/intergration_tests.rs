use anatomy_of_a_test;

mod common;

#[test]
fn it_works() {
    common::setup();
    assert_eq!(anatomy_of_a_test::add(2, 2), 4);
}

