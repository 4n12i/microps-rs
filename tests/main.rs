extern crate rsps;

use rsps::util::*;

mod data;

#[test]
fn step0() {
    debugf("Hello, World!");
    debugdump(data::TEST_DATA);
}
