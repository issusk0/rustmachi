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
    let socket_clone_2=socket.try_clone().expect("Failed to clone");
    let tun = Arc::new(Mutex::new(tunnel));
    let tun_1 = Arc::clone(&tun);
    let tun_2 = Arc::clone(&tun);
    let target = Arc::new(load_target());
    thread::spawn(move ||{
        let udp_tun = Arc::clone(&tun_1);
        let mut buffer = [0u8;1400];
        loop {
            let (n, s) = socket_clone.recv_from(&mut buffer).expect("not able to recv all data");
            let data = &buffer[0..n];
            let tun_guard = udp_tun.lock().unwrap();
            tun_guard.send(data).expect("Error when sending data to tun");
            
        }

    }).join().expect("UDP socket ended");
    thread::spawn (move || {
        //aca se inicia el tun
        let mut buffer = [0u8; 1400];
        loop {
            let n = {
                let tun_guard = tun_2.lock().unwrap(); // Use the cloned Arc
                tun_guard.recv(&mut buffer).expect("No data found")
            };

            socket_clone_2.send_to(&buffer[0..n], target.real_addr.as_str()).ok();
        }
        
    }).join().expect("Tunnel ended");

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





