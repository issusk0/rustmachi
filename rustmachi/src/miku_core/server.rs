//This is for server TUN in the Peer to Peer VPN
use serde::Deserialize;
use std::fs;
use tun_rs::{DeviceBuilder, SyncDevice};
use std::net::{SocketAddrV4,Ipv4Addr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Deserialize)]
pub struct Target{
    real_addr: String,
    virtual_addr: String,
    port: u16,
    name: String,


}
impl Target{
    pub fn get_target_name(&self) -> &String{
        &self.name
    }

    pub fn get_target_port (&self) -> u16{
        self.port
    }
    pub fn get_target_v_addr(&self) -> &String {
        &self.virtual_addr
    }
    pub fn get_target_r_addr(&self) -> &String {
        &self.real_addr
    }
}
#[derive(Deserialize, Debug)]
pub struct Server{
    virtual_addr: String,
    real_addr: String,
    port: u16,
    name:  String,

}
impl Server{
    pub fn get_server_name(&self) -> &String {
        &self.name
    }
    pub fn get_server_port (&self) -> u16 {
            self.port

    }
    pub fn get_server_addr (&self) -> &String{
       &self.real_addr
    }
    pub fn get_server_virtual_addr(&self) -> &String {
        &self.virtual_addr
    }
}
pub fn server(socket: UdpSocket, tunnel: SyncDevice){
    let socket_clone = socket.try_clone().expect("Couldnt clone the socket");
    let socket_clone_2 = socket.try_clone().expect("Failed to clone");
    
    let tun = Arc::new(Mutex::new(tunnel));
    let tun_1 = Arc::clone(&tun);
    let tun_2 = Arc::clone(&tun);

    let remote_peer = Arc::new(Mutex::new(None::<std::net::SocketAddr>));
    let remote_peer_clone = Arc::clone(&remote_peer);

    // udp -> tun
    let handle_upd = thread::spawn(move ||{
        let mut buffer = [0u8; 1400];
        loop {
            if let Ok((n, src_addr)) = socket_clone.recv_from(&mut buffer) {
                // update peer internet from
                {
                    let mut peer_lock = remote_peer_clone.lock().unwrap();
                    *peer_lock = Some(src_addr);
                }
                
                // inject to tunel
                let tun_guard = tun_1.lock().unwrap();
                if let Ok(_) = tun_guard.send(&buffer[..n]) {
                    println!("<<< [UDP -> TUN] Recibidos {} bytes de {}", n, src_addr);
                }
            } 
        }
    });

    // tun -> internet
    let handle_tun = thread::spawn (move || {
        let mut buffer = [0u8; 1500];
        loop {
            // read data from from tun
            let n = {
                let tun_guard = tun_2.lock().unwrap();
                tun_guard.recv(&mut buffer).unwrap_or(0)
            };

            if n > 0 {
                let peer_lock = remote_peer.lock().unwrap();
                if let Some(target_addr) = *peer_lock {
                    if let Ok(_) = socket_clone_2.send_to(&buffer[..n], target_addr) {
                        println!(">>> [TUN -> UDP] Enviando {} bytes a {}", n, target_addr);
                    }
                }
            }
        }
    });

    let _ = handle_upd.join();
    let _ = handle_tun.join();
}

//create tunel to send-recv data
pub fn create_udp_listener(config: Server) -> UdpSocket{
    let addr: Ipv4Addr = config.real_addr.parse().expect("Couldnt parse addr");
    let socket_udp = SocketAddrV4::new(addr, config.port);
    let socket = UdpSocket::bind(socket_udp).expect("Couldn't bind the socket!");
    socket
}

//create tunel to recv-send
pub fn create_tun_interface()-> SyncDevice{
    let config = load_config();
    let config_virtual_addr: Ipv4Addr = config.virtual_addr.parse().expect("Couldn't parse virtual address");
    let target = load_target();
    let target_virtual_addr: Ipv4Addr = target.virtual_addr.parse().expect("Couldn't parse target virtual address");
    let dev = DeviceBuilder::new()
        .name("rustmachi")
            //[IPV4, Netmask, IPVDIRECTION]
        .ipv4(config_virtual_addr, 24, Some(target_virtual_addr))
        .mtu(1400)
        .build_sync()
        .unwrap();
    dev
}


pub fn load_config() -> Server{
    let config_toml = fs::read_to_string("config.toml").expect("Error when find the config.toml file");
    let config: Server = toml::from_str(&config_toml).expect("Not able to load toml.");
    config
}
pub fn load_target() -> Target{
    let target_toml = fs::read_to_string("tun.toml").expect("Error when find the tun.toml file");
    let target: Target = toml::from_str(&target_toml).expect("not able to read toml structure.");
    target
}





