/*
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use std::convert::{From, Into};
use net::packet::size::*;



mod header {

    use std::convert::{From, Into};
    use net::packet::size::*;

    /// a packet could have data or be an heartbeat
    #[derive(Debug, PartialEq, Eq)]
    pub enum PacketType {
        HeartBeat, Data
    }
    const MASK_TYPE : u8 = 0b1000_0000;

    /// hold the packet attributes
    #[derive(Debug, PartialEq, Eq)]
    struct PacketDescriptor(PacketType, IdSizeType);

    /// internal binary representation of PacketFlags. Use the accessors
    #[derive(Debug, PartialEq, Eq)]
    struct PacketFlags(u64);

    /// internal binary reprentation. Use the methods
    #[derive(Debug, PartialEq, Eq)]
    struct AckBitField(u64);

    #[derive(Debug, PartialEq, Eq)]
    struct PacketHeader<Size : IdSize> {
        flags: PacketFlags,
        acks:  AckBitField,
        packet_id: Size,
        last_ack:  Size
    }

    impl<'a> From<&'a u8> for PacketDescriptor {
        fn from(other: &'a u8) -> PacketDescriptor {
            PacketDescriptor(PacketType::from(other), IdSizeType::from(other))
        }
    }
    impl From<u8> for PacketDescriptor {
        fn from(other: u8) -> PacketDescriptor { PacketDescriptor::from(&other) }
    }

    impl<'a> From<&'a u8> for PacketType {
        fn from(other: &'a u8) -> PacketType {
            match other & MASK_TYPE {
                MASK_TYPE => PacketType::Data,
                _ => PacketType::HeartBeat
            }
        }
    }
    impl Into<PacketType> for u8 {
        fn into(self) -> PacketType {
            match self & MASK_TYPE {
                MASK_TYPE => PacketType::Data,
                _ => PacketType::HeartBeat
            }
        }
    }

    #[test]
    fn equivalence_packet_type() {
        let vals = [
            0b1100_0000, 0b1100_0001, 0b1101_0000, 0b1111_1111,
            0b0000_0000, 0b0000_1111, 0b1011_1111, 0b1010_1010
        ];

        for v in vals.iter() {
            let t1 = PacketType::from(v);
            let t2 : PacketType = v.into();
            assert_eq!(t1, t2);
        }
    }

    impl PacketFlags {
        pub fn header(&self) -> PacketDescriptor {
            let first_byte : u8 = (self.0 >> 56) as u8;
            PacketDescriptor::from(first_byte)
        }

        pub fn ack_bitfield(&self) -> AckBitField {
            AckBitField((self.0 << 8) & 0xFFFF_FFFF_FFFF_FF00)
        }
    }

    #[test]
    fn header() {
        use net::packet::size::IdSizeType;
        use super::PacketDescriptor;

        let tmp = PacketFlags(0xC000_0000_0000_0000);
        assert_eq!(PacketDescriptor(PacketType::Data, IdSizeType::Short), tmp.header());
        let tmp = PacketFlags(0x8000_0000_0000_0000);
        assert_eq!(PacketDescriptor(PacketType::Data, IdSizeType::Long), tmp.header());
        let tmp = PacketFlags(0x4000_0000_0000_0000);
        assert_eq!(PacketDescriptor(PacketType::HeartBeat, IdSizeType::Short), tmp.header());
        let tmp = PacketFlags(0);
        assert_eq!(PacketDescriptor(PacketType::HeartBeat, IdSizeType::Long), tmp.header());
    }

    #[test]
    fn ack_bitfield() {
        let tmp = PacketFlags(0xdead_beef_b00b_cafe);
        assert_eq!(AckBitField(0xad_beef_b00b_cafe_00), tmp.ack_bitfield());
    }

}

mod body {
    use net::packet::size::*;

    type RawData = [u8];

    #[derive(Debug, PartialEq, Eq)]
    struct DataChunkHeader<Size : IdSize> {
        size: u16,
        number: u16,
        head: Size,
        data: RawData
    }

    #[derive(Debug)]
    enum PacketBody<Size : IdSize> {
        HeartBeat,
        Data(RawData),
        DataChunk(DataChunkHeader<Size>)
    }

}
