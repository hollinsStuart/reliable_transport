use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::net::UdpSocket;

const MAX_UDP_SIZE: usize = 65_507; // Maximum safe UDP payload size (65,507 bytes)

fn main() -> io::Result<()> {
    // Parse command-line arguments: host, port, filename
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: sender <host> <port> <filename>");
        std::process::exit(1);
    }
    let host = &args[1];
    let port = &args[2];
    let filename = &args[3];

    // Read the file and partition into chunks
    let file_data = read_file(filename)?;
    let chunks = partition_file(&file_data);

    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let destination = format!("{}:{}", host, port);

    // Send chunk one by one
    for (index, chunk) in chunks.iter().enumerate() {
        socket.send_to(chunk, &destination)?;
        println!("Sent chunk {} to {}", index, destination);

        // Listen for acknowledgment
        let mut ack_buffer = [0; 1024];
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
    data.chunks(MAX_UDP_SIZE).map(|chunk| chunk.to_vec()).collect()
}
