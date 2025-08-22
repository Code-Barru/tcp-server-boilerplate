# 🔐 Encrypted TCP Server-Client

A high-performance, secure TCP server-client implementation in Rust featuring AES-256-GCM encryption and Diffie-Hellman key exchange.

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Project Structure](#project-structure)
- [Quick Start](#quick-start)
- [How It Works](#how-it-works)
- [Protocol Documentation](#protocol-documentation)
- [Building and Running](#building-and-running)
- [Configuration](#configuration)
- [Security](#security)
- [Contributing](#contributing)

## 🚀 Overview

This project provides a secure TCP communication framework with automatic encryption setup. The server can handle multiple concurrent clients, and all communication is encrypted using industry-standard cryptographic protocols.

**Key Components:**
- **Server**: Async TCP server using Tokio
- **Agent**: Synchronous TCP client 
- **Shared**: Common encryption and packet handling utilities

## ✨ Features

- 🔒 **End-to-End Encryption**: AES-256-GCM encryption for all data transmission
- 🤝 **Secure Handshake**: X25519 Elliptic Curve Diffie-Hellman key exchange
- ⚡ **High Performance**: Async server supporting concurrent connections
- 🛡️ **Memory Safe**: Written in Rust with zero-copy optimizations
- 📦 **Modular Design**: Clean separation between server, client, and shared components
- 🔄 **Auto-Reconnection**: Client automatically reconnects on connection failure
- 📊 **Logging**: Comprehensive tracing and logging support

## 📁 Project Structure

```
├── agent/          # TCP Client implementation
├── server/         # TCP Server implementation  
├── shared/         # Common utilities and protocols
├── docs/           # Protocol documentation
│   ├── packets/    # Packet format specifications
│   └── protocols/  # Communication protocol docs
└── Cargo.toml      # Workspace configuration
```

### Crate Breakdown

- **`agent/`**: Synchronous TCP client with automatic connection management
- **`server/`**: Asynchronous TCP server using Tokio runtime
- **`shared/`**: Common encryption, packet serialization, and error handling

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (uses 2024 edition)
- Cargo

### Running the Server

```bash
# From project root
cargo run --bin server
```

The server will start listening on `0.0.0.0:1337` 🎯

### Running the Client

```bash
# In a separate terminal
cargo run --bin agent
```

The client will connect to `127.0.0.1:1337` and start sending encrypted messages! 📡

### Expected Output

**Server:**
```
INFO server: Server started on 0.0.0.0:1337
INFO server: Accepted connection from 127.0.0.1:xxxxx
INFO server: Received data from 127.0.0.1:xxxxx: [72, 101, 108, 108, 111, ...]
```

**Client:**
```
Connected to server successfully!
Received from server: [72, 101, 108, 108, 111, ...]
```

## 🔧 How It Works

### 1. Connection Establishment
The client initiates a TCP connection to the server.

### 2. Secure Handshake 🤝
Both parties perform a cryptographic handshake:

1. **Client → Server**: [Encryption Request](./docs/packets/0x01_encryption_request.md)
   - Client's X25519 public key
   - Random verification token

2. **Server → Client**: [Encryption Response](./docs/packets/0x02_encryption_response.md)  
   - Server's X25519 public key
   - Encrypted verification token
   - Nonce for decryption

3. **Key Derivation**: Both parties compute the shared secret using ECDH

### 3. Encrypted Communication 🔐
All subsequent messages are encrypted using AES-256-GCM:

```
[4-byte length][12-byte nonce][encrypted payload]
```

### 4. Message Flow
- Client sends encrypted "Hello from client!" messages
- Server echoes the decrypted data back, encrypted
- Connection gracefully closes after 5 messages

## 📚 Protocol Documentation

Detailed protocol specifications are available in the `docs/` directory:

- **[Handshake Protocol](./docs/protocols/handshake.md)** - Complete handshake sequence
- **[Encryption Request Packet](./docs/packets/0x01_encryption_request.md)** - Client's initial packet
- **[Encryption Response Packet](./docs/packets/0x02_encryption_response.md)** - Server's response packet

## 🛠️ Building and Running

### Development Build
```bash
cargo build
```

### Release Build  
```bash
cargo build --release
```

### Running Tests
```bash
cargo test
```

### Running with Logs
```bash
RUST_LOG=debug cargo run --bin server
```

## ⚙️ Configuration

### Server Configuration
- **Address**: `0.0.0.0:1337` (hardcoded in `server/src/main.rs`)
- **Log Level**: Configurable via `RUST_LOG` environment variable

### Client Configuration  
- **Server Address**: `127.0.0.1:1337` (hardcoded in `agent/src/main.rs`)
- **Reconnection**: 5-second delay between reconnection attempts
- **Message Count**: Sends 5 messages before closing

### Customization
To modify connection parameters, edit the respective `main.rs` files:

```rust
// Server - change bind address
let listener = TcpListener::bind("0.0.0.0:8080").await?;

// Client - change server address  
let mut client = Client::new("192.168.1.100:8080")?;
```

## 🔒 Security

### Cryptographic Primitives
- **Key Exchange**: X25519 Elliptic Curve Diffie-Hellman
- **Symmetric Encryption**: AES-256-GCM
- **Random Number Generation**: Cryptographically secure RNG

### Security Properties
- ✅ **Forward Secrecy**: New ephemeral keys for each connection
- ✅ **Authentication**: Verification token prevents MITM attacks  
- ✅ **Integrity**: GCM mode provides built-in authentication
- ✅ **Confidentiality**: AES-256 encryption protects data

### Security Considerations
- Keys are generated using cryptographically secure random number generators
- Verification tokens prevent replay attacks during handshake
- Connection state is properly cleaned up on termination
- No persistent key storage (ephemeral keys only)