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

    pub fn stepu(&mut self) { let _ = self.step(); }
}

#[test]
fn test_step_incr() {
    let mut seq = Sequence::new(0 as u8);
    assert_eq!(0, seq.0);
    seq.step();
    assert_eq!(1, seq.0);
    seq.step();
    assert_eq!(2, seq.0);
    seq.step();
    assert_eq!(3, seq.0);
    seq.step();
    assert_eq!(4, seq.0)
}

#[test]
fn test_step_cycle() {
    use std::u8;
    let mut seq = Sequence::new(0 as u8);
    for _ in 0..(u8::MAX as u16 + 1) { seq.stepu() }
    assert_eq!(0, seq.0);
    seq.step();
    for _ in 0..(u8::MAX as u16 + 1) { seq.stepu() }
    assert_eq!(1, seq.0)
}
