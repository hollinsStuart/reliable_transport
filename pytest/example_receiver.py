import socket
from Packet import Packet  # Import the Packet struct definition
from ctypes import *  # Import the sizeof function


if __name__ == '__main__':
    # Set up the UDP socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(('localhost', 8888))  # Use the appropriate IP and port

    while true:
        # Receive the packet
        data, addr = sock.recvfrom(sizeof(Packet))  # Ensure the buffer size matches the struct size

        # Cast the received data to the Packet structure
        packet = Packet.from_buffer_copy(data)

        # Access the fields
        print("Packet Type:", packet.header.type)
        print("Sequence Number:", packet.header.seqNum)
        print("Data Length:", packet.header.length)
        print("Checksum:", packet.header.checksum)
        print("Data:", packet.data[:packet.header.length].decode('utf-8'))
        print("--------")
