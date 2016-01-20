/*
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

//use std::cmp::*;
use std::convert::{From, Into};

pub const LONG_SIZE : u8 = 64;
pub const SHORT_SIZE : u8 = 32;
pub const MASK_SIZE : u8 = 0b0100_0000;

#[derive(Debug, PartialEq, Eq)]
pub enum IdSizeType { Long, Short }
pub type LongSize = u64;
pub type ShortSize = u32;

pub trait IdSize {
    fn size(&self) -> u8;
    fn is_long(&self) -> bool { self.size() == LONG_SIZE }
    fn is_short(&self) -> bool { self.size() == SHORT_SIZE }
}

impl IdSize for LongSize { fn size(&self) -> u8 { LONG_SIZE } }
impl IdSize for ShortSize { fn size(&self) -> u8 { SHORT_SIZE } }

impl IdSizeType {
    pub fn make_new(&self) -> Box<IdSize> {
        match self {
            &IdSizeType::Long => Box::new(0u64 as LongSize),
            &IdSizeType::Short => Box::new(0u32 as ShortSize)
        }
    }
}

impl<'a> From<&'a u8> for IdSizeType {
    fn from(other: &'a u8) -> IdSizeType {
        match other & MASK_SIZE {
            MASK_SIZE => IdSizeType::Short,
            _ => IdSizeType::Long
        }
    }
}

impl From<u8> for IdSizeType {
    fn from(other: u8) -> IdSizeType { IdSizeType::from(&other) }
}

#[test]
fn test_conversion() {
    let short_vals = [0b0100_0000, 0b0100_0001, 0b0101_0000, 0b1111_1111];
    let long_vals = [0b0000_0000, 0b0000_1111, 0b1011_1111, 0b1010_1010];
    let tests = [(IdSizeType::Short, &short_vals), (IdSizeType::Long, &long_vals)];

    for &(ref expected, ref values) in tests.into_iter() {
        for &v in values.into_iter() { assert_eq!(*expected, IdSizeType::from(&v)) }
    }
}

#[test]
fn equivalence() {
    let vals = [
        0b0100_0000, 0b0100_0001, 0b0101_0000, 0b1111_1111,
        0b0000_0000, 0b0000_1111, 0b1011_1111, 0b1010_1010
    ];

    for v in vals.iter() {
        let t1 = IdSizeType::from(v);
        let t2 : IdSizeType = v.into();
        assert_eq!(t1, t2);
    }
}

#[test]
fn test_is_long() {
    let tmp = IdSizeType::Long;
    assert_eq!(true, tmp.make_new().is_long());
    assert_eq!(false, tmp.make_new().is_short());
    let tmp = IdSizeType::Short;
    assert_eq!(false, tmp.make_new().is_long());
    assert_eq!(true, tmp.make_new().is_short())
}
