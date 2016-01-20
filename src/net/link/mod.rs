/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

//! The link is the layer which emit and receive UDP datagrams using the `net::packet::Packet`
//! It uses `net::connection::Connection` to track state in an UDP exchange between two hosts

use net::connection::*;
use std::net::*;

quick_error! {
    #[derive(Debug)]
    pub enum LinkError {
        BadAddress(descr: String) {
            description(descr)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Link {
    remote_addr: SocketAddr
}

fn check_addr_v4(addr: &Ipv4Addr) -> bool {
    match addr.octets() {
        [127,0,0,0] => false,
        [0,0,0,0] => true,
        [127,_,_,_] => true,
        _ => false
    }
}

fn check_addr_v6(addr: &Ipv6Addr) -> bool {
    addr.is_loopback()
}

impl Link {
    fn new_server(remote: &SocketAddr) -> Result<Link, LinkError> {
        let check = match remote {
            &SocketAddr::V4(addr_v4) => check_addr_v4(addr_v4.ip()),
            &SocketAddr::V6(addr_v6) => check_addr_v6(addr_v6.ip())
        };
        if check {
            Ok(Link{
                remote_addr: remote.clone()
            })
        } else {
            Err(LinkError::BadAddress(
                format!("server must have local address. {} is a remote address", remote)
            ))
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use test_utils::*;
    use test_utils::ResType::*;
    use std::net::SocketAddr;
    use std::str::FromStr;

    #[test]
    fn server_only_on_localhost() {
        // server on remote is forbidden

        let addrs = [
            // ipv4
            Bad("192.168.1.1:0"), Bad("192.168.1.2:0"), Bad("10.10.1.1:0"), Bad("1.1.1.1:0"), Bad("8.8.8.8:0"),
            Good("127.0.0.1:0"), Good("0.0.0.0:0"), Good("127.1.1.1:0"), Good("127.127.127.127:0"),
            // ipv4 - special case
            Bad("127.0.0.0:0"),
            // ipv6
            Bad("[2001:db8::211:22ff:fe33:4455]:0"),
            Bad("[2021:db8::211:22ff:fe33:4455]:0"),
            Bad("[2001:db8::211:22ff:fe33:5555]:0"),
            Bad("[2001:470:26:307:0503:c039:de6b:18d4]:0"),
            Bad("[2001:470:26:307:6484:691f:1e4f:9ef6]:0"),
            Bad("[2001:470:26:307:d90e:988a:779b:af51]:0"),
            Good("[::1]:0")
        ];

        let addrs = addrs.iter()
                         .map(|str| match str {
                            &Bad(addr) => Bad(SocketAddr::from_str(addr)
                                                .expect(format!("BadAddr: {}", addr).as_str())),
                            &Good(addr) => Good(SocketAddr::from_str(addr)
                                                .expect(format!("BadAddr: {}", addr).as_str()))
                         });
        test_loop_with_result(addrs, &Link::new_server)
    }

    /*
    #[test]
    fn init_link() {
        let serv_addr : std::net::SocketAddr = "127.0.0.1:4242".parse().unwrap();

        let server = Link::server(serv_addr);
        let client = Link::client(serv_addr)
    }
    */
}
