// This file was autogenerated from schema/Opc.Ua.Pn.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Einar Omang
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
pub mod enums;
pub use enums::*;
pub mod structs;
pub use structs::*;
#[cfg(feature = "xml")]
#[derive(Debug, Default, Copy, Clone)]
pub struct TypesXmlLoader;
#[cfg(feature = "xml")]
impl opcua::types::xml::XmlLoader for TypesXmlLoader {
    fn load_extension_object(
        &self,
        body: &opcua::types::xml::XmlElement,
        node_id: &opcua::types::NodeId,
        ctx: &opcua::types::xml::XmlContext<'_>,
    ) -> Option<Result<opcua::types::ExtensionObject, opcua::types::xml::FromXmlError>> {
        use opcua::types::xml::FromXml;
        let idx = ctx
            .namespaces
            .namespaces()
            .get_index("http://opcfoundation.org/UA/PROFINET/")?;
        if idx != node_id.namespace {
            return None;
        }
        let object_id = match node_id
            .as_u32()
            .and_then(|v| crate::ObjectId::try_from(v).ok())
            .ok_or_else(|| format!("Invalid object ID: {node_id}"))
        {
            Ok(i) => i,
            Err(e) => return Some(Err(e.into())),
        };
        Some(match object_id {
            r @ crate::ObjectId::PnDeviceDiagnosisDataType_Encoding_DefaultXml => {
                PnDeviceDiagnosisDataType::from_xml(body, ctx)
                    .map(|v| opcua::types::ExtensionObject::from_encodable(r, &v))
            }
            r @ crate::ObjectId::PnDeviceRoleOptionSet_Encoding_DefaultXml => {
                PnDeviceRoleOptionSet::from_xml(body, ctx)
                    .map(|v| opcua::types::ExtensionObject::from_encodable(r, &v))
            }
            r @ crate::ObjectId::PnIM5DataType_Encoding_DefaultXml => {
                PnIM5DataType::from_xml(body, ctx)
                    .map(|v| opcua::types::ExtensionObject::from_encodable(r, &v))
            }
            _ => return None,
        })
    }
}
