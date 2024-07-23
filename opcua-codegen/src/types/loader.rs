use std::collections::{HashMap, HashSet};

use log::trace;
use roxmltree::{Document, Node};

use crate::{
    error::CodeGenError,
    xml::{to_snake_case, NodeExt},
    StructureField, StructureFieldType, StructuredType,
};

use super::{enum_type::EnumReprType, EnumType, EnumValue};

pub struct TypeLoader<'a> {
    ignored: HashSet<String>,
    native_type_mappings: HashMap<String, String>,
    xml: Document<'a>,
}

fn strip_first_segment<'a>(
    val: &'a str,
    sep: &'static str,
) -> Result<&'a str, CodeGenError<'static>> {
    val.split_once(sep)
        .ok_or_else(|| CodeGenError::WrongFormat(format!("A{sep}B.."), val.to_owned()))
        .map(|v| v.1)
}

#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct LoadedTypes {
    pub structures: Vec<StructuredType>,
    pub enums: Vec<EnumType>,
}

#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(feature = "ser", serde(untagged))]
#[derive(Debug)]
pub enum LoadedType {
    Struct(StructuredType),
    Enum(EnumType),
}

impl LoadedType {
    pub fn name(&self) -> &str {
        match self {
            LoadedType::Struct(s) => &s.name,
            LoadedType::Enum(s) => &s.name,
        }
    }
}

