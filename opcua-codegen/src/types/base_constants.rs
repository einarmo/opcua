use std::collections::{HashMap, HashSet};

pub fn make_import_lookup_map(inverted: HashMap<String, Vec<String>>) -> HashMap<String, String> {
    let mut res = HashMap::new();
    for (key, val) in inverted {
        for v in val {
            res.insert(v, key.clone());
        }
    }
    res
}

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

pub fn basic_types_import_map() -> HashMap<String, Vec<String>> {
    [
        ("string", vec!["UAString", "XmlElement"]),
        ("byte_string", vec!["ByteString"]),
        ("variant", vec!["Variant"]),
        ("guid", vec!["Guid"]),
        ("localized_text", vec!["LocalizedText"]),
        ("qualified_name", vec!["QualifiedName"]),
        ("diagnostic_info", vec!["DiagnosticInfo"]),
        ("extension_object", vec!["ExtensionObject"]),
        ("data_types", vec!["Duration", "UtcTime"]),
        ("request_header", vec!["RequestHeader"]),
        ("response_header", vec!["ResponseHeader"]),
        (
            "service_types::enums",
            vec![
                "MessageSecurityMode",
                "MonitoringMode",
                "TimestampsToReturn",
                "FilterOperator",
                "BrowseDirection",
                "NodeClass",
                "SecurityTokenRequestType",
                "ApplicationType",
                "UserTokenType",
                "DataChangeTrigger",
                "HistoryUpdateType",
                "PerformUpdateType",
                "ServerState",
                "AxisScaleEnumeration",
                "BrokerTransportQualityOfService",
                "JsonDataSetMessageContentMask",
                "JsonNetworkMessageContentMask",
                "DataSetFieldContentMask",
                "DataSetFieldFlags",
                "UadpDataSetMessageContentMask",
                "UadpNetworkMessageContentMask",
                "OverrideValueHandling",
                "DataSetOrderingType",
                "PermissionType",
                "StructureType",
                "IdentityCriteriaType",
            ],
        ),
        ("expanded_node_id", vec!["ExpandedNodeId"]),
        ("node_id", vec!["NodeId"]),
        ("data_value", vec!["DataValue"]),
        ("date_time", vec!["DateTime"]),
        ("status_codes", vec!["StatusCode"]),
    ]
    .into_iter()
    .map(|(k, v)| {
        (
            k.to_owned(),
            v.into_iter().map(|l| l.to_owned()).collect::<Vec<_>>(),
        )
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
