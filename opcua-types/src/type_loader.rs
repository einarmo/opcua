use std::{borrow::Cow, io::Read, sync::Arc};

use chrono::TimeDelta;
use hashbrown::HashMap;

use crate::{BinaryDecodable, DecodingOptions, DynEncodable, EncodingResult, NamespaceMap, NodeId};

pub struct TypeLoaderInstance {
    binary_types:
        HashMap<u32, fn(&mut dyn Read, &Context<'_>) -> EncodingResult<Box<dyn DynEncodable>>>,

    #[cfg(feature = "xml")]
    xml_types: HashMap<
        u32,
        fn(
            &opcua_xml::XmlElement,
            &crate::xml::XmlContext<'_>,
        ) -> Result<Box<dyn DynEncodable>, crate::xml::FromXmlError>,
    >,

    #[cfg(feature = "json")]
    json_types: HashMap<
        u32,
        fn(
            &mut crate::json::JsonStreamReader<&mut dyn std::io::Read>,
            &Context<'_>,
        ) -> EncodingResult<Box<dyn DynEncodable>>,
    >,
}

pub fn binary_decode_to_enc<T: DynEncodable + BinaryDecodable>(
    stream: &mut dyn Read,
    ctx: &Context<'_>,
) -> EncodingResult<Box<dyn DynEncodable>> {
    Ok(Box::new(T::decode(stream, ctx)?))
}

#[cfg(feature = "json")]
pub fn json_decode_to_enc<T: DynEncodable + crate::json::JsonDecodable>(
    stream: &mut crate::json::JsonStreamReader<&mut dyn std::io::Read>,
    ctx: &Context<'_>,
) -> EncodingResult<Box<dyn DynEncodable>> {
    Ok(Box::new(T::decode(stream, ctx)?))
}

#[cfg(feature = "xml")]
pub fn xml_decode_to_enc<T: DynEncodable + crate::xml::FromXml>(
    body: &opcua_xml::XmlElement,
    ctx: &crate::xml::XmlContext<'_>,
) -> Result<Box<dyn DynEncodable>, crate::xml::FromXmlError> {
    Ok(Box::new(T::from_xml(body, ctx)?))
}

impl TypeLoaderInstance {
    pub fn new() -> Self {
        Self {
            binary_types: HashMap::new(),
            #[cfg(feature = "xml")]
            xml_types: HashMap::new(),
            #[cfg(feature = "json")]
            json_types: HashMap::new(),
        }
    }

    pub fn add_binary_type(
        &mut self,
        data_type: u32,
        encoding_type: u32,
        fun: fn(&mut dyn Read, &Context<'_>) -> EncodingResult<Box<dyn DynEncodable>>,
    ) {
        self.binary_types.insert(data_type, fun);
        self.binary_types.insert(encoding_type, fun);
    }

    #[cfg(feature = "xml")]
    pub fn add_xml_type(
        &mut self,
        data_type: u32,
        encoding_type: u32,
        fun: fn(
            &opcua_xml::XmlElement,
            &crate::xml::XmlContext<'_>,
        ) -> Result<Box<dyn DynEncodable>, crate::xml::FromXmlError>,
    ) {
        self.xml_types.insert(data_type, fun);
        self.xml_types.insert(encoding_type, fun);
    }

    #[cfg(feature = "json")]
    pub fn add_json_type(
        &mut self,
        data_type: u32,
        encoding_type: u32,
        fun: fn(
            &mut crate::json::JsonStreamReader<&mut dyn std::io::Read>,
            &Context<'_>,
        ) -> EncodingResult<Box<dyn DynEncodable>>,
    ) {
        self.json_types.insert(data_type, fun);
        self.json_types.insert(encoding_type, fun);
    }

    pub fn decode_binary(
        &self,
        ty: u32,
        stream: &mut dyn Read,
        context: &Context<'_>,
    ) -> Option<EncodingResult<Box<dyn DynEncodable>>> {
        let fun = self.binary_types.get(&ty)?;
        Some(fun(stream, context))
    }

    #[cfg(feature = "xml")]
    pub fn decode_xml(
        &self,
        ty: u32,
        body: &opcua_xml::XmlElement,
        context: &crate::xml::XmlContext<'_>,
    ) -> Option<Result<Box<dyn DynEncodable>, crate::xml::FromXmlError>> {
        let fun = self.xml_types.get(&ty)?;
        Some(fun(body, context))
    }

    #[cfg(feature = "json")]
    pub fn decode_json(
        &self,
        ty: u32,
        stream: &mut crate::json::JsonStreamReader<&mut dyn std::io::Read>,
        context: &Context<'_>,
    ) -> Option<EncodingResult<Box<dyn DynEncodable>>> {
        let fun = self.json_types.get(&ty)?;
        Some(fun(stream, context))
    }
}

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

    pub fn namespaces(&self) -> &NamespaceMap {
        &self.namespaces
    }

    pub fn options(&self) -> &DecodingOptions {
        &self.options
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

    pub fn with_zero_offset(&self) -> Cow<'_, Self> {
        if self.options.client_offset.is_zero() {
            Cow::Borrowed(self)
        } else {
            Cow::Owned(Self {
                namespaces: self.namespaces,
                loaders: self.loaders,
                options: DecodingOptions {
                    client_offset: TimeDelta::zero(),
                    ..self.options.clone()
                },
            })
        }
    }
}
