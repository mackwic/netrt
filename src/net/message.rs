/*
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

#[derive(Debug)]
struct Message {
    /// The message ID should be unique for each kind of message
    message_id: u32,

    /// For debug purposes, being able to print the name
    #[cfg(debug)]
    message_name: String,

    /// Opaque [u8]. This is your data
    payload: [u8]
}


