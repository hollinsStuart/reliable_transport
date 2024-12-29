#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PacketHeader {
    pub packet_type: u32,  // 0: START; 1: END; 2: DATA; 3: ACK
    pub seq_num: u32,      // Sequence number
    pub length: u32,       // Length of data; 0 for ACK packets
    pub checksum: u32,     // 32-bit CRC
}
