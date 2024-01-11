# echo-server.py
import socket

HOST = "127.0.0.1"  # Standard loopback interface address (localhost)
PORT = 7878  # Port to listen on (non-privileged ports are > 1023)
# echo-client.py

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    s.sendall(b"Hello, world")
    data = s.recvmsg(1024)
    print(f"Received {data}")

print(f"Received {data!r}")
