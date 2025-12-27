## Rustmachi
Rustmachi is a peer-to-peer Layer 3 (TUN-based) VPN implemented in Rust, inspired by tools like Hamachi. It establishes a virtual IPv4 network between peers using a TUN interface and encapsulates IP packets over UDP, enabling direct host-to-host communication without relying on centralized routing of traffic.

The project is focused on low-level networking, packet forwarding, and virtual network interface management, serving both as a functional VPN prototype and as an educational exploration of VPN internals, routing, and tunneling mechanisms in Rust.
It uses the `tun-rs` crate for TUN interface management.

# How to setup
You have these two files:

 - config.toml
 - tun.toml
In both files you have the same format (not the same data) you make sure you fill it correctly because it might not work:

```toml
name = ""
real_addr = "your_real_ipv6_address"
virtual_addr = "10.0.0.x"
port = "your_udp_port"
```
#
Once you fill both of those files you set up the binary running `cargo build` and make sure that the binary is in the root directory (not of the system) if you clone in `~/Documents/git/rustmachi` then make sure that rustmachi is in `~/Documents/git/rustmachi/rustmachi` because thats where the config files are, example directory structure:
```toml
~/your/path/rustmachi
.
├── Cargo.lock
├── Cargo.toml
├── config.toml
├── rustmachi
├── src
│   ├── main.rs
│   └── miku_core
│       ├── mod.rs
│       ├── rustmachi.rs
│       └── server.rs
```

Once you done with that, then you are free to run rustmachi as sudo ./rustmachi
#
# Important
Rustmachi uses IPv6 UDP sockets as the transport layer, so make sure that you put your data without the []:: in both of the files, otherwise you will have an error.
The tun interface operates at L3.
I havent tested on windows yet, so i don´t know if it works in windows or not.
Be aware that Rustmachi is a project for me to explore these lands, if you use it may be unstable, and may exhibit packet loss or high latency.