impl<'input> TypeLoader<'input> {
    pub fn new(
        ignored: HashSet<String>,
        native_type_mappings: HashMap<String, String>,
        data: &'input str,
    ) -> Result<Self, CodeGenError<'input>> {
        Ok(Self {
            ignored,
            native_type_mappings,
            xml: Document::parse(data)?,
        })
    }

    fn massage_type_name(&self, name: &str) -> String {
        self.native_type_mappings
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.to_owned())
    }

    fn load_structure<'a>(
        &self,
        name: &str,
        node: Node<'a, 'input>,
    ) -> Result<StructuredType, CodeGenError<'input>> {
        let mut fields_to_add = Vec::new();
        let mut fields_to_hide = Vec::new();
        for field in node.with_name("Field") {
            let field_name = to_snake_case(field.try_attribute("Name")?);
            let typ = strip_first_segment(field.try_attribute("TypeName")?, ":")?;
            let typ = self.massage_type_name(typ);

            if let Some(length_attr) = field.attribute("LengthField") {
                fields_to_add.push(StructureField {
                    name: field_name,
                    typ: StructureFieldType::Array(typ),
                });
                fields_to_hide.push(to_snake_case(length_attr));
            } else {
                fields_to_add.push(StructureField {
                    name: field_name,
                    typ: StructureFieldType::Field(typ),
                });
            }
        }
        Ok(StructuredType {
            name: name.to_owned(),
            fields: fields_to_add,
            hidden_fields: fields_to_hide,
            documentation: node.child_contents("Documentation").map(|v| v.to_owned()),
            base_type: node.attribute("BaseType").map(|v| v.to_owned()),
            is_union: false,
        })
    }

    fn load_enum<'a>(
        &self,
        name: &str,
        node: Node<'a, 'input>,
    ) -> Result<EnumType, CodeGenError<'input>> {
        let len = node.try_attribute("LengthInBits")?;
        let len: u64 = len
            .parse()
            .map_err(|e| CodeGenError::ParseInt(len.to_owned(), e))?;
        let len_bytes = ((len as f64) / 8.0).ceil() as u64;
        let ty = match len_bytes {
            1 => EnumReprType::u8,
            2 => EnumReprType::i16,
            4 => EnumReprType::i32,
            8 => EnumReprType::i64,
            r => {
                return Err(CodeGenError::Other(format!(
                    "Unexpected enum length. {r} bytes for {name}"
                )))
            }
        };
        let mut variants = Vec::new();
        for val in node.with_name("EnumeratedValue") {
            let value = val.try_attribute("Value")?;
            let value = value
                .parse()
                .map_err(|e| CodeGenError::ParseInt(value.to_owned(), e))?;

            variants.push(EnumValue {
                name: val.try_attribute("Name")?.to_owned(),
                value,
            });
        }

        Ok(EnumType {
            name: name.to_owned(),
            values: variants,
            documentation: node.child_contents("Documentation").map(|v| v.to_owned()),
            option: node.attribute("IsOptionSet") == Some("true"),
            typ: ty,
            size: len_bytes,
            default_value: None,
        })
    }

    pub fn from_bsd(&self) -> Result<Vec<LoadedType>, CodeGenError<'input>> {
        let type_dict = self.xml.root().first_child_with_name("TypeDictionary")?;

        let mut types = Vec::new();

        for element in type_dict.children() {
            match element.tag_name().name() {
                "StructuredType" => {
                    let name = element.try_attribute("Name")?;
                    if self.ignored.contains(name) {
                        continue;
                    }
                    types.push(LoadedType::Struct(self.load_structure(name, element)?));
                }
                "EnumeratedType" => {
                    let name = element.try_attribute("Name")?;
                    if self.ignored.contains(name) {
                        continue;
                    }
                    types.push(LoadedType::Enum(self.load_enum(name, element)?));
                }
                r => {
                    trace!("Unknown field type {r}");
                    continue;
                }
            }
        }

        for element in type_dict.with_name("StructuredType") {
            let name = element.tag_name().name();
            if self.ignored.contains(name) {
                continue;
            }
        }

        Ok(types)
    }

    pub fn from_nodeset(&self) -> Result<Vec<LoadedType>, CodeGenError<'input>> {
        let mut types = Vec::new();

        let mut type_names: HashMap<_, _> = [
            ("i=1", "bool"),
            ("i=2", "i8"),
            ("i=3", "u8"),
            ("i=4", "i16"),
            ("i=5", "u16"),
            ("i=6", "i32"),
            ("i=7", "u32"),
            ("i=8", "i64"),
            ("i=9", "u64"),
            ("i=10", "f32"),
            ("i=11", "f64"),
            ("i=12", "String"),
            ("i=13", "time.Time"),
            ("i=14", "*GUID"),
            ("i=15", "[u8]"),
            ("i=16", "XMLElement"),
            ("i=17", "NodeID"),
            ("i=18", "ExpandedNodeID"),
            ("i=19", "StatusCode"),
            ("i=20", "QualifiedName"),
            ("i=21", "LocalizedText"),
            ("i=22", "ExtensionObject"),
            ("i=23", "DataValue"),
            ("i=24", "Variant"),
            ("i=25", "DiagnosticInfo"),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect();

        // Load type names first
        let node_set = self.xml.root().first_child_with_name("UANodeSet")?;
        for data_type in node_set.with_name("UADataType") {
            type_names.insert(
                data_type.try_attribute("NodeId")?.to_owned(),
                data_type.try_child_contents("DisplayName")?.to_owned(),
            );
        }

        for data_type in node_set.with_name("UADataType") {
            let name = data_type.try_child_contents("DisplayName")?;
            if self.ignored.contains(name) {
                continue;
            }

            let Ok(definition) = data_type.first_child_with_name("Definition") else {
                continue;
            };

            let fields: Vec<_> = definition.with_name("Field").collect();
            let is_enum = fields.iter().any(|f| f.attribute("Value").is_some());

            if is_enum {
                let mut enum_fields = Vec::new();
                for field in fields {
                    let value = field.try_attribute("Value")?;
                    let value = value
                        .parse()
                        .map_err(|e| CodeGenError::ParseInt(value.to_owned(), e))?;
                    enum_fields.push(EnumValue {
                        name: field.try_attribute("Name")?.to_owned(),
                        value,
                    })
                }

                types.push(LoadedType::Enum(EnumType {
                    name: name.to_owned(),
                    values: enum_fields,
                    documentation: data_type
                        .child_contents("Documentation")
                        .map(|v| v.to_owned()),
                    typ: EnumReprType::i32,
                    size: 4,
                    option: definition.child_contents("IsOptionSet") == Some("true"),
                    default_value: None,
                }));
            } else {
                let mut fields_to_add = Vec::new();

                for field in fields {
                    let field_name = field.try_attribute("Name")?;

                    let raw_typ = field.try_attribute("DataType")?;
                    let typ = if let Ok(r) = strip_first_segment(raw_typ, ":") {
                        r
                    } else {
                        raw_typ
                    };
                    let typ = type_names
                        .get(typ)
                        .ok_or_else(|| CodeGenError::Other(format!("Unknown type: {typ}")))?;

                    let value_rank: Option<i32> =
                        field.attribute("ValueRank").and_then(|v| v.parse().ok());
                    let is_array = value_rank.is_some_and(|v| v != 0);
                    if is_array {
                        fields_to_add.push(StructureField {
                            name: field_name.to_owned(),
                            typ: StructureFieldType::Array(typ.to_owned()),
                        });
                    } else {
                        fields_to_add.push(StructureField {
                            name: field_name.to_owned(),
                            typ: StructureFieldType::Field(typ.to_owned()),
                        });
                    }
                }

                let base_type_node = data_type
                    .first_child_with_name("References")?
                    .with_name("Reference")
                    .find(|r| {
                        r.attribute("ReferenceType") == Some("HasSubtype")
                            && r.attribute("IsForward") == Some("true")
                    });
                let base_type = base_type_node
                    .and_then(|n| n.text())
                    .and_then(|v| type_names.get(v));

                types.push(LoadedType::Struct(StructuredType {
                    name: name.to_owned(),
                    fields: fields_to_add,
                    hidden_fields: Vec::new(),
                    documentation: data_type
                        .child_contents("Documentation")
                        .map(|v| v.to_owned()),
                    base_type: base_type.cloned(),
                    is_union: definition.attribute("IsUnion") == Some("true"),
                }))
            }
        }

        Ok(types)
    }
}
