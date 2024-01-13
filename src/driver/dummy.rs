use crate::net::NetDevice as DummyDevice;
use crate::net::*;
use anyhow::bail;
use anyhow::Result;
use pretty_hex::PrettyHex;
use std::collections::LinkedList;
use std::net::Ipv4Addr;
use tracing::debug;
use tracing::instrument;

const DUMMY_MTU: u16 = u16::MAX; // Maximum size of IP datagram

impl NetDeviceOps for DummyDevice {
    fn open(&self) -> Result<()> {
        Ok(())
    }

    fn close(&self) -> Result<()> {
        Ok(())
    }

    #[instrument(skip_all)]
    fn transmit(&self, type_: u16, data: &[u8], _dst: Option<&[u8]>) -> Result<()> {
        debug!(
            "dev={}, type=0x{:04x}, len={}",
            self.name,
            type_,
            data.len()
        );
        debug!("{:?}", data.hex_dump());
        // Drop data
        Ok(())
    }
}

#[instrument(skip_all)]
pub fn dummy_init(devs: &mut LinkedList<DummyDevice>) -> Result<()> {
    let dev = DummyDevice {
        index: 0,
        name: "net0".to_string(),
        type_: NET_DEVICE_TYPE_DUMMY,
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
