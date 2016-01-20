/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

//#![feature(convert)]
#![allow(dead_code)]

extern crate mio;

use mio::*;
use mio::udp::UdpSocket;


const SERVER: mio::Token = mio::Token(0);

struct Test {socket: UdpSocket}
impl mio::Handler for Test {
    type Timeout = ();
    type Message = u64;

    fn ready(&mut self, _eloop: &mut mio::EventLoop<Test>, _token: mio::Token, event: mio::EventSet) {
        /*
        println!("READY. Token is {:?}", token);
        println!("EVENT: is_readable: {:?}, is_writable: {:?}, is_error: {:?}",
            event.is_readable(), event.is_writable(), event.is_error()
        );
        */
        print!(".");

        if !event.is_readable() {
            return;
        } else { print!("\n") }

        loop {
            let mut buffer = [0u8; 65535];
            match self.socket.recv_from(buffer.as_mut()) {
                Ok(Some((len, source))) => {
                    println!("** got {:?} bytes from {:?}", len, source);
                    println!("GOT: {}", String::from_utf8_lossy(buffer.as_ref()))
                }
                Ok(None) => { println!("no data to read"); return },
                Err(e) => {println!("error: {:?}", e); return }
            }

        }

    }
}

pub fn main() {

    let serv_addr : std::net::SocketAddr = "127.0.0.1:4242".parse().unwrap();

    let server = UdpSocket::bound(&serv_addr).unwrap();
    let client = UdpSocket::v4().unwrap();

    let mut eloop = mio::EventLoop::new().unwrap();
    eloop.register(&server, SERVER, EventSet::all(), PollOpt::level()).expect("error in the event loop setup");


    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(5));
        println!("sending data to udp socket");

        client.send_to("THIS IS DOG".as_bytes(), &serv_addr).expect("send data");
        let v = ["hello", " ", "world", " !", "how ", "are ", "you ", "doing ", "?",
            "loooooooooooooooooong message omgomgomgomogmomgomgomgomogm this is sooooooooooooooooooooo
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            loooooooooooooooooong loooooooooooooooooong llooooooooooooooooooong llooooooooooooooooooong
            "
        ];
        for string in v.iter() {
            client.send_to(string.as_bytes(), &serv_addr).expect("can't send data to socket");
        }
    });


    println!("running server");
    eloop.run(&mut Test{socket: server}).expect("cant' run the event loop");
}
