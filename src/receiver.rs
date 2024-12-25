use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?; // Bind to a specific port
    println!("Receiver listening on 127.0.0.1:8080");

    let mut buf = [0; 1024];
    loop {
        let (size, src) = socket.recv_from(&mut buf)?;
        println!(
            "Received {} bytes from {}: {}",
            size,
            src,
            String::from_utf8_lossy(&buf[..size])
        );
    }
}
