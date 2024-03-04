use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;
use tracing::warn;

use crate::driver::dummy::dummy_init;
use crate::net::net_device_output;
use crate::net::net_init;
use crate::net::net_run;
use crate::net::net_shutdown;
use crate::steps::data::TEST_DATA;

pub fn run() -> Result<()> {
    // Flag to detect Ctrl-C
    let running = Arc::new(AtomicBool::new(true));

    // Handler setting
    let r = Arc::clone(&running);
    let mut signals = Signals::new([SIGINT]).expect("failed to create signal handler");
    thread::spawn(move || {
        for _ in signals.forever() {
            r.store(false, Ordering::SeqCst);
        }
    });

    warn!("Press Ctrl-C to exit");

    let mut net_devices = Vec::new();
    net_init()?;
    dummy_init(&mut net_devices)?;
    net_run(&mut net_devices)?;

    while running.load(Ordering::SeqCst) {
        net_device_output(&net_devices[0], 0x0800, TEST_DATA, None)?;
        sleep(Duration::from_secs(1));
    }
    net_shutdown(&mut net_devices)?;

    Ok(())
}
