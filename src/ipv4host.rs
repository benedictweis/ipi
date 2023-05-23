use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use crate::host;
use crate::host::{IpHost, num_to_ipv4_mask, ParseHostError};

#[derive(Debug)]
pub struct Ipv4Host {
    pub ip_address: Ipv4Addr,
    pub subnet_mask: Ipv4Addr,
}

impl IpHost for Ipv4Host {
    type Inner = Ipv4Addr;

    fn new(ip_address: Self::Inner, subnet_mask: Self::Inner) -> Self {
        Ipv4Host { ip_address, subnet_mask}
    }

    fn inner(self) -> (Self::Inner, Self::Inner) {
        (self.ip_address, self.subnet_mask)
    }

    fn network_type(&self) -> char {
        host::network_type(self.ip_address.octets()[0])
    }

    fn subnet(&self) -> Self::Inner {
        let subnet_num = u32::from(self.ip_address) & u32::from(self.subnet_mask);
        Ipv4Addr::from(subnet_num)
    }

    fn broadcast(&self) -> Self::Inner {
        let mask_num = u32::from(self.subnet_mask);
        let subnet_num = u32::from(self.ip_address) & mask_num;
        Ipv4Addr::from(subnet_num + !mask_num)
    }
}

impl FromStr for Ipv4Host {
    type Err = ParseHostError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut subnet_mask_num = 24;

        let s = s.trim().replace([' ', '\n'], "");
        let nets: Vec<&str> = s.split('/').collect();

        if nets.len() != 1 && nets.len() != 2{
            return Err(ParseHostError);
        } else if nets.len() == 2 {
            subnet_mask_num = nets[1].parse::<usize>().unwrap();
        }

        Ok(Ipv4Host::new(nets[0].parse::<Ipv4Addr>().unwrap(), num_to_ipv4_mask(subnet_mask_num)))
    }
}