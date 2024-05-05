extern crate learn_rust;

mod common;

#[test]
#[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
fn panic_when_guess_is_greater_than_100() {
    common::setup();
    learn_rust::Guess::new(200);
}
