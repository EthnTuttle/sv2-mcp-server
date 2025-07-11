use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// Type aliases for clarity in specifications
pub type MessageTypeEnum = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolSpec {
    pub version: String,
    pub description: String,
    pub message_types: Vec<MessageType>,
    pub extensions: Vec<ExtensionInfo>,
    pub security_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageType {
    pub name: String,
    pub direction: String,
    pub fields: Vec<MessageField>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageField {
    pub name: String,
    pub field_type: String,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub extension_type: u16,
    pub name: String,
    pub description: String,
    pub negotiation_required: bool,
    pub messages: Vec<String>,
    pub tlv_fields: Vec<TLVFieldInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLVFieldInfo {
    pub field_type: u8,
    pub name: String,
    pub data_type: String,
    pub max_length: Option<usize>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLVField {
    pub extension_type: u16,
    pub field_type: u8,
    pub length: u16,
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTLVFieldResponse {
    pub tlv_field: Option<TLVField>,
    pub encoded_bytes: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseTLVFieldsResponse {
    pub tlv_fields: Vec<TLVField>,
    pub errors: Vec<String>,
    pub parsed_successfully: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLVValidationResult {
    pub valid: bool,
    pub error: Option<String>,
    pub field_info: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateTLVFieldResponse {
    pub validation: TLVValidationResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExtensionInfoResponse {
    pub extension_info: Option<ExtensionInfo>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateTestMessageResponse {
    pub message: Option<Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEncodingResult {
    pub success: bool,
    pub encoded_bytes: Option<String>,
    pub decoded_message: Option<Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodeMessageResponse {
    pub result: MessageEncodingResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodeMessageResponse {
    pub result: MessageEncodingResult,
} 