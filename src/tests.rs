use super::*;

#[test]
fn initialize() {
	if init().is_err() {
		panic!();
	}
}
