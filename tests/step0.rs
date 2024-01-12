extern crate rsps;

use rsps::util::*;

mod data;

#[test]
fn main() {
    debugf("Hello, World!");
    debugdump(data::TEST_DATA);
}
