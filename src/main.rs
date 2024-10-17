mod network;

fn main() {
    let broad = network::get_broadcast_address(std::net::Ipv4Addr::new(10,253,0,0), std::net::Ipv4Addr::new(255,255,128,0));
    network::server(broad);
}
