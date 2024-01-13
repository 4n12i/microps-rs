extern crate microps_rs;

use anyhow::Result;
use data::TEST_DATA;
use microps_rs::driver::dummy::*;
use microps_rs::net::*;
use microps_rs::util::tracing_init;
use std::collections::LinkedList;
use std::thread::sleep;
use std::time::Duration;

mod data;

#[test]
fn step1() -> Result<()> {
    let mut net_devices = LinkedList::new();

    tracing_init();
    net_init()?;
    dummy_init(&mut net_devices)?;
    net_run(&mut net_devices)?;

    let dev = net_devices.back().unwrap();
    loop {
        net_device_output(dev, 0x0800, TEST_DATA, None)?;
        sleep(Duration::from_secs(1));
    }
}
