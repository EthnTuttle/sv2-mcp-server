# MCP Integration Guide

This guide provides step-by-step instructions for integrating the Stratum V2 MCP server with [Blocks Goose](https://github.com/block/goose) and Anthropic's Claude/Cursor.

## Quick Start

### Prerequisites
- Rust 1.70+ installed
- Cargo package manager
- Access to [Blocks Goose](https://github.com/block/goose) or Cursor

### 1. Build the MCP Server

```bash
# Clone the repository
git clone <repository-url>
cd sv2-mcp-server

# Build the server
cargo build --release

# Verify the build
cargo run -- --help
```

## [Blocks Goose](https://github.com/block/goose) Integration

### Step 1: Install Goose

```bash
pip install goose-python
```

### Step 2: Configure Goose

Create a `goose-config.json` file in your project:

```json
{
  "mcpServers": {
    "stratum-v2": {
      "command": "cargo",
      "args": ["run", "--release"],
      "cwd": "/absolute/path/to/sv2-mcp-server"
    }
  }
}
```

### Step 3: Use in Python

```python
from goose import Goose

# Initialize Goose with MCP configuration
g = Goose(config_file="goose-config.json")

# Analyze Stratum V2 protocol
protocol_analysis = g.stratum_v2.analyze_protocol()
print("Protocol Analysis:", protocol_analysis)

# Create TLV field for worker tracking
tlv_result = g.stratum_v2.create_tlv_field(
    extension_type=2,
    field_type=1,
    value="worker123"
)
print("TLV Field:", tlv_result)

# Generate test mining message
message = g.stratum_v2.generate_test_message("SubmitSharesStandard")
print("Test Message:", message)

# Get extension information
extension_info = g.stratum_v2.get_extension_info(2)
print("Extension Info:", extension_info)

# Demonstrate advanced features
features = g.stratum_v2.demonstrate_advanced_features()
print("Advanced Features:", features)
```

### Step 4: Advanced Usage Examples

```python
# Validate TLV field
validation = g.stratum_v2.validate_tlv_field(
    extension_type=2,
    field_type=1,
    value="worker123"
)

# Parse TLV fields from hex
parsed = g.stratum_v2.parse_tlv_fields("0200010900776f726b6572313233")

# List all message types
message_types = g.stratum_v2.list_message_types()

# Demonstrate roles logic
roles = g.stratum_v2.demonstrate_roles_logic()

# Show noise protocol features
noise = g.stratum_v2.demonstrate_noise_protocol()
```

## Claude/Cursor Integration

### Step 1: Setup Cursor

1. Open Cursor
2. Navigate to your project directory
3. Create `cursor-mcp-config.json` in the project root

### Step 2: Configure MCP Server

Create `cursor-mcp-config.json`:

```json
{
  "mcpServers": {
    "stratum-v2": {
      "command": "cargo",
      "args": ["run", "--release"],
      "cwd": "/absolute/path/to/sv2-mcp-server"
    }
  }
}
```

### Step 3: Restart Cursor

Restart Cursor to load the MCP configuration.

### Step 4: Use with Claude

The MCP server is now available in Cursor. You can ask Claude questions like:

#### Protocol Analysis
```
"Analyze the Stratum V2 protocol specification and explain the key improvements over Stratum V1"
```

#### TLV Operations
```
"Create a TLV field for worker identity tracking in extension 0x0002 with the value 'miner001'"
```

#### Message Generation
```
"Generate a test SetupConnection message for a mining pool with protocol version 2.0.0"
```

#### Security Features
```
"Explain how the Noise protocol provides security in Stratum V2 and what cryptographic primitives are used"
```

#### Role-based Architecture
```
"Describe the role-based architecture in Stratum V2 and how it differs from traditional mining protocols"
```

#### Binary Protocol
```
"Show me how to work with the binary data types in Stratum V2 like U256 and B032"
```

## Example Workflows

### Mining Pool Development

```python
# Using Goose for mining pool development
g = Goose()

# 1. Analyze protocol requirements
protocol = g.stratum_v2.analyze_protocol()

# 2. Generate connection setup message
setup_msg = g.stratum_v2.generate_test_message("SetupConnection")

# 3. Create worker tracking TLV
worker_tlv = g.stratum_v2.create_tlv_field(2, 1, "pool_worker_001")

# 4. Generate share submission message
share_msg = g.stratum_v2.generate_test_message("SubmitSharesStandard")

# 5. Understand security features
security = g.stratum_v2.demonstrate_noise_protocol()
```

### Protocol Analysis

```python
# Comprehensive protocol analysis
g = Goose()

# Get complete protocol specification
spec = g.stratum_v2.analyze_protocol()

# List all message types
messages = g.stratum_v2.list_message_types()

# List all extensions
extensions = g.stratum_v2.list_extensions()

# Get detailed extension info
ext_info = g.stratum_v2.get_extension_info(2)

# Understand roles
roles = g.stratum_v2.demonstrate_roles_logic()

# Advanced features
features = g.stratum_v2.demonstrate_advanced_features()
```

### TLV Field Development

```python
# TLV field development workflow
g = Goose()

# 1. Create TLV field
tlv = g.stratum_v2.create_tlv_field(2, 1, "worker123")

# 2. Validate the field
validation = g.stratum_v2.validate_tlv_field(2, 1, "worker123")

# 3. Parse TLV fields from binary
parsed = g.stratum_v2.parse_tlv_fields(tlv["encoded_bytes"])

# 4. Get extension information
ext_info = g.stratum_v2.get_extension_info(2)
```

## Troubleshooting

### Common Issues

1. **Server not found**: Ensure the path in `cwd` is absolute and correct
2. **Build errors**: Run `cargo build --release` to ensure the server builds successfully
3. **Permission denied**: Make sure the binary has execute permissions
4. **Configuration not loaded**: Restart Cursor/Goose after changing configuration

### Debug Mode

Run the server in debug mode to see detailed output:

```bash
cargo run -- --help
cargo run analyze-protocol
```

### Logs

Check the console output for any error messages when the MCP server starts.

## API Reference

### Available Methods

| Method | Description | Parameters |
|--------|-------------|------------|
| `analyze_protocol()` | Complete protocol analysis | None |
| `list_message_types()` | List all message types | None |
| `list_extensions()` | List all extensions | None |
| `get_extension_info(ext_type)` | Get extension details | `ext_type: int` |
| `create_tlv_field(ext_type, field_type, value)` | Create TLV field | `ext_type: int, field_type: int, value: str` |
| `parse_tlv_fields(bytes)` | Parse TLV fields | `bytes: str` |
| `validate_tlv_field(ext_type, field_type, value)` | Validate TLV field | `ext_type: int, field_type: int, value: str` |
| `generate_test_message(msg_type)` | Generate test message | `msg_type: str` |
| `demonstrate_advanced_features()` | Show advanced features | None |
| `demonstrate_roles_logic()` | Show roles architecture | None |
| `demonstrate_noise_protocol()` | Show security features | None |
| `demonstrate_buffer_management()` | Show buffer management | None |

### Message Types

Available message types for `generate_test_message()`:

- `SetupConnection`
- `SubmitSharesStandard`
- `NewTemplate`
- `DeclareTransaction`

### Extension Types

- `1` - Extensions Negotiation
- `2` - Worker-Specific Hashrate Tracking

## Best Practices

1. **Use absolute paths** in MCP configuration
2. **Test the server** with CLI commands before MCP integration
3. **Handle errors** gracefully in your applications
4. **Validate inputs** before sending to the MCP server
5. **Use appropriate message types** for your use case
6. **Understand TLV field constraints** (e.g., worker names max 32 bytes)

## Support

For issues with MCP integration:

1. Check the troubleshooting section
2. Verify the server builds and runs standalone
3. Check the configuration file syntax
4. Review the API reference
5. Open an issue on GitHub with detailed error information 