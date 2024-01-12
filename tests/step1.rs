extern crate rsps;

use anyhow::bail;
use anyhow::Result;
use rsps::driver::dummy::*;
use rsps::net::*;
use rsps::util::*;
use std::collections::LinkedList;

mod data;

#[test]
fn main() -> Result<()> {
    let mut net_devices = LinkedList::new();

    if net_init().is_err() {
        errorf("net_init() failure");
        bail!("Error")
    }
    if dummy_init(&mut net_devices).is_err() {
        errorf("dummy_init() failure");
        bail!("Error")
    }
    if net_run(&mut net_devices).is_err() {
        errorf("net_run() failure");
        bail!("Error")
    }
    Ok(())
}
