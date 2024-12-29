use crate::packet_header::PacketHeader;
use std::fmt;

use crc::{Crc, CRC_32_ISO_HDLC};
use rand::Rng;

pub const DATA_SIZE: usize = 1456;
pub const MAX_PACKET_SIZE: usize = 1472;
#[derive(Debug, Clone)]
pub struct Packet {
    pub header: PacketHeader,
    pub data: [u8; DATA_SIZE],
}

impl Packet {
    // Default constructor
    pub fn new() -> Self {
        Self {
            header: PacketHeader {
                packet_type: 0,
                seq_num: 0,
                length: 0,
                checksum: 0,
            },
            data: [0; DATA_SIZE],
        }
    }

    // Constructor with header and data
    pub fn with_header_and_data(mut header: PacketHeader, data: &[u8]) -> Self {
        let mut packet_data = [0; DATA_SIZE];
        let copy_size = DATA_SIZE.min(data.len());
        packet_data[..copy_size].copy_from_slice(&data[..copy_size]);

        // Recalculate length based on the actual data size
        header.length = copy_size as u32;

        // Recalculate checksum based on the actual data
        header.checksum = calculate_crc(&packet_data[..copy_size]);

        Self {
            header,
            data: packet_data,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(MAX_PACKET_SIZE);

        // Serialize the PacketHeader (u32 fields) into the buffer
        buffer.extend_from_slice(&self.header.packet_type.to_le_bytes());
        buffer.extend_from_slice(&self.header.seq_num.to_le_bytes());
        buffer.extend_from_slice(&self.header.length.to_le_bytes());
        buffer.extend_from_slice(&self.header.checksum.to_le_bytes());

        // Serialize the data
        let data_to_encode = &self.data[..self.header.length as usize];
        buffer.extend_from_slice(data_to_encode);

        // Pad the remaining space with zeros to ensure the buffer is 1456 bytes
        let padding_size = DATA_SIZE - data_to_encode.len();
        buffer.extend(std::iter::repeat(0).take(padding_size));

        buffer
    }
}

fn calculate_crc(data: &[u8]) -> u32 {
    let crc_calculator = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    crc_calculator.checksum(data)
}

fn validate_checksum(packet: &Packet) -> bool {
    calculate_crc(&packet.data[..packet.header.length as usize]) == packet.header.checksum
}

// Implement the `Display` trait for logging/debugging
impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Packet Header:\n  Type: {}\n  SeqNum: {}\n  Length: {}\n  Checksum: {}\n",
            self.header.packet_type, self.header.seq_num, self.header.length, self.header.checksum
        )
    }
}
