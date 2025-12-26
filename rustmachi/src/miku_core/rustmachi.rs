use tun_rs::SyncDevice;

use crate::miku_core::server::{self, create_tun_interface, create_udp_listener, load_config};





pub fn rustmachi (){
    let config = load_config();
    let socket = create_udp_listener(config);
    let tunnel: SyncDevice = create_tun_interface();
    server::server(socket, tunnel);

}