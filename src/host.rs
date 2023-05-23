use std::net::Ipv4Addr;
use std::str::FromStr;

pub trait IpHost: FromStr {
    type Inner;

    fn new(ip_address: Self::Inner, subnet_mask: Self::Inner) -> Self;
    fn inner(self) -> (Self::Inner, Self::Inner);
    fn network_type(&self) -> char;
    fn subnet(&self) -> Self::Inner;
    fn broadcast(&self) -> Self::Inner;
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseHostError;

pub fn network_type(first_octet: u8) -> char {
    match first_octet {
        o if o <= 127 => 'A',
        o if o <= 191 => 'B',
        o if o <= 223 => 'C',
        o if o <= 239 => 'D',
        _ => 'E'
    }
}

pub fn num_to_ipv4_mask(num: usize) -> Ipv4Addr {
    let mut mask: u32 = 0;
    for _ in 0..num {
        mask += 1;
        mask <<= 1;
    }
    if num == 32 { mask += 1 };
    mask <<= 31_usize.saturating_sub(num);
    Ipv4Addr::from(mask)
}