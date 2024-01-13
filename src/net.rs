use anyhow::bail;
use anyhow::Result;
use core::fmt;
use pretty_hex::PrettyHex;
use std::collections::LinkedList;
use std::net::Ipv4Addr;
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
    pub type_: u16,
    pub mtu: u16,   // Maximum Transmission Unit
    pub flags: u16, // NET_DEVICE_FLAG_*
    pub hlen: u16,  // Header length
    pub alen: u16,  // Address length
    pub addr: Ipv4Addr,
    pub union: Option<Union>,
    // TODO: void *priv;
}

impl fmt::Debug for NetDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.type_)
    }
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

impl NetDevice {
    // pub fn new(index: usize, type_: u16, mtu: u16) -> Self {
    //     // TODO: Generate index for each device
    //     Self {
    //         index,
    //         name: format!("net{index}"),
    //         type_,
    //         mtu,
    //         flags: NET_DEVICE_FLAG_UNSPECIFIED,
    //         hlen: 0,
    //         alen: 0,
    //         addr: Ipv4Addr::UNSPECIFIED,
    //         union: None,
    //     }
    // }
}

#[instrument(skip_all)]
pub fn net_device_register(dev: NetDevice, devs: &mut LinkedList<NetDevice>) -> Result<()> {
    info!("registered, dev={}, type=0x{:04x}", dev.name, dev.type_);
    devs.push_front(dev);
    Ok(())
}

#[instrument]
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

#[instrument]
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
    type_: u16,
    data: &[u8],
    dst: Option<&[u8]>,
) -> Result<()> {
    if !net_device_is_up(dev) {
        bail!("not opend, dev={}", dev.name);
    }
    if data.len() > dev.mtu as usize {
        bail!(
            "too long, dev={}, mtu={}, len={}",
            dev.name,
            dev.mtu,
            data.len()
        );
    }
    debug!("dev={}, type=0x{}, len={}", dev.name, type_, data.len());
    debug!("{:?}", data.hex_dump());
    if dev.transmit(type_, data, dst).is_err() {
        bail!(
            "device transmit failure, dev={}, len={}",
            dev.name,
            data.len()
        );
    }
    Ok(())
}

#[instrument]
pub fn net_run(devs: &mut LinkedList<NetDevice>) -> Result<()> {
    debug!("open all devices...");
    for dev in devs {
        net_device_open(dev)?;
    }
    debug!("running...");
    Ok(())
}

#[instrument]
pub fn net_shutdown(net_devices: &mut LinkedList<NetDevice>) -> Result<()> {
    debug!("close all devices...");
    for dev in net_devices {
        net_device_close(dev)?;
    }
    debug!("running...");
    Ok(())
}

#[instrument]
pub fn net_init() -> Result<()> {
    info!("initialized");
    Ok(())
}
