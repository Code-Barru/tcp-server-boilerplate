## Handshake

The connection sequence between Agent & Server

A->S - Agent to Server

S->A - Server to Agent

1. A->S [Encryption Request](../packets/0x01_encryption_request.md)
2. S->A [Encryption Response](../packets/0x02_encryption_response.md)

The connection is now encrypted using [AES-GCM](https://en.wikipedia.org/wiki/Galois/Counter_Mode)
