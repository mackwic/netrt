/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

pub use expectest::prelude::*;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum ResType<T> {
    Bad(T), Good(T)
}

fn ret_err<T>(_val: &T) -> Result<(), ()> {
    Err(())
}

fn ret_ok<T>(_val: &T) -> Result<(), ()> {
    Ok(())
}

pub fn test_loop_with_result<'a, Iter, T, Res, Err>(values: Iter , action: &Fn(&T) -> Result<Res, Err>)
    where Iter : Iterator<Item=ResType<T>>,
          Res: Debug,
          Err: Debug,
          T: 'a {

    for val in values {
        match val {
            ResType::Bad(ref v) => {expect!(action(v)).to(be_err());()},
            ResType::Good(ref v) => {expect!(action(v)).to(be_ok());()}
        }
    }
}
