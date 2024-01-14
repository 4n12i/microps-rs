extern crate microps_rs;

use data::TEST_DATA;
use microps_rs::debug_dump;
use microps_rs::util::tracing_init;
use tracing::debug;

mod data;

#[test]
fn step0() {
    tracing_init();
    debug!("Hello, World!");
    debug_dump!(TEST_DATA);
}
