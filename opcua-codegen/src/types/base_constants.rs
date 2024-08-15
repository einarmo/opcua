use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

pub fn base_ignored_types() -> HashSet<String> {
    [
        "ExtensionObject",
        "DataValue",
        "LocalizedText",
        "QualifiedName",
        "DiagnosticInfo",
        "Variant",
        "ExpandedNodeId",
        "NodeId",
        "ByteStringNodeId",
        "GuidNodeId",
        "StringNodeId",
        "NumericNodeId",
        "FourByteNodeId",
        "TwoByteNodeId",
        "XmlElement",
        "Union",
        "RequestHeader",
        "ResponseHeader",
        "Node",
        "InstanceNode",
        "TypeNode",
        "ObjectNode",
        "ObjectTypeNode",
        "VariableNode",
        "VariableTypeNode",
        "ReferenceTypeNode",
        "MethodNode",
        "ViewNode",
        "DataTypeNode",
        "ReferenceNode",
    ]
    .into_iter()
    .map(|v| v.to_owned())
    .collect()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalType {
    /// Relative path in the OPC-UA types library.
    pub path: String,
    /// Whether this type has a default implementation.
    pub has_default: Option<bool>,
}

impl ExternalType {
    pub fn new(path: &str, has_default: bool) -> Self {
        Self {
            path: path.to_owned(),
            has_default: Some(has_default),
        }
    }
}

pub fn basic_types_import_map(root: &str) -> HashMap<String, ExternalType> {
    [
        ("UAString", ExternalType::new("types::string", true)),
        ("ByteString", ExternalType::new("types::byte_string", true)),
        ("XmlElement", ExternalType::new("types::string", true)),
        ("Variant", ExternalType::new("types::variant", true)),
        ("Guid", ExternalType::new("types::guid", true)),
        (
            "LocalizedText",
            ExternalType::new("types::localized_text", true),
        ),
        (
            "QualifiedName",
            ExternalType::new("types::qualified_name", true),
        ),
        (
            "DiagnosticInfo",
            ExternalType::new("types::diagnostic_info", true),
        ),
        (
            "ExtensionObject",
            ExternalType::new("types::extension_object", true),
        ),
        ("Duration", ExternalType::new("types::data_types", true)),
        ("UtcTime", ExternalType::new("types::data_types", true)),
        (
            "RequestHeader",
            ExternalType::new("types::request_header", true),
        ),
        (
            "ResponseHeader",
            ExternalType::new("types::response_header", true),
        ),
        (
            "ExpandedNodeId",
            ExternalType::new("types::expanded_node_id", true),
        ),
        ("NodeId", ExternalType::new("types::node_id", true)),
        ("DataValue", ExternalType::new("types::data_value", true)),
        ("DateTime", ExternalType::new("types::date_time", true)),
        ("StatusCode", ExternalType::new("types::status_code", true)),
    ]
    .into_iter()
    .map(|(k, mut v)| {
        v.path = format!("{}::{}", root, v.path);
        (k.to_owned(), v)
    })
    .collect()
}

pub fn base_json_serialized_types() -> HashSet<String> {
    [
        "ReadValueId",
        "DataChangeFilter",
        "EventFilter",
        "SimpleAttributeOperand",
        "ContentFilter",
        "ContentFilterElement",
        "MonitoredItemNotification",
        "ServerDiagnosticsSummaryDataType",
        "EventFieldList",
        "DataChangeTrigger",
        "FilterOperator",
        "TimestampsToReturn",
        "MonitoringMode",
        "ConfigurationVersionDataType",
        "DataSetMetaDataType",
        "StructureDescription",
        "EnumDescription",
        "SimpleTypeDescription",
        "StructureDefinition",
        "EnumDefinition",
        "FieldMetaData",
        "KeyValuePair",
        "DataSetFieldFlags",
        "StructureType",
        "StructureField",
        "EnumField",
    ]
    .into_iter()
    .map(|v| v.to_owned())
    .collect()
}

pub fn base_native_type_mappings() -> HashMap<String, String> {
    [
        ("String", "UAString"),
        ("Boolean", "bool"),
        ("SByte", "i8"),
        ("Byte", "u8"),
        ("Int16", "i16"),
        ("UInt16", "u16"),
        ("Int32", "i32"),
        ("UInt32", "u32"),
        ("Int64", "i64"),
        ("UInt64", "u64"),
        ("Float", "f32"),
        ("Double", "f64"),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_owned(), v.to_owned()))
    .collect()
}
