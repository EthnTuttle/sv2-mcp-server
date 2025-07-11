# Stratum V2 MCP Server for Claude Code

## Overview
This is a Model Context Protocol (MCP) server that provides comprehensive tools for working with the Stratum V2 mining protocol specification. It enables you to analyze protocol specifications, work with TLV fields, generate test messages, and explore advanced Stratum V2 features.

## Setup

### Building the Server
```bash
cargo build --release
```

### Testing CLI Mode
```bash
# Test CLI functionality
cargo run -- analyze-protocol
cargo run -- list-message-types
cargo run -- create-tlv-field 2 1 "worker123"
```

### MCP Server Mode
When run without arguments, the server operates in MCP mode and communicates via stdin/stdout:
```bash
cargo run
```

## Available Tools

### Protocol Analysis
- **analyze_protocol**: Get complete Stratum V2 protocol specification
- **list_message_types**: List all supported message types
- **list_extensions**: List all protocol extensions

### Extension Management
- **get_extension_info**: Get detailed info about a specific extension
- **create_tlv_field**: Create TLV (Type-Length-Value) fields
- **parse_tlv_fields**: Parse TLV fields from hex data
- **validate_tlv_field**: Validate TLV field format and constraints

### Message Operations
- **generate_test_message**: Create test messages for different types
  - Available types: SetupConnection, SubmitSharesStandard, NewTemplate, DeclareTransaction

### Advanced Features
- **demonstrate_advanced_features**: Show binary types, TLV structures, and protocol components
- **demonstrate_roles_logic**: Explain role-based architecture (Pool, Proxy, Miner, Job Declarator)  
- **demonstrate_noise_protocol**: Show security features and cryptographic primitives
- **demonstrate_buffer_management**: Explain buffer management capabilities

## Integration with Claude Code

Add this to your Claude Code configuration:

```json
{
  "mcpServers": {
    "stratum-v2": {
      "command": "cargo",
      "args": ["run", "--release", "--"],
      "cwd": "/absolute/path/to/sv2-mcp-server"
    }
  }
}
```

## Usage Examples

### Basic Protocol Analysis
Ask Claude: "What are the key components of the Stratum V2 protocol?"

### TLV Field Operations
Ask Claude: "Create a TLV field for worker identity tracking with the value 'miner001'"

### Message Generation
Ask Claude: "Generate a test SetupConnection message for a mining pool"

### Advanced Features
Ask Claude: "Explain the role-based architecture in Stratum V2"

## Features

- **Comprehensive Protocol Coverage**: All major Stratum V2 components
- **TLV Field Support**: Create, parse, and validate Type-Length-Value fields
- **Message Generation**: Generate test messages for development
- **Security Analysis**: Understand Noise protocol implementation
- **Role-Based Architecture**: Explore different mining roles and responsibilities
- **Binary Protocol Support**: Work with specialized binary data types

## Notes

- The server supports both CLI and MCP modes
- TLV fields are properly encoded using little-endian format
- Worker identity fields are limited to 32 bytes
- All test messages include descriptions of their usage
- Advanced features demonstrate integration with official Stratum V2 crates