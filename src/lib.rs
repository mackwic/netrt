/*
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

#![feature(step_trait)]
#![feature(zero_one)]

// TODO: remove when done
#![allow(dead_code)]

mod net;
mod strategies;

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[test]
fn it_works() {
    //net::connection::Connection::new("8.8.8.8:4242", 42424)
}
