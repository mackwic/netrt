/*
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

//! Utilities for computing the Round Time Trip

use std::default::Default;

pub type rtt_size = u16;

#[derive(Debug)]
pub struct RTT {
    /// the estimated Round Time Trip (RTT) of the connection in ms
    /// used for the congestion avoidance strategy. defaults to 100ms
    estimed_RTT: rtt_size,
    /// time until we discard the packets. defaults to 1000 ms
    max_RTT: rtt_size
}

// TODO load from config
const DEFAULT_ESTIMATED_RTT : rtt_size = 100;
const DEFAULT_MAX_RTT : rtt_size = 1000;

impl Default for RTT {
    fn default() -> Self {
        RTT::new(DEFAULT_ESTIMATED_RTT, DEFAULT_MAX_RTT)
    }
}

impl RTT {
    /// Use std::default::Default() for default values
    pub fn new(estimed_RTT: rtt_size, max_RTT: rtt_size) -> RTT {
        RTT { estimed_RTT: estimed_RTT, max_RTT: max_RTT }
    }
}
