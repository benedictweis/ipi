use clap::Parser;

/// Simple program to examine ip subnets via a single hosts ip address
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {

    /// ip address to inspect
    pub ip_address: String,

    /// force ipv4, will fail if ip_address is ipv6
    #[arg(short = '4', long, default_value_t = false)]
    pub force_ipv4: bool,

    /// force ipv6, will fail if ip_address is ipv4
    #[arg(short = '6', long, default_value_t = false)]
    pub force_ipv6: bool,
}
