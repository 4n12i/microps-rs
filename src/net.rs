use crate::util::*;
use anyhow::bail;
use anyhow::Result;
use std::collections::LinkedList;
use std::net::Ipv4Addr;

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
    // next: Box<NetDevice>,
    index: usize,
    // name: [char; IFNAMSIZ],
    pub name: String,
    type_: u16,
    mtu: u16,   // Maximum Transmission Unit
    flags: u16, // NET_DEVICE_FLAG_*
    hlen: u16,  // Header length
    alen: u16,  // Address length
    // addr: [u8; NET_DEVICE_ADDR_LEN],
    addr: Ipv4Addr,
    union: Option<Union>,
    // net_device_ops: fn(&NetDeviceOps),
    // TODO: void *priv;
}

#[allow(dead_code)]
pub struct Union {
    peer: Ipv4Addr,
    broadcast: Ipv4Addr,
}

pub trait NetDeviceOps {
    fn open(&self) -> Result<()>;
    fn close(&self) -> Result<()>;
    fn transmit(&self, type_: u16, data: &[u8]) -> Result<()>;
}

impl NetDevice {
    pub fn new(index: usize, type_: u16, mtu: u16, flags: u16, union: Option<Union>) -> Self {
        // TODO: Generate index for each device
        Self {
            index,
            name: format!("net{index}"),
            type_,
            mtu,
            flags,
            hlen: 0,
            alen: 0,
            addr: Ipv4Addr::UNSPECIFIED,
            union,
        }
    }
}

pub fn net_device_register(dev: NetDevice, devs: &mut LinkedList<NetDevice>) -> Result<()> {
    infof(&format!(
        "registered, dev={}, type=0x{:04x}",
        dev.name, dev.type_
    ));
    devs.push_front(dev);
    Ok(())
}

pub fn net_device_open(dev: &mut NetDevice) -> Result<()> {
    if net_device_is_up(dev) {
        errorf(&format!("already opened, dev={}", dev.name));
        bail!("Error")
    }
    if dev.open().is_err() {
        errorf(&format!("failure, dev={}", dev.name));
        bail!("Error")
    }
    dev.flags |= NET_DEVICE_FLAG_UP;
    infof(&format!(
        "dev={}, state={}",
        dev.name,
        net_device_state(dev)
    ));
    Ok(())
}

pub fn net_device_close(dev: &mut NetDevice) -> Result<()> {
    if !net_device_is_up(dev) {
        errorf(&format!("not opened, dev{}", dev.name));
        bail!("Error")
    }
    if dev.close().is_err() {
        errorf(&format!("failure, dev={}", dev.name));
        bail!("Error")
    }
    dev.flags &= !NET_DEVICE_FLAG_UP;
    infof(&format!(
        "dev={}, state={}",
        dev.name,
        net_device_state(dev)
    ));
    Ok(())
}

pub fn net_device_output(dev: &NetDevice, type_: u16, data: &[u8], len: usize) -> Result<()> {
    if !net_device_is_up(dev) {
        errorf(&format!("not opend, dev={}", dev.name));
        bail!("Error")
    }
    if len > dev.mtu as usize {
        errorf(&format!(
            "too long, dev={}, mtu={}, len={}",
            dev.name, dev.mtu, len
        ));
        bail!("Error")
    }
    debugf(&format!("dev={}, type=0x{}, len={}", dev.name, type_, len));
    debugdump(data);
    if dev.transmit(type_, data).is_err() {
        errorf(&format!(
            "device transmit failure, dev={}, len={}",
            dev.name, len
        ));
        bail!("Error")
    }
    Ok(())
}

pub fn net_run(devs: &mut LinkedList<NetDevice>) -> Result<()> {
    debugf("open all devices...");
    for dev in devs {
        net_device_open(dev)?;
    }
    debugf("running...");
    Ok(())
}

pub fn net_shutdown(net_devices: &mut LinkedList<NetDevice>) -> Result<()> {
    debugf("close all devices...");
    for dev in net_devices {
        net_device_close(dev)?;
    }
    debugf("running...");
    Ok(())
}

pub fn net_init() -> Result<()> {
    infof("initialized");
    Ok(())
}
