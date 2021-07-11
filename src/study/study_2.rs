use dns_lookup;

// https://crates.io/crates/dns-lookup
pub fn run() {
    let hostname = "seed.bitcoin.sipa.be";
    let ips: Vec<std::net::IpAddr> = dns_lookup::lookup_host(hostname).unwrap();

    for ip in ips {
        println!("{}", ip);
    }
}
