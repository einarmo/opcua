use std::{
    collections::HashMap,
    io::{Cursor, Read, Write},
    sync::Arc,
};

use opcua_types::{
    json::{
        consume_raw_value, JsonDecodable, JsonEncodable, JsonReader, JsonStreamReader,
        JsonStreamWriter, JsonWriter,
    },
    Array, ByteString, Context, DataValue, DateTime, DiagnosticInfo, DynEncodable, EncodingResult,
    Error, ExpandedNodeId, ExtensionObject, Guid, LocalizedText, NodeId, QualifiedName, StatusCode,
    UAString, Variant, XmlElement,
};

use super::{
    custom_struct::{DynamicStructure, DynamicTypeLoader},
    type_tree::{ParsedStructureField, StructTypeInfo},
};

impl DynamicStructure {
    fn json_encode_array(
        &self,
        stream: &mut JsonStreamWriter<&mut dyn Write>,
        field: &ParsedStructureField,
        ctx: &Context<'_>,
        items: &[Variant],
        remaining_dims: &[u32],
        index: &mut usize,
    ) -> EncodingResult<()> {
        if remaining_dims.len() == 1 {
            stream.begin_array()?;
            for _ in 0..remaining_dims[0] {
                self.json_encode_field(
                    stream,
                    items.get(*index).unwrap_or(&Variant::Empty),
                    field,
                    ctx,
                )?;
                *index += 1;
            }
            stream.end_array()?;
        } else {
            stream.begin_array()?;
            for _ in 0..remaining_dims[0] {
                self.json_encode_array(stream, field, ctx, items, remaining_dims, index)?;
            }
            stream.end_array()?;
        }

        Ok(())
    }

    fn json_encode_field(
        &self,
        stream: &mut JsonStreamWriter<&mut dyn Write>,
        f: &Variant,
        field: &ParsedStructureField,
        ctx: &Context<'_>,
    ) -> EncodingResult<()> {
        match f {
            Variant::ExtensionObject(o) => {
                let Some(field_ty) = self.type_tree.get_struct_type(&field.type_id) else {
                    return Err(Error::encoding(format!(
                        "Dynamic type field missing from type tree: {}",
                        field.type_id
                    )));
                };
                if field_ty.is_abstract {
                    o.encode(stream, ctx)
                } else {
                    let Some(body) = &o.body else {
                        return Err(Error::encoding(
                            "Dynamic type field is missing extension object body",
                        ));
                    };
                    body.encode_json(stream, ctx)
                }
            }
            Variant::Array(a) => {
                if field.value_rank > 1 {
                    let Some(dims) = &a.dimensions else {
                        return Err(Error::encoding(
                            "ArrayDimensions are required for fields with value rank > 1",
                        ));
                    };
                    if dims.len() as i32 != field.value_rank {
                        return Err(Error::encoding(
                            "ArrayDimensions must have length equal to field valuerank",
                        ));
                    }
                    let mut index = 0;
                    self.json_encode_array(stream, field, ctx, &a.values, &dims, &mut index)?;
                } else {
                    stream.begin_array()?;
                    for value in a.values.iter() {
                        self.json_encode_field(stream, value, field, ctx)?;
                    }
                    stream.end_array()?;
                }

                Ok(())
            }
            r => r.serialize_variant_value(stream, ctx),
        }
    }
}

