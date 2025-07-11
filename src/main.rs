use stratum_v2_mcp_server::StratumV2MCPServer;
use clap::{Parser, Subcommand, CommandFactory};
use serde_json::Value;

#[derive(Parser)]
#[command(name = "stratum-v2-mcp-server")]
#[command(about = "MCP server for working with Stratum V2 protocol specification")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze the Stratum V2 protocol specification
    AnalyzeProtocol,
    
    /// List all supported message types
    ListMessageTypes,
    
    /// List all supported extensions
    ListExtensions,
    
    /// Get detailed information about a specific extension
    GetExtensionInfo {
        #[arg(value_name = "EXTENSION_TYPE")]
        extension_type: u16,
    },
    
    /// Create a TLV field for a specific extension
    CreateTlvField {
        #[arg(value_name = "EXTENSION_TYPE")]
        extension_type: u16,
        #[arg(value_name = "FIELD_TYPE")]
        field_type: u8,
        #[arg(value_name = "VALUE")]
        value: String,
    },
    
    /// Parse TLV fields from a hex string
    ParseTlvFields {
        #[arg(value_name = "HEX_BYTES")]
        hex_bytes: String,
    },
    
    /// Validate a TLV field
    ValidateTlvField {
        #[arg(value_name = "EXTENSION_TYPE")]
        extension_type: u16,
        #[arg(value_name = "FIELD_TYPE")]
        field_type: u8,
        #[arg(value_name = "VALUE")]
        value: String,
    },
    
    /// Generate a test message
    GenerateTestMessage {
        #[arg(value_name = "MESSAGE_TYPE")]
        message_type: String,
    },
    
    /// Encode a JSON message to binary
    EncodeMessage {
        #[arg(value_name = "MESSAGE_TYPE")]
        message_type: String,
        #[arg(value_name = "JSON_MESSAGE")]
        json_message: String,
    },
    
    /// Decode binary data to JSON
    DecodeMessage {
        #[arg(value_name = "MESSAGE_TYPE")]
        message_type: String,
        #[arg(value_name = "HEX_BYTES")]
        hex_bytes: String,
    },

    /// Demonstrate advanced Stratum V2 features using official crates
    DemonstrateAdvancedFeatures,

    /// Demonstrate Stratum V2 roles architecture
    DemonstrateRolesLogic,

    /// Demonstrate Noise protocol security features
    DemonstrateNoiseProtocol,

    /// Demonstrate buffer management capabilities
    DemonstrateBufferManagement,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let server = StratumV2MCPServer::new();

    // CLI mode
    match &cli.command {
        Some(Commands::AnalyzeProtocol) => {
            let result = server.analyze_protocol_spec().await;
            println!("{}", result);
        },
        
        Some(Commands::ListMessageTypes) => {
            let result = server.list_message_types().await;
            println!("{}", result);
        },
        
        Some(Commands::ListExtensions) => {
            let result = server.list_extensions().await;
            println!("{}", result);
        },
        
        Some(Commands::GetExtensionInfo { extension_type }) => {
            let result = server.get_extension_info(*extension_type).await;
            println!("{}", result);
        },
        
        Some(Commands::CreateTlvField { extension_type, field_type, value }) => {
            let result = server.create_tlv_field(*extension_type, *field_type, value.clone()).await;
            println!("{}", result);
        },
        
        Some(Commands::ParseTlvFields { hex_bytes }) => {
            let bytes = hex::decode(hex_bytes)?;
            let result = server.parse_tlv_fields(bytes).await;
            println!("{}", result);
        },
        
        Some(Commands::ValidateTlvField { extension_type, field_type, value }) => {
            let result = server.validate_tlv_field(*extension_type, *field_type, value.clone()).await;
            println!("{}", result);
        },
        
        Some(Commands::GenerateTestMessage { message_type }) => {
            let result = server.generate_test_message(message_type.clone()).await;
            println!("{}", result);
        },
        
        Some(Commands::EncodeMessage { message_type, json_message }) => {
            let message: Value = serde_json::from_str(json_message)?;
            let result = server.encode_message(message, message_type.clone()).await;
            println!("{}", result);
        },
        
        Some(Commands::DecodeMessage { message_type, hex_bytes }) => {
            let bytes = hex::decode(hex_bytes)?;
            let result = server.decode_message(bytes, message_type.clone()).await;
            println!("{}", result);
        },

        Some(Commands::DemonstrateAdvancedFeatures) => {
            let result = server.demonstrate_advanced_features().await;
            println!("{}", result);
        },

        Some(Commands::DemonstrateRolesLogic) => {
            let result = server.demonstrate_roles_logic().await;
            println!("{}", result);
        },

        Some(Commands::DemonstrateNoiseProtocol) => {
            let result = server.demonstrate_noise_protocol().await;
            println!("{}", result);
        },

        Some(Commands::DemonstrateBufferManagement) => {
            let result = server.demonstrate_buffer_management().await;
            println!("{}", result);
        },

        None => {
            // Show help if no command provided
            let _ = Cli::command().print_help();
        }
    }

    Ok(())
} 