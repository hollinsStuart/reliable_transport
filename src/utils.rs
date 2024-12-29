use crc::{Crc, CRC_32_ISO_HDLC};
use rand::Rng;

fn get_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=10000)
}

fn calculate_crc(data: &[u8]) -> u32 {
    let crc_calculator = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    crc_calculator.checksum(data)
}