impl DynamicTypeLoader {
    fn json_decode_field_value(
        &self,
        field: &ParsedStructureField,
        stream: &mut opcua_types::json::JsonStreamReader<&mut dyn std::io::Read>,
        ctx: &opcua_types::Context<'_>,
    ) -> EncodingResult<Variant> {
        match field.scalar_type {
            opcua_types::VariantScalarTypeId::Boolean => {
                Ok(Variant::from(<bool as JsonDecodable>::decode(stream, ctx)?))
            }
            opcua_types::VariantScalarTypeId::SByte => {
                Ok(Variant::from(<i8 as JsonDecodable>::decode(stream, ctx)?))
            }
            opcua_types::VariantScalarTypeId::Byte => {
                Ok(Variant::from(<u8 as JsonDecodable>::decode(stream, ctx)?))
            }
            opcua_types::VariantScalarTypeId::Int16 => {
                Ok(Variant::from(<i16 as JsonDecodable>::decode(stream, ctx)?))
            }
            opcua_types::VariantScalarTypeId::UInt16 => {
                Ok(Variant::from(<u16 as JsonDecodable>::decode(stream, ctx)?))
            }
            opcua_types::VariantScalarTypeId::Int32 => {
                Ok(Variant::from(<i32 as JsonDecodable>::decode(stream, ctx)?))
            }
            opcua_types::VariantScalarTypeId::UInt32 => {
                Ok(Variant::from(<u32 as JsonDecodable>::decode(stream, ctx)?))
            }
            opcua_types::VariantScalarTypeId::Int64 => {
                Ok(Variant::from(<i64 as JsonDecodable>::decode(stream, ctx)?))
            }
            opcua_types::VariantScalarTypeId::UInt64 => {
                Ok(Variant::from(<u64 as JsonDecodable>::decode(stream, ctx)?))
            }
            opcua_types::VariantScalarTypeId::Float => {
                Ok(Variant::from(<f32 as JsonDecodable>::decode(stream, ctx)?))
            }
            opcua_types::VariantScalarTypeId::Double => {
                Ok(Variant::from(<f64 as JsonDecodable>::decode(stream, ctx)?))
            }
            opcua_types::VariantScalarTypeId::String => Ok(Variant::from(
                <UAString as JsonDecodable>::decode(stream, ctx)?,
            )),
            opcua_types::VariantScalarTypeId::DateTime => Ok(Variant::from(
                <DateTime as JsonDecodable>::decode(stream, ctx)?,
            )),
            opcua_types::VariantScalarTypeId::Guid => {
                Ok(Variant::from(<Guid as JsonDecodable>::decode(stream, ctx)?))
            }
            opcua_types::VariantScalarTypeId::ByteString => Ok(Variant::from(
                <ByteString as JsonDecodable>::decode(stream, ctx)?,
            )),
            opcua_types::VariantScalarTypeId::XmlElement => Ok(Variant::from(
                <XmlElement as JsonDecodable>::decode(stream, ctx)?,
            )),
            opcua_types::VariantScalarTypeId::NodeId => Ok(Variant::from(
                <NodeId as JsonDecodable>::decode(stream, ctx)?,
            )),
            opcua_types::VariantScalarTypeId::ExpandedNodeId => Ok(Variant::from(
                <ExpandedNodeId as JsonDecodable>::decode(stream, ctx)?,
            )),
            opcua_types::VariantScalarTypeId::StatusCode => Ok(Variant::from(
                <StatusCode as JsonDecodable>::decode(stream, ctx)?,
            )),
            opcua_types::VariantScalarTypeId::QualifiedName => Ok(Variant::from(
                <QualifiedName as JsonDecodable>::decode(stream, ctx)?,
            )),
            opcua_types::VariantScalarTypeId::LocalizedText => Ok(Variant::from(
                <LocalizedText as JsonDecodable>::decode(stream, ctx)?,
            )),
            opcua_types::VariantScalarTypeId::ExtensionObject => {
                let Some(field_ty) = self.type_tree.get_struct_type(&field.type_id) else {
                    return Err(Error::decoding(format!(
                        "Dynamic type field missing from type tree: {}",
                        field.type_id
                    )));
                };

                if field_ty.is_abstract {
                    Ok(Variant::from(<ExtensionObject as JsonDecodable>::decode(
                        stream, ctx,
                    )?))
                } else {
                    Ok(Variant::from(ctx.load_from_json(
                        &field_ty.node_id,
                        stream,
                        ctx,
                    )?))
                }
            }
            opcua_types::VariantScalarTypeId::DataValue => Ok(Variant::from(
                <DataValue as JsonDecodable>::decode(stream, ctx)?,
            )),
            opcua_types::VariantScalarTypeId::Variant => Ok(Variant::Variant(Box::new(
                <Variant as JsonDecodable>::decode(stream, ctx)?,
            ))),
            opcua_types::VariantScalarTypeId::DiagnosticInfo => Ok(Variant::from(
                <DiagnosticInfo as JsonDecodable>::decode(stream, ctx)?,
            )),
        }
    }

    fn json_decode_array(
        &self,
        field: &ParsedStructureField,
        stream: &mut opcua_types::json::JsonStreamReader<&mut dyn std::io::Read>,
        ctx: &Context<'_>,
        value_rank: i32,
        depth: i32,
        values: &mut Vec<Variant>,
        dims: &mut Vec<u32>,
    ) -> EncodingResult<()> {
        let mut size = 0;
        stream.begin_array()?;
        if value_rank > depth {
            while stream.has_next()? {
                size += 1;
                self.json_decode_array(field, stream, ctx, value_rank, depth, values, dims)?;
            }
        } else {
            while stream.has_next()? {
                size += 1;
                values.push(self.json_decode_field_value(field, stream, ctx)?);
            }
        }
        let old_dim = dims[depth as usize];
        if old_dim > 0 && size != old_dim {
            return Err(Error::decoding(format!(
                "JSON matrix in field {} does not have even dimensions",
                field.name
            )));
        } else if old_dim == 0 {
            dims[depth as usize] = size;
        }
        stream.end_array()?;

        Ok(())
    }

