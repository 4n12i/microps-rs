use std::net::Ipv4Addr;

use anyhow::bail;
use anyhow::Result;
use tracing::debug;
use tracing::instrument;

use crate::debug_dump;
use crate::net::net_device_index;
use crate::net::net_device_register;
use crate::net::NetDevice as DummyDevice;
use crate::net::NetDeviceOps;
use crate::net::NET_DEVICE_FLAG_UNSPECIFIED;
use crate::net::NET_DEVICE_TYPE_DUMMY;

const DUMMY_MTU: u16 = u16::MAX; // Maximum size of IP datagram

impl NetDeviceOps for DummyDevice {
    #[instrument(skip_all)]
    fn open(&self) -> Result<()> {
        Ok(())
    }

    #[instrument(skip_all)]
    fn close(&self) -> Result<()> {
        Ok(())
    }

    #[instrument(skip_all)]
    fn transmit(&self, device_type: u16, data: &[u8], _dst: Option<&[u8]>) -> Result<()> {
        debug!(
            "dev={}, type=0x{:04x}, len={}",
            self.name,
            device_type,
            data.len()
        );
        debug_dump!(data);
        // Drop data
        Ok(())
    }
}

#[instrument(skip_all)]
pub fn dummy_init(devs: &mut Vec<DummyDevice>) -> Result<()> {
    let index = net_device_index();
    let dev = DummyDevice {
        index,
        name: format!("net{}", index),
        device_type: NET_DEVICE_TYPE_DUMMY,
        mtu: DUMMY_MTU,
        flags: NET_DEVICE_FLAG_UNSPECIFIED,
        hlen: 0,
        alen: 0,
        addr: Ipv4Addr::UNSPECIFIED,
        union: None,
    };

    if net_device_register(dev, devs).is_err() {
        bail!("net_device_register() failure");
    }
    debug!("initialized");

    Ok(())
}
