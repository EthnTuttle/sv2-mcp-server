# Claude Code Integration Guide

## Overview
This guide shows how to integrate the Stratum V2 MCP Server with Claude Code to provide Stratum V2 protocol analysis and development tools.

## Quick Setup

### 1. Build the Server
```bash
cd /Users/garykrause/repos/sv2-mcp-server
cargo build --release
```

### 2. Test CLI Mode (Optional)
```bash
# Test that the server works
cargo run -- analyze-protocol
cargo run -- list-message-types
cargo run -- create-tlv-field 2 1 "worker123"
```

### 3. Configure Claude Code

Add the following to your Claude Code MCP configuration:

```json
{
  "mcpServers": {
    "stratum-v2": {
      "command": "cargo",
      "args": ["run", "--release", "--"],
      "cwd": "/Users/garykrause/repos/sv2-mcp-server"
    }
  }
}
```

**Important**: Replace `/Users/garykrause/repos/sv2-mcp-server` with your actual project path.

### 4. Restart Claude Code
Restart Claude Code for the configuration to take effect.

## Available Tools

Once integrated, you can ask Claude to use these tools:

### Protocol Analysis
- **analyze_protocol**: Complete Stratum V2 protocol specification
- **list_message_types**: All supported message types
- **list_extensions**: Protocol extensions and their capabilities

### TLV Field Operations
- **create_tlv_field**: Create Type-Length-Value fields for extensions
- **parse_tlv_fields**: Parse TLV fields from hex data
- **validate_tlv_field**: Validate TLV field format and constraints

### Message Generation
- **generate_test_message**: Create test messages for:
  - SetupConnection
  - SubmitSharesStandard
  - NewTemplate
  - DeclareTransaction

### Advanced Features
- **demonstrate_advanced_features**: Binary types, TLV structures, protocol components
- **demonstrate_roles_logic**: Role-based architecture (Pool, Proxy, Miner, Job Declarator)
- **demonstrate_noise_protocol**: Security features and cryptographic primitives
- **demonstrate_buffer_management**: Buffer management capabilities

## Usage Examples

### Basic Protocol Analysis
**Ask Claude**: "What are the key components of the Stratum V2 protocol?"

Claude will use the `analyze_protocol` tool to provide comprehensive protocol information.

### TLV Field Operations
**Ask Claude**: "Create a TLV field for worker identity tracking with the value 'miner001'"

Claude will use the `create_tlv_field` tool with:
- extension_type: 2 (Worker-Specific Hashrate Tracking)
- field_type: 1 (user_identity)
- value: "miner001"

### Message Generation
**Ask Claude**: "Generate a test SetupConnection message for a mining pool"

Claude will use the `generate_test_message` tool to create a properly formatted message.

### Advanced Analysis
**Ask Claude**: "Explain the role-based architecture in Stratum V2 and how it differs from Stratum V1"

Claude will use the `demonstrate_roles_logic` tool to explain the different mining roles.

## Troubleshooting

### Server Not Found
If Claude Code can't find the server:
1. Verify the `cwd` path is correct and absolute
2. Ensure the project builds successfully with `cargo build --release`
3. Check that Cargo is in your PATH

### Permission Issues
If you get permission errors:
1. Ensure the project directory is readable
2. Try running `cargo build --release` manually to verify permissions

### No Tools Available
If Claude says no tools are available:
1. Restart Claude Code completely
2. Check the MCP configuration syntax
3. Verify the server starts in MCP mode (no CLI arguments)

## Features

### Comprehensive Protocol Coverage
- All major Stratum V2 message types
- Extension system with TLV fields
- Role-based architecture analysis
- Security features documentation

### Development Tools
- Test message generation
- TLV field creation and validation
- Binary protocol analysis
- Buffer management insights

### Real-World Integration
- Uses official Stratum V2 crates
- Proper binary encoding formats
- Industry-standard cryptographic primitives
- Production-ready protocol components

## Notes

- The server automatically detects MCP mode vs CLI mode
- All TLV fields use proper little-endian encoding
- Worker identity fields are limited to 32 bytes (per specification)
- Test messages include usage descriptions
- Advanced features demonstrate integration with official SV2 ecosystem