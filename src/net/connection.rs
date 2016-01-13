/*
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use std::default::Default;
use std::collections::VecDeque;
use net::{socket, rtt, sequence};

type todo = ();

#[derive(Debug)]
pub struct Connection {
    remote_addr: Box<socket::SocketAddr>,
    socket: Box<socket::Socket>,
    /// used for tracking the acknowledges
    sended_queue: VecDeque<todo>,
    /// internal queue of packets to send
    send_queue: VecDeque<todo>,
    RTT: rtt::RTT,
    MTU: u16,
    sequence: sequence::Sequence<u32>
}

const DEFAULT_MTU : u16 = 1500;

impl Connection {
    pub fn new(socket: Box<socket::Socket>, remote_addr: Box<socket::SocketAddr>)
        -> Connection {
        {
            let ref mut s = socket;
            // TODO config;
            s.set_blocking(false).expect("unable to set socket to non-blocking")
        }

        Connection {
            remote_addr: remote_addr,
            socket: socket,
            sended_queue: VecDeque::with_capacity(35),
            send_queue: VecDeque::with_capacity(35),
            RTT: rtt::RTT::default(),
            MTU: DEFAULT_MTU,
            sequence: Default::default()
        }
    }

    /// Send data on the wire. This is a bare-bone version. You should prefer `send_msg`
    pub fn send_data(&mut self, payload: &mut [u8]) {
        self.socket.send_to(payload, *self.remote_addr);
    }

    /* FIXME: find a better way to send data
       #[inline(always)]
       pub fn send_msg(&mut self, message: message::Message) {
       self.send(message.payload);
       }
       */
}