    fn json_decode_field(
        &self,
        field: &ParsedStructureField,
        stream: &mut opcua_types::json::JsonStreamReader<&mut dyn std::io::Read>,
        ctx: &Context<'_>,
    ) -> EncodingResult<Variant> {
        if field.value_rank > 0 {
            let mut values = Vec::new();
            let mut dims = vec![0u32; field.value_rank as usize];
            self.json_decode_array(
                field,
                stream,
                ctx,
                field.value_rank,
                1,
                &mut values,
                &mut dims,
            )?;

            if dims.len() > 1 {
                Ok(Variant::Array(Box::new(
                    Array::new_multi(field.scalar_type, values, dims).map_err(Error::decoding)?,
                )))
            } else {
                Ok(Variant::Array(Box::new(
                    Array::new(field.scalar_type, values).map_err(Error::decoding)?,
                )))
            }
        } else {
            self.json_decode_field_value(field, stream, ctx)
        }
    }

    pub(super) fn json_decode_type_inner(
        &self,
        stream: &mut opcua_types::json::JsonStreamReader<&mut dyn std::io::Read>,
        ctx: &Context<'_>,
        t: &Arc<StructTypeInfo>,
    ) -> EncodingResult<Box<dyn DynEncodable>> {
        match t.structure_type {
            opcua_types::StructureType::Structure
            | opcua_types::StructureType::StructureWithOptionalFields => {
                let mut by_name = HashMap::new();
                stream.begin_object()?;
                while stream.has_next()? {
                    let name = stream.next_name()?;
                    let Some(field) = t.get_field_by_name(name) else {
                        stream.skip_value()?;
                        continue;
                    };
                    by_name.insert(
                        field.name.as_str(),
                        self.json_decode_field(field, stream, ctx)?,
                    );
                }
                let mut data = Vec::with_capacity(by_name.len());
                for field in &t.fields {
                    let Some(f) = by_name.remove(field.name.as_str()) else {
                        // Just ignore decoding mask here, there really is no reason
                        // to care about it when it comes to JSON decoding.
                        if field.is_optional {
                            data.push(Variant::Empty);
                            continue;
                        }
                        return Err(Error::decoding(format!(
                            "Missing required field {}",
                            field.name
                        )));
                    };
                    data.push(f);
                }
                stream.end_object()?;

                Ok(Box::new(DynamicStructure {
                    type_def: t.clone(),
                    discriminant: 0,
                    type_tree: self.type_tree.clone(),
                    data,
                }))
            }
            opcua_types::StructureType::Union => {
                let mut value_raw: Option<Vec<u8>> = None;
                let mut value: Option<Variant> = None;
                let mut discriminant: Option<u32> = None;

                stream.begin_object()?;
                while stream.has_next()? {
                    let name = stream.next_name()?;
                    match name {
                        "Value" => {
                            if let Some(disc) = discriminant {
                                let Some(field) = t.get_field(disc as usize) else {
                                    return Err(Error::decoding(format!(
                                        "Invalid discriminant: {}",
                                        disc
                                    )));
                                };
                                value = Some(self.json_decode_field(field, stream, ctx)?);
                            } else {
                                value_raw = Some(consume_raw_value(stream)?);
                            }
                        }
                        "SwitchField" => {
                            discriminant = Some(stream.next_number()??);
                        }
                        _ => stream.skip_value()?,
                    }
                }

                let Some(discriminant) = discriminant else {
                    return Err(Error::decoding("Missing union SwitchField"));
                };

                let value = if let Some(raw) = value_raw {
                    let mut cursor = Cursor::new(raw);
                    let Some(field) = t.get_field(discriminant as usize) else {
                        return Err(Error::decoding(format!(
                            "Invalid discriminant: {}",
                            discriminant
                        )));
                    };
                    let mut inner_stream = JsonStreamReader::new(&mut cursor as &mut dyn Read);
                    self.json_decode_field(field, &mut inner_stream, ctx)?
                } else if let Some(value) = value {
                    value
                } else {
                    return Err(Error::decoding("Missing union value"));
                };

                Ok(Box::new(DynamicStructure {
                    type_def: t.clone(),
                    discriminant,
                    type_tree: self.type_tree.clone(),
                    data: vec![value],
                }))
            }
        }
    }
}

