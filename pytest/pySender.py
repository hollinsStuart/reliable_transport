import socket
from Packet import *  # Import Packet structure and PacketHeader
import zlib
import random

def calculate_crc(received_data):
    return zlib.crc32(received_data) & 0xffffffff


if __name__ == '__main__':
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

    server_address = ('localhost', 8888)  # Adjust IP and port as needed

    message = "This is a test message.".encode('utf-8')
    data = message + b"\0" * (DATA_SIZE - len(message))
    print(f"length: {len(message)}")
    header = PacketHeader(type=3, seqNum=random.randint(0, 10000), length=len(message), checksum=calculate_crc(message))
    print(f"crc: {header.checksum}")
    print(data.decode('utf-8'))
    print(f"length: {len(message)}")
    # Create the Packet and set the header and data
    packet = Packet(header=header, data=data)

    # Send the packet
    try:
        print("Sending packet...")
        sock.sendto(packet, server_address)
        print("Packet sent successfully.")
    finally:
        sock.close()
