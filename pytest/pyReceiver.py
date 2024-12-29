import socket
from ctypes import sizeof
from Packet import *  # Import the Packet struct definition
import zlib

def calculate_crc(received_data):
    return zlib.crc32(received_data) & 0xffffffff

# Set up the UDP socket
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.bind(('localhost', 8888))  # Use the appropriate IP and port

print("Listening for packets...")
count = 0
while count < 10:
    # Receive the packet
    data, addr = sock.recvfrom(sizeof(Packet))  # Use ctypes.sizeof() to get the size of Packet

    # Cast the received data to the Packet structure
    packet = Packet.from_buffer_copy(data)

    # Print the fields
    print("Packet received:")
    print("  Type:", packet.header.type)
    print("  Sequence Number:", packet.header.seqNum)
    print("  Data Length:", packet.header.length)
    print("  Checksum:", packet.header.checksum)
    print("  Data:", packet.data.decode('utf-8'))
    print("  Data Length:", len(packet.data))
    calculated_crc = calculate_crc(packet.data)
    
    if calculated_crc != packet.header.checksum:
        print("Checksums do not match!")
        print(f"Calculated CRC32:   {calculated_crc}")
        print(f"Received CRC32:     {packet.header.checksum}")
    else:
        print("Checksums match!")
        header = PacketHeader(type=3, seqNum=packet.header.seqNum, length=0, checksum=0)
        ack_message = f"Received packet, seq={packet.header.seqNum}".encode('utf-8')
        data = ack_message + b"\0" * (1456 - len(ack_message))
        ack_packet = Packet(header=header, data=data)
        sock.sendto(ack_packet, addr)
        print("Packet sent successfully.")
        count += 1


print("Exiting after receiving the first ten packets.")
