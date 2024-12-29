mod packet;
mod packet_header;
mod utils;

use packet::Packet;
use packet_header::PacketHeader;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::net::UdpSocket;

const MAX_UDP_SIZE: usize = 1_456; // Maximum safe IP payload size (1,456 bytes)

fn main() -> io::Result<()> {
    // Parse command-line arguments: host, port, filename
    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        eprintln!("Usage: sender <IP address> <port> <window size> <input filename> <log filename>");
        std::process::exit(1);
    }
    let host = &args[1];
    println!("Host: {}", host);
    let port = &args[2];
    println!("Port: {}", port);
    let window_size: i16 = args[3].parse().unwrap_or_else(|_| {
        eprintln!("Invalid window size. Please provide a valid 16-bit signed integer.");
        std::process::exit(1);
    });
    println!("Window size: {}", window_size);
    let input_filename = &args[4];
    println!("Input file: {}", input_filename);

    // Read the file and partition into chunks
    let file_data = read_file(input_filename)?;
    // let chunks: Vec<Packet> = partition_file(&file_data);
    // TODO: use real packets
    let chunks: Vec<Packet> = generate_test_packets(5);

    // Create a UDP socket
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0")?;
    let destination: String = format!("{}:{}", host, port);

    // Send chunk one by one
    for (index, chunk) in chunks.iter().enumerate() {
        let buffer = chunk.encode();
        socket.send_to(&buffer, &destination)?;
        println!("Sent packet, seq={}", chunk.header.seq_num);

        // Listen for acknowledgment
        let mut ack_buffer = [0; MAX_UDP_SIZE];
        match socket.recv_from(&mut ack_buffer) {
            Ok((_, src)) => {
                let ack = String::from_utf8_lossy(&ack_buffer);
                println!("Received ACK from {}: {}", src, ack.trim());
            }
            Err(e) => {
                eprintln!("Failed to receive ACK for chunk {}: {}", index, e);
            }
        }
    }

    Ok(())
}

// Read the file into a vector of bytes
fn read_file(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

// Partition the file data into chunks of MAX_UDP_SIZE
fn partition_file(data: &[u8]) -> Vec<Vec<u8>> {
    data.chunks(MAX_UDP_SIZE)
        .map(|chunk| chunk.to_vec())
        .collect()
}

fn generate_test_packets(count: usize) -> Vec<Packet> {
    let mut packets = Vec::new(); // Create a vector to store the generated packets

    for i in 0..count {
        let header = PacketHeader {
            packet_type: 2,
            seq_num: i as u32,
            length: 0,
            checksum: 0,
        };
        let data = format!("[TEST] Packet number {}", i);
        let data_bytes = data.as_bytes(); // Convert the string to bytes
        let packet = Packet::with_header_and_data(header, data_bytes);
        packets.push(packet);
    }
    packets
}

