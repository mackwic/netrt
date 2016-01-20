/*
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

#![feature(step_trait)]
#![feature(zero_one)]
#![feature(slice_patterns)]

// TODO: remove when done
#![allow(dead_code)]
#![allow(unused_imports)]

/** TEST UTILS **/
#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;
#[cfg(test)]
mod test_utils;
/** END TEST UTILS */

#[macro_use] extern crate quick_error;

mod net;
mod strategies;

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[test]
fn it_works() {
    //net::connection::Connection::new("8.8.8.8:4242", 42424)
}
