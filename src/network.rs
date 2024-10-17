use std::net::SocketAddrV4;
use std::net::UdpSocket;
use std::net::Ipv4Addr;
use socket2::SockAddr;
use socket2::{Type, Domain, Protocol, Socket};

pub fn get_broadcast_address(host_ip: Ipv4Addr, netmask: Ipv4Addr) -> Ipv4Addr{
    let host_ip_bits = u32::from(host_ip);
    let netmask_bits = u32::from(netmask);
    Ipv4Addr::from(host_ip_bits | (!netmask_bits))
}

pub fn serverDemo0(ipv4_broad_addr: Ipv4Addr) {
    println!("{}",ipv4_broad_addr.to_string());
    let socketstr = ipv4_broad_addr.to_string() + ":7878";
    let socket = UdpSocket::bind(socketstr).expect("Unable to bind");
    socket.set_broadcast(true).expect("Unable to set socket to broadcast");
    let _ = socket.set_nonblocking(false);

    let buffer = [0;1024];
    while true {
        println!("Send data");
        socket.send(b"StayingAlive");
    }
}

pub fn server(broad_addr: Ipv4Addr){
    let socket = Socket::new(Domain::IPV6, Type::DGRAM, None).unwrap();
    let addr: SocketAddrV4 = "10.253.127.255:8888".parse().unwrap();
    socket.set_broadcast(true);
    socket.set_reuse_address(true);


    socket.bind(&(addr.into()));
    while true {
        socket.send(b"AAAA");
    }
}
