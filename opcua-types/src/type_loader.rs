use std::{io::Read, sync::Arc};

use crate::{DecodingOptions, NamespaceMap, NodeId};

pub struct ContextOwned {
    namespaces: NamespaceMap,
    loaders: Vec<Arc<dyn TypeLoader>>,
    options: DecodingOptions,
}

impl ContextOwned {
    pub fn new(
        namespaces: NamespaceMap,
        loaders: Vec<Arc<dyn TypeLoader>>,
        options: DecodingOptions,
    ) -> Self {
        Self {
            namespaces,
            loaders,
            options,
        }
    }

    pub fn context(&self) -> Context<'_> {
        Context {
            namespaces: &self.namespaces,
            loaders: &self.loaders,
            options: self.options.clone(),
        }
    }
}

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
        node_id: &NodeId,
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
                return Ok(crate::ExtensionObject { body: Some(r?) });
            }
        }
        log::warn!("No type loader defined for {node_id}");
        Err(crate::StatusCode::BadDecodingError.into())
    }

    pub fn load_from_binary(
        &self,
        node_id: &NodeId,
        stream: &mut dyn Read,
        ctx: &Context<'_>,
    ) -> crate::EncodingResult<crate::ExtensionObject> {
        for loader in self.loaders {
            if let Some(r) = loader.load_from_binary(node_id, stream, ctx) {
                return Ok(crate::ExtensionObject { body: Some(r?) });
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
