/*
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

mod net;
mod strategies;

use net::*;

pub fn add(a: u32, b: u32) -> u32 {
    let _s  = net::socket::Socket::bind("toto");
    a + b
}

#[test]
fn it_works() {
    net::connection::Connection::new("8.8.8.8:4242", 42424)
}
