use common::Interface;
use network_interface::{Addr, NetworkInterface, NetworkInterfaceConfig};

pub fn get_networking_interfaces() -> Vec<Interface> {
    let network_interfaces = NetworkInterface::show().unwrap_or_default();
    network_interfaces.into_iter().map(into_interface).collect()
}

fn into_interface(i: NetworkInterface) -> Interface {
    let mut interface = Interface {
        name: i.name,
        ip: Default::default(),
        broadcast: Default::default(),
        netmask: Default::default(),
        mac: i.mac_addr.unwrap_or_default(),
    };

    if let Some(Addr::V4(addr)) = i.addr {
        interface.ip = addr.ip.to_string();
        if let Some(broadcast) = addr.broadcast {
            interface.broadcast = broadcast.to_string();
        }
        if let Some(netmask) = addr.netmask {
            interface.netmask = netmask.to_string();
        }
    }

    if let Some(Addr::V6(addr)) = i.addr {
        interface.ip = addr.ip.to_string();
        if let Some(broadcast) = addr.broadcast {
            interface.broadcast = broadcast.to_string();
        }
        if let Some(netmask) = addr.netmask {
            interface.netmask = netmask.to_string();
        }
    }
    interface
}
