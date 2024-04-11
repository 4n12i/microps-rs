use std::sync::Arc;
use std::sync::Barrier;
use std::thread;

use anyhow::bail;
use anyhow::Result;
use signal_hook::consts::SIGHUP;
use signal_hook::iterator::Signals;
use tracing::debug;
use tracing::instrument;

use crate::platform::linux::INTR_IRQ_SHARED;

#[allow(dead_code)]
// Interrupt request
struct IrqEntry {
    irq: u8, /* interrupt number: 1~64 */
    flags: u16,
    name: String,
}

#[allow(dead_code)]
struct IntrManager {
    irqs: Vec<IrqEntry>,
    sigmask: Vec<u8>,
    tid: thread::JoinHandle<()>,
    barrier: Arc<Barrier>,
}

#[instrument(skip_all)]
fn intr_request_irq(
    tmp: &mut IntrManager,
    irqs: &mut Vec<IrqEntry>,
    irq: u8,
    flags: u16,
    name: &str,
) -> Result<()> {
    debug!("irq={}, flags={}, name={}", irq, flags, name);

    for entry in &mut *irqs {
        if entry.irq == irq && (entry.flags ^ INTR_IRQ_SHARED != 0 || flags ^ INTR_IRQ_SHARED != 0)
        {
            bail!("conflicts with already registered IRQs");
        }
    }

    let entry = IrqEntry {
        irq,
        flags,
        name: name.to_string(),
    };
    irqs.push(entry);
    tmp.sigmask.push(irq);
    debug!("registered: irq={}, name={}", irq, name);

    Ok(())
}

#[instrument(skip_all)]
pub fn intr_raise_irq(_irq: u8) -> Result<()> {
    // pthread_kill
    Ok(())
}

#[instrument(skip_all)]
fn intr_thread(irqs: &mut Vec<IrqEntry>) -> Result<()> {
    let barrier = Arc::new(Barrier::new(2));

    debug!("start...");
    barrier.wait();

    let signals = &[SIGHUP];
    let mut sigs = Signals::new(signals)?;
    if let Some(signal) = (&mut sigs).into_iter().next() {
        match signal {
            SIGHUP => {
                debug!("SIGHUP");
            }
            _ => {
                for entry in &mut *irqs {
                    if entry.irq as i32 == signal {
                        debug!("irq={}, name={}", entry.irq, entry.name);
                        // TODO: entry->handler(entry->irq, entry->dev);
                    }
                }
            }
        }
    }

    debug!("terminated");
    Ok(())
}

#[instrument]
pub fn intr_run() -> Result<()> {
    // signal mask setting
    // start a thread for interrupt handling
    // wait for starting thread (barrier wait)

    Ok(())
}

#[instrument]
pub fn intr_shutdown() -> Result<()> {
    // check if another thread started
    // send a signal to another thread
    // wait for finishing another thread (join)
    Ok(())
}

#[instrument]
pub fn intr_init() -> Result<()> {
    Ok(())
}