impl JsonEncodable for DynamicStructure {
    fn encode(
        &self,
        stream: &mut opcua_types::json::JsonStreamWriter<&mut dyn std::io::Write>,
        ctx: &opcua_types::Context<'_>,
    ) -> opcua_types::EncodingResult<()> {
        let s = &self.type_def;
        stream.begin_object()?;
        match s.structure_type {
            opcua_types::StructureType::Structure => {
                for (value, field) in self.data.iter().zip(s.fields.iter()) {
                    stream.name(&field.name)?;
                    self.json_encode_field(stream, value, field, ctx)?;
                }
            }
            opcua_types::StructureType::StructureWithOptionalFields => {
                let mut encoding_mask = 0u32;
                for (idx, (value, field)) in self.data.iter().zip(s.fields.iter()).enumerate() {
                    if !field.is_optional || !matches!(value, Variant::Empty) {
                        encoding_mask |= 1 << idx;
                    }
                }
                stream.name("EncodingMask")?;
                stream.number_value(encoding_mask)?;

                for (value, field) in self.data.iter().zip(s.fields.iter()) {
                    if !field.is_optional || !matches!(value, Variant::Empty) {
                        stream.name(&field.name)?;
                        self.json_encode_field(stream, value, field, ctx)?;
                    }
                }
            }
            opcua_types::StructureType::Union => {
                stream.name("SwitchField")?;
                stream.number_value(self.discriminant)?;
                let (Some(value), Some(field)) =
                    (self.data.get(0), s.fields.get(self.discriminant as usize))
                else {
                    return Err(Error::encoding(
                        "Discriminant was out of range of known fields",
                    ));
                };
                stream.name("Value")?;
                self.json_encode_field(stream, value, field, ctx)?;
            }
        }
        stream.end_object()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Cursor, Read, Seek, Write},
        sync::Arc,
    };

    use opcua_types::{
        json::{JsonDecodable, JsonEncodable, JsonStreamReader, JsonStreamWriter, JsonWriter},
        ContextOwned, DecodingOptions, EUInformation, ExtensionObject, LocalizedText, NamespaceMap,
        TypeLoaderCollection, Variant,
    };

    use crate::custom_types::{
        custom_struct::tests::{add_eu_information, make_type_tree},
        DynamicStructure, DynamicTypeLoader,
    };

    #[test]
    fn json_dynamic_struct_round_trip() {
        let mut type_tree = make_type_tree();
        add_eu_information(&mut type_tree);

        let loader = DynamicTypeLoader::new(Arc::new(type_tree));
        let mut loaders = TypeLoaderCollection::new_empty();
        loaders.add_type_loader(loader);
        let ctx = ContextOwned::new(NamespaceMap::new(), loaders, DecodingOptions::test());

        let mut write_buf = Vec::<u8>::new();
        let mut cursor = Cursor::new(&mut write_buf);
        let mut writer = JsonStreamWriter::new(&mut cursor as &mut dyn Write);

        let obj = ExtensionObject::from_message(EUInformation {
            namespace_uri: "my.namespace.uri".into(),
            unit_id: 5,
            display_name: "Degrees Celsius".into(),
            description: "Description".into(),
        });

        JsonEncodable::encode(&obj, &mut writer, &ctx.context()).unwrap();
        writer.finish_document().unwrap();
        cursor.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut reader = JsonStreamReader::new(&mut cursor as &mut dyn Read);

        let obj2: ExtensionObject = JsonDecodable::decode(&mut reader, &ctx.context()).unwrap();

        // Decode it back, resulting in a dynamic structure.
        let value = obj2.inner_as::<DynamicStructure>().unwrap();
        assert_eq!(value.data.len(), 4);
        assert_eq!(value.data[0], Variant::from("my.namespace.uri"));
        assert_eq!(value.data[1], Variant::from(5i32));
        assert_eq!(
            value.data[2],
            Variant::from(LocalizedText::from("Degrees Celsius"))
        );
        assert_eq!(
            value.data[3],
            Variant::from(LocalizedText::from("Description"))
        );

        // Re-encode it
        cursor.seek(std::io::SeekFrom::Start(0)).unwrap();
        let mut writer = JsonStreamWriter::new(&mut cursor as &mut dyn Write);
        JsonEncodable::encode(&obj2, &mut writer, &ctx.context()).unwrap();
        writer.finish_document().unwrap();

        // Make a new context, this time with the regular decoder for EUInformation
        let ctx = ContextOwned::new_default(NamespaceMap::new(), DecodingOptions::test());
        cursor.seek(std::io::SeekFrom::Start(0)).unwrap();
        let mut reader = JsonStreamReader::new(&mut cursor as &mut dyn Read);
        let obj3: ExtensionObject = JsonDecodable::decode(&mut reader, &ctx.context()).unwrap();

        assert_eq!(obj, obj3);
    }
}
