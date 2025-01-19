TCP Chat Server
Definition: TCP (Transmission Control Protocol) is a connection-oriented protocol that ensures reliable, ordered, and error-checked delivery of data between clients and servers.
Usage in Flow:
1. Client Connections: Client A and Client B establish connections with the TCP chat server.
2. Message Exchange: Client A sends a message ("Hello, B!") to the server.
3. The server receives the message and forwards it to Client B.
4. Client B responds with "Hi, A!", which the server then sends back to Client A.
UDP Chat Server
Definition: UDP (User Datagram Protocol) is a connectionless protocol that allows for fast transmission of data without guarantees of delivery, order, or error correction.
Usage in Flow:
1. Message Sending: Client A sends a datagram ("Hello, everyone!") directly to the UDP chat server.
2 .Broadcasting Messages: The server broadcasts this datagram to all connected clients, who receive and display the message without any assurance that it was delivered successfully.
