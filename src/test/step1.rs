use crate::driver::dummy::dummy_init;
use crate::net::net_device_output;
use crate::net::net_init;
use crate::net::net_run;
use crate::net::net_shutdown;
use crate::test::data::TEST_DATA;
use crate::util::tracing_init;
use anyhow::Result;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tracing::warn;

pub fn run() -> Result<()> {
    tracing_init();

    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    warn!("Press Ctrl-C to exit.");

    let mut net_devices = Vec::new();
    net_init()?;
    dummy_init(&mut net_devices)?;
    net_run(&mut net_devices)?;

    let dev = net_devices.pop().unwrap();
    while running.load(Ordering::SeqCst) {
        net_device_output(&dev, 0x0800, TEST_DATA, None)?;
        sleep(Duration::from_secs(1));
    }
    net_shutdown(&mut net_devices)?;

    Ok(())
}
