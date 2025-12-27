use std::net::{Ipv6Addr, SocketAddrV6};

use tun_rs::SyncDevice;

use crate::miku_core::server::{self, create_tun_interface, create_udp_listener, load_config, load_target};





pub fn rustmachi (){
    let config = load_config();
    let target = load_target();
    let target_addr: Ipv6Addr = target.real_addr.parse().expect("Error while parsing ipv6(rustmachi 14)");
    let socket = create_udp_listener(config);
    let tunnel: SyncDevice = create_tun_interface();
    let initial_peer: SocketAddrV6 = SocketAddrV6::new(target_addr, target.get_target_port(), 0,0);
    server::server(initial_peer,socket, tunnel);

}