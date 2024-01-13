extern crate microps_rs;

use data::TEST_DATA;
use microps_rs::util::*;
use pretty_hex::PrettyHex;
use tracing::debug;

mod data;

#[test]
fn step0() {
    tracing_init();
    debug!("Hello, World!");
    debug!("{:?}", TEST_DATA.hex_dump());
}
