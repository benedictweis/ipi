use std::str::FromStr;
use cli::Args;
use clap::Parser;
use crate::host::IpHost;
use crate::ipv4host::Ipv4Host;

mod cli;
mod ipv4host;
mod host;

fn main() {

    let args = Args::parse();

    println!("{:?}", args);

    let host = Ipv4Host::from_str(&args.ip_address).unwrap();

    println!("IP: {}", host.ip_address);
    println!("TY: {}", host.network_type());
    println!("MA: {}", host.subnet_mask);
    println!("SN: {}", host.subnet());
    println!("BR: {}", host.broadcast());
}
