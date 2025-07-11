# Stratum V2 MCP Server

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server for working with the [Stratum V2](https://github.com/stratum-mining/stratum) mining protocol specification. This tool provides deep insights into Bitcoin mining protocols, TLV extensions, message structures, and the complete Stratum V2 ecosystem.

## 🚀 Features

- **🔍 Protocol Analysis**: Detailed analysis of Stratum V2 protocol specification
- **🏷️ TLV Field Operations**: Create, parse, and validate Type-Length-Value extension fields
- **📨 Message Generation**: Generate test messages for all major Stratum V2 message types
- **🔢 Binary Protocol Support**: Handle specialized Bitcoin mining data types (U256, U24, B032, etc.)
- **👥 Role-based Architecture**: Understand Pool, Proxy, Mining Device, and Job Declarator roles
- **🔐 Security Features**: Noise protocol encryption, cryptographic handshakes
- **📡 Multiple Subprotocols**: Mining, Job Declaration, and Template Distribution protocols
- **📦 Comprehensive Crate Integration**: Utilizes all 19 official Stratum V2 Rust crates

## 📋 Table of Contents

- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Usage](#-usage)
- [MCP Integration](#-mcp-integration)
- [API Reference](#-api-reference)
- [Examples](#-examples)
- [Architecture](#-architecture)
- [Contributing](#-contributing)
- [License](#-license)

## 🛠️ Installation

### Prerequisites

- **Rust**: 1.70+ ([Install Rust](https://rustup.rs/))
- **Cargo**: Included with Rust installation

### From Source

```bash
# Clone the repository
git clone https://github.com/average-gary/sv2-mcp-server.git
cd sv2-mcp-server

# Build the project
cargo build --release

# Verify installation
cargo run -- --help
```

## 🚀 Quick Start

### Basic Protocol Analysis

```bash
# Analyze the complete Stratum V2 protocol specification
cargo run -- analyze-protocol

# List all supported message types
cargo run -- list-message-types

# List all protocol extensions
cargo run -- list-extensions
```

### TLV Field Operations

```bash
# Create a TLV field for worker identity tracking
cargo run -- create-tlv-field 2 1 "worker123"

# Parse TLV fields from hex bytes
cargo run -- parse-tlv-fields "0200010900776f726b6572313233"
```

### Message Generation

```bash
# Generate test messages for different protocols
cargo run -- generate-test-message "SubmitSharesStandard"
cargo run -- generate-test-message "SetupConnection"
cargo run -- generate-test-message "NewTemplate"
```

## 📖 Usage

### Command Line Interface

The server provides a comprehensive CLI for direct interaction with Stratum V2 protocol features.

#### Protocol Analysis Commands

```bash
# Complete protocol specification analysis
cargo run -- analyze-protocol

# List all supported message types
cargo run -- list-message-types

# List all protocol extensions
cargo run -- list-extensions

# Get detailed information about a specific extension
cargo run -- get-extension-info 2
```

#### TLV Field Operations

```bash
# Create a TLV field for worker identity tracking
cargo run -- create-tlv-field 2 1 "worker123"

# Validate a TLV field
cargo run -- validate-tlv-field 2 1 "worker123"

# Parse TLV fields from hex bytes
cargo run -- parse-tlv-fields "0200010900776f726b6572313233"
```

#### Message Operations

```bash
# Generate test messages for different protocols
cargo run -- generate-test-message "SubmitSharesStandard"
cargo run -- generate-test-message "SetupConnection"
cargo run -- generate-test-message "NewTemplate"
cargo run -- generate-test-message "DeclareTransaction"

# Encode a JSON message to binary
cargo run -- encode-message "SubmitSharesStandard" '{"channel_id": 1, "sequence_number": 1}'

# Decode binary data to JSON
cargo run -- decode-message "SubmitSharesStandard" "0100000001000000"
```

#### Advanced Demonstrations

```bash
# Comprehensive feature demonstration
cargo run -- demonstrate-advanced-features

# Roles architecture explanation
cargo run -- demonstrate-roles-logic

# Noise protocol security features
cargo run -- demonstrate-noise-protocol

# Buffer management capabilities
cargo run -- demonstrate-buffer-management
```

## 🔌 MCP Integration

This server integrates with [Model Context Protocol](https://modelcontextprotocol.io/) clients like [Blocks Goose](https://github.com/block/goose) and [Cursor](https://cursor.sh/).

### Quick Setup

1. **Build the server**:
   ```bash
   cargo build --release
   ```

2. **Configure MCP**:
   Copy `example-config.json` and update the path:
   ```json
   {
     "mcpServers": {
       "stratum-v2": {
         "command": "cargo",
         "args": ["run", "--release"],
         "cwd": "/path/to/your/sv2-mcp-server"
       }
     }
   }
   ```

3. **Use with Goose**:
   ```python
   from goose import Goose
   g = Goose(config_file="goose-config.json")
   
   # Analyze protocol
   protocol = g.stratum_v2.analyze_protocol()
   
   # Create TLV field
   tlv = g.stratum_v2.create_tlv_field(2, 1, "worker123")
   ```

4. **Use with Cursor**:
   - Create `cursor-mcp-config.json` with the same configuration
   - Restart Cursor
   - Ask Claude about Stratum V2 protocol features

For detailed integration instructions, troubleshooting, and advanced usage examples, see [MCP_INTEGRATION.md](MCP_INTEGRATION.md).

## 📚 API Reference

### Core Methods

#### Protocol Analysis
- `analyze_protocol()` - Complete protocol specification analysis
- `list_message_types()` - All supported message types
- `list_extensions()` - Available protocol extensions
- `get_extension_info(extension_type)` - Detailed extension information

#### TLV Operations
- `create_tlv_field(extension_type, field_type, value)` - Create TLV field
- `parse_tlv_fields(bytes)` - Parse TLV fields from binary data
- `validate_tlv_field(extension_type, field_type, value)` - Validate TLV field

#### Message Operations
- `generate_test_message(message_type)` - Generate test message
- `encode_message(message, message_type)` - Encode message to binary
- `decode_message(bytes, message_type)` - Decode binary to message

#### Demonstrations
- `demonstrate_advanced_features()` - Comprehensive feature showcase
- `demonstrate_roles_logic()` - Role-based architecture explanation
- `demonstrate_noise_protocol()` - Security features demonstration
- `demonstrate_buffer_management()` - Buffer management capabilities

### Message Types

#### Mining Protocol
- `SetupConnection` - Initial connection setup
- `SubmitSharesStandard` - Standard share submission
- `SubmitSharesExtended` - Extended share submission with TLV
- `OpenStandardMiningChannel` - Open mining channel
- `SetTarget` - Set mining target
- `SetCustomMiningJob` - Set custom mining job

#### Job Declaration Protocol
- `DeclareTransaction` - Declare transaction for mining
- `DeclareTransactionSuccess` - Transaction declaration success
- `DeclareTransactionError` - Transaction declaration error

#### Template Distribution Protocol
- `NewTemplate` - New block template
- `SetNewPrevHash` - Set new previous hash
- `SetNewPrevHashSuccess` - Previous hash update success

### Extension Types

- `0x0001` - Extensions Negotiation
- `0x0002` - Worker-Specific Hashrate Tracking

## 💡 Examples

### Worker Identity Tracking

```bash
# Create worker identity TLV field
cargo run -- create-tlv-field 2 1 "worker123"
```

**Output:**
```json
{
  "tlv_field": {
    "extension_type": 2,
    "field_type": 1,
    "length": 9,
    "value": [119, 111, 114, 107, 101, 114, 49, 50, 51]
  },
  "encoded_bytes": "0200010900776f726b6572313233",
  "error": null
}
```

### Protocol Analysis

```bash
cargo run -- analyze-protocol
```

**Output includes:**
- Complete protocol specification
- Message type definitions
- Extension information
- Security features
- Binary protocol structure

### Advanced Features Demonstration

```bash
cargo run -- demonstrate-advanced-features
```

**Output includes:**
- Binary types (U256, U24, B032, etc.)
- TLV extension system
- Noise protocol security
- Role-based architecture
- Multiple subprotocols
- Message framing
- All 19 available Stratum V2 crates

## 🏗️ Architecture

### Stratum V2 Ecosystem Integration

This MCP server integrates with the complete Stratum V2 ecosystem:

#### Core Protocol Crates
- `binary_sv2` - Binary data types
- `binary_codec_sv2` - Binary encoding/decoding
- `codec_sv2` - High-level codec with encryption
- `framing_sv2` - Message framing
- `noise_sv2` - Noise protocol encryption
- `buffer_sv2` - Buffer management
- `error_handling` - Error utilities

#### Subprotocol Crates
- `common_messages_sv2` - Common message types
- `mining_sv2` - Mining protocol
- `job_declaration_sv2` - Job declaration protocol
- `template_distribution_sv2` - Template distribution
- `roles_logic_sv2` - Role implementations

#### Network Utilities
- `network_helpers_sv2` - Network utilities

### Security Features

#### Noise Protocol
- ChaCha20-Poly1305 AEAD encryption
- X25519 elliptic curve Diffie-Hellman
- BLAKE2s cryptographic hash function
- Forward secrecy
- Mutual authentication
- Resistance to man-in-the-middle attacks

#### Binary Protocol
- Efficient message encoding
- Length-prefixed messages
- Message integrity checks
- TLV extension support

### Role-based Architecture

#### Pool Server
- Coordinates mining operations
- Manages mining channels
- Distributes work to miners
- Validates submitted shares
- Handles payouts

#### Mining Proxy
- Aggregates multiple miners
- Reduces network overhead
- Provides redundancy
- Optimizes communication

#### Mining Device
- Performs proof-of-work calculations
- Submits valid shares
- Reports hashrate statistics
- Handles job updates

#### Job Declarator
- Selects transactions for blocks
- Constructs coinbase transactions
- Distributes templates to pools
- Manages transaction selection

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/average-gary/sv2-mcp-server.git
cd sv2-mcp-server

# Install dependencies
cargo build

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt
```

### Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Ensure all tests pass (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

### Getting Help

- **Documentation**: Check this README and [MCP_INTEGRATION.md](MCP_INTEGRATION.md)
- **Issues**: [Open an issue](https://github.com/average-gary/sv2-mcp-server/issues) on GitHub
- **Discussions**: [Start a discussion](https://github.com/average-gary/sv2-mcp-server/discussions) for questions and ideas

### Common Issues

- **Build errors**: Ensure you have Rust 1.70+ installed
- **MCP integration issues**: Check the [MCP Integration Guide](MCP_INTEGRATION.md)
- **Protocol questions**: Review the [Stratum V2 Specification](https://stratumprotocol.org)

## 🙏 Acknowledgments

- [Stratum v2 Reference Implementation](https://github.com/stratum-mining/stratum) for the Rust reference implementation of Stratum v2
- [Model Context Protocol](https://modelcontextprotocol.io/) for the MCP standard
- [Blocks Goose](https://github.com/block/goose) for MCP client implementation
- [Cursor](https://cursor.sh/) for AI-powered development environment

## 📚 References

- [Stratum V2 Specification](https://github.com/stratum-mining/stratum)
- [Model Context Protocol](https://modelcontextprotocol.io/)
- [Blocks Goose](https://github.com/block/goose)
- [Cursor](https://cursor.sh/)
- [Rust Programming Language](https://www.rust-lang.org/)

---

**Made with ❤️ for the Bitcoin mining community** 