/*
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

//! Utilities for paquet sequencing

use std::cmp::*;
use std::iter::Step;
use std::num::{Zero, One};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Sequence<I>(I);

impl<I: Clone + Step + Zero + One> Sequence<I> {
    /// use `let seq : Sequence = Default::default()` if no start value
    pub fn new(start: I) -> Sequence<I> {
        Sequence(start)
    }

    pub fn step(&mut self) -> I {
        match self.0.step(&I::one()) {
            Some(val) => self.0 = val,
            None => self.0 = I::zero()
        };
        self.0.clone()
    }
}
