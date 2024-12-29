# Packet.py (Python Definition)
from ctypes import *

DATA_SIZE = 1456

class PacketHeader(Structure):
    _fields_ = [
        ("type", c_uint),
        ("seqNum", c_uint),
        ("length", c_uint),
        ("checksum", c_uint),
    ]


class Packet(Structure):
    _fields_ = [
        ("header", PacketHeader),
        ("data", c_char * DATA_SIZE),
    ]
