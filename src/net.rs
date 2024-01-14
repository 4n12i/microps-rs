use crate::debug_dump;
use anyhow::bail;
use anyhow::Result;
use std::net::Ipv4Addr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use tracing::debug;
use tracing::info;
use tracing::instrument;

const _IFNAMSIZ: usize = 16;

pub const NET_DEVICE_TYPE_DUMMY: u16 = 0x0000;
const _NET_DEVICE_TYPE_LOOPBACK: u16 = 0x0001;
const _NET_DEVICE_TYPE_ETHERNET: u16 = 0x0002;

pub const NET_DEVICE_FLAG_UNSPECIFIED: u16 = 0x0000;
const NET_DEVICE_FLAG_UP: u16 = 0x0001;
const _NET_DEVICE_FLAG_LOOPBACK: u16 = 0x0010;
const _NET_DEVICE_FLAG_BROADCAST: u16 = 0x0020;
const _NET_DEVICE_FLAG_P2P: u16 = 0x0040;
const _NET_DEVICE_FLAG_NEED_ARP: u16 = 0x0100;

const _NET_DEVICE_ADDR_LEN: usize = 16;

pub static NET_DEVICE_INDEX: AtomicUsize = AtomicUsize::new(0);

pub fn net_device_index() -> usize {
    NET_DEVICE_INDEX.load(SeqCst)
}

fn net_device_is_up(dev: &NetDevice) -> bool {
    dev.flags & NET_DEVICE_FLAG_UP != 0
}

fn net_device_state(dev: &NetDevice) -> String {
    if net_device_is_up(dev) {
        "up".to_string()
    } else {
        "down".to_string()
    }
}

#[allow(dead_code)]
pub struct NetDevice {
    pub index: usize,
    pub name: String,
    pub device_type: u16,
    pub mtu: u16,   // Maximum Transmission Unit
    pub flags: u16, // NET_DEVICE_FLAG_*
    pub hlen: u16,  // Header length
    pub alen: u16,  // Address length
    pub addr: Ipv4Addr,
    pub union: Option<Union>,
    // TODO: void *priv;
}

pub struct Union {
    _peer: Ipv4Addr,
    _broadcast: Ipv4Addr,
}

pub trait NetDeviceOps {
    fn open(&self) -> Result<()>;
    fn close(&self) -> Result<()>;
    fn transmit(&self, type_: u16, data: &[u8], dst: Option<&[u8]>) -> Result<()>;
}

#[instrument(skip_all)]
pub fn net_device_register(dev: NetDevice, devs: &mut Vec<NetDevice>) -> Result<()> {
    info!(
        "registered, dev={}, type=0x{:04x}",
        dev.name, dev.device_type
    );
    devs.push(dev);
    NET_DEVICE_INDEX.fetch_add(1, SeqCst);
    Ok(())
}

#[instrument(skip_all)]
pub fn net_device_open(dev: &mut NetDevice) -> Result<()> {
    if net_device_is_up(dev) {
        bail!("already opened, dev={}", dev.name);
    }
    if dev.open().is_err() {
        bail!("failure, dev={}", dev.name);
    }
    dev.flags |= NET_DEVICE_FLAG_UP;
    info!("dev={}, state={}", dev.name, net_device_state(dev));
    Ok(())
}

#[instrument(skip_all)]
pub fn net_device_close(dev: &mut NetDevice) -> Result<()> {
    if !net_device_is_up(dev) {
        bail!("not opened, dev{}", dev.name);
    }
    if dev.close().is_err() {
        bail!("failure, dev={}", dev.name);
    }
    dev.flags &= !NET_DEVICE_FLAG_UP;
    info!("dev={}, state={}", dev.name, net_device_state(dev));
    Ok(())
}

#[instrument(skip_all)]
pub fn net_device_output(
    dev: &NetDevice,
    device_type: u16,
    data: &[u8],
    dst: Option<&[u8]>,
) -> Result<()> {
    if !net_device_is_up(dev) {
        bail!("not opened, dev={}", dev.name);
    }
    if data.len() > dev.mtu as usize {
        bail!(
            "too long, dev={}, mtu={}, len={}",
            dev.name,
            dev.mtu,
            data.len()
        );
    }
    debug!(
        "dev={}, type=0x{:04x}, len={}",
        dev.name,
        device_type,
        data.len()
    );
    debug_dump!(data);
    if dev.transmit(device_type, data, dst).is_err() {
        bail!(
            "device transmit failure, dev={}, len={}",
            dev.name,
            data.len()
        );
    }
    Ok(())
}

#[instrument(skip_all)]
pub fn net_run(devs: &mut Vec<NetDevice>) -> Result<()> {
    debug!("open all devices...");
    for dev in devs {
        net_device_open(dev)?;
    }
    debug!("running...");
    Ok(())
}

#[instrument(skip_all)]
pub fn net_shutdown(net_devices: &mut Vec<NetDevice>) -> Result<()> {
    debug!("close all devices...");
    for dev in net_devices {
        net_device_close(dev)?;
    }
    debug!("shutting down");
    Ok(())
}

#[instrument]
pub fn net_init() -> Result<()> {
    info!("initialized");
    Ok(())
}
