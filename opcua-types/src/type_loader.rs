use std::{io::Read, sync::Arc};

use crate::{DecodingOptions, NamespaceMap, NodeId};

#[derive(Clone)]
pub struct Context<'a> {
    namespaces: &'a NamespaceMap,
    loaders: &'a [Arc<dyn TypeLoader>],
    options: DecodingOptions,
}

pub trait TypeLoader {
    #[cfg(feature = "xml")]
    fn load_from_xml(
        &self,
        node_id: &crate::NodeId,
        body: &opcua_xml::XmlElement,
        ctx: &crate::xml::XmlContext<'_>,
    ) -> Option<Result<Box<dyn crate::DynEncodable>, crate::xml::FromXmlError>>;

    #[cfg(feature = "json")]
    fn load_from_json(
        &self,
        node_id: &crate::NodeId,
        stream: &mut crate::json::JsonStreamReader<&mut dyn std::io::Read>,
        ctx: &Context<'_>,
    ) -> Option<crate::EncodingResult<Box<dyn crate::DynEncodable>>>;

    fn load_from_binary(
        &self,
        node_id: NodeId,
        stream: &mut dyn Read,
        ctx: &Context<'_>,
    ) -> Option<crate::EncodingResult<Box<dyn crate::DynEncodable>>>;
}

impl<'a> Context<'a> {
    #[cfg(feature = "json")]
    pub fn load_from_json(
        &self,
        node_id: &NodeId,
        stream: &mut crate::json::JsonStreamReader<&mut dyn Read>,
        ctx: &Context<'_>,
    ) -> crate::EncodingResult<crate::ExtensionObject> {
        for loader in self.loaders {
            if let Some(r) = loader.load_from_json(node_id, stream, ctx) {
                let r = r?;
                let node_id = r
                    .binary_type_id()
                    .try_resolve(&ctx.namespaces)
                    .ok_or(crate::StatusCode::BadDecodingError)?
                    .into_owned();
                let mut data = Vec::with_capacity(r.byte_len_dyn());
                let mut cursor = std::io::Cursor::new(&mut data);
                r.encode_dyn(&mut cursor)?;
                return Ok(crate::ExtensionObject {
                    node_id,
                    body: crate::ExtensionObjectEncoding::ByteString(crate::ByteString::from(data)),
                });
            }
        }
        log::warn!("No type loader defined for {node_id}");
        Err(crate::StatusCode::BadDecodingError.into())
    }

    pub fn options(&self) -> &DecodingOptions {
        &self.options
    }

    pub fn namespaces(&self) -> &'a NamespaceMap {
        self.namespaces
    }
}
