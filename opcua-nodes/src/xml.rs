use std::{
    path::Path,
    str::FromStr,
    sync::{Arc, OnceLock},
};

use log::warn;
use opcua_types::{
    xml::{XmlContext, XmlLoader},
    DataTypeDefinition, DataValue, EnumDefinition, EnumField, LocalizedText, NodeClass, NodeId,
    QualifiedName, StructureDefinition, StructureField, StructureType, Variant,
};
use opcua_xml::{
    load_nodeset2_file,
    schema::ua_node_set::{
        self, ArrayDimensions, ListOfReferences, UADataType, UAMethod, UANodeSet, UAObject,
        UAObjectType, UAReferenceType, UAVariable, UAVariableType, UAView,
    },
    XmlError,
};
use regex::Regex;
use thiserror::Error;

use crate::{
    Base, DataType, EventNotifier, ImportedItem, ImportedReference, Method, NodeSetImport, Object,
    ObjectType, ReferenceType, Variable, VariableType, View,
};

/// [`NodeSetImport`] implementation for dynamically loading NodeSet2 files at
/// runtime. Note that structures must be loaded with a type loader. By default
/// the type loader for the base types is registered, but if your NodeSet2 file uses custom types
/// you will have to add an [`XmlLoader`] using [`NodeSet2Import::add_type_loader`].
pub struct NodeSet2Import {
    type_loaders: Vec<Arc<dyn XmlLoader>>,
    dependent_namespaces: Vec<String>,
    preferred_locale: String,
    file: UANodeSet,
}

static QUALIFIED_NAME_REGEX: OnceLock<Regex> = OnceLock::new();

fn qualified_name_regex() -> &'static Regex {
    QUALIFIED_NAME_REGEX.get_or_init(|| Regex::new(r"^((?P<ns>[0-9]+):)?(?P<name>.*)$").unwrap())
}

#[derive(Error, Debug)]
/// Error when loading NodeSet2 XML.
pub enum LoadXmlError {
    /// The XML file failed to parse.
    #[error("{0}")]
    Xml(#[from] XmlError),
    /// The file failed to load.
    #[error("{0}")]
    Io(#[from] std::io::Error),
    /// The nodeset section is missing from the file. It is most likely invalid.
    #[error("Missing <NodeSet> section from file")]
    MissingNodeSet,
}

impl NodeSet2Import {
    /// Create a new NodeSet2 importer.
    /// The `dependent_namespaces` array contains namespaces that this nodeset requires, in order,
    /// but that are _not_ included in the nodeset file itself.
    /// It does not need to include the base namespace, but it may.
    ///
    /// # Example
    ///
    /// ```ignore
    /// NodeSet2Import::new(
    ///     "en",
    ///     "My.ISA95.Extension.NodeSet2.xml",
    ///     // Since we depend on ISA95, we need to include the ISA95 namespace.
    ///     // Typically, the NodeSet will reference ns=1 as ISA95, and ns=2 as its own
    ///     // namespace, this will allow us to interpret ns=1 correctly. Without this,
    ///     // we would panic when failing to look up ns=2.
    ///     vec!["http://www.OPCFoundation.org/UA/2013/01/ISA95"]
    /// )
    /// ```
    pub fn new(
        preferred_locale: &str,
        path: impl AsRef<Path>,
        dependent_namespaces: Vec<String>,
    ) -> Result<Self, LoadXmlError> {
        let content = std::fs::read_to_string(path)?;
        let nodeset = load_nodeset2_file(&content)?;
        let nodeset = nodeset
            .node_set
            .ok_or_else(|| LoadXmlError::MissingNodeSet)?;

        Ok(Self {
            preferred_locale: preferred_locale.to_owned(),
            type_loaders: vec![Arc::new(opcua_types::service_types::TypesXmlLoader)],
            dependent_namespaces,
            file: nodeset,
        })
    }

    /// Create a new importer with a pre-loaded nodeset.
    /// The `dependent_namespaces` array contains namespaces that this nodeset requires, in order,
    /// but that are _not_ included in the nodeset file itself.
    /// It does not need to include the base namespace, but it may.
    pub fn new_nodeset(
        preferred_locale: &str,
        nodeset: UANodeSet,
        dependent_namespaces: Vec<String>,
    ) -> Self {
        Self {
            preferred_locale: preferred_locale.to_owned(),
            type_loaders: vec![Arc::new(opcua_types::service_types::TypesXmlLoader)],
            file: nodeset,
            dependent_namespaces,
        }
    }

    pub fn add_type_loader(&mut self, loader: Arc<dyn XmlLoader>) {
        self.type_loaders.push(loader);
    }

    fn select_localized_text(&self, texts: &[ua_node_set::LocalizedText]) -> Option<LocalizedText> {
        let mut selected_str = None;
        for text in texts {
            if text.locale.0 == "" && selected_str.is_none()
                || text.locale.0 == self.preferred_locale
            {
                selected_str = Some(text);
            }
        }
        let selected_str = selected_str.or_else(|| texts.first());
        let selected = selected_str?;
        Some(LocalizedText::new(&selected.locale.0, &selected.text))
    }

    fn make_node_id(&self, node_id: &ua_node_set::NodeId, ctx: &XmlContext<'_>) -> Option<NodeId> {
        let mut node_id_str = &node_id.0;
        if let Some(aliased) = ctx.aliases.get(&node_id.0) {
            node_id_str = aliased;
        };

        let Some(mut parsed) = NodeId::from_str(node_id_str).ok() else {
            warn!("Failed to parse node ID: {node_id_str}");
            return None;
        };
        parsed.namespace = ctx.namespaces.get_index(parsed.namespace);
        Some(parsed)
    }

    fn make_qualified_name(
        &self,
        qname: &ua_node_set::QualifiedName,
        ctx: &XmlContext<'_>,
    ) -> Option<QualifiedName> {
        let captures = qualified_name_regex().captures(&qname.0)?;

        let namespace = if let Some(ns) = captures.name("ns") {
            ns.as_str().parse::<u16>().ok()?
        } else {
            0
        };

        let namespace = ctx.namespaces.get_index(namespace);
        Some(QualifiedName::new(
            namespace,
            captures.name("name")?.as_str(),
        ))
    }

    fn make_array_dimensions(&self, dims: &ArrayDimensions) -> Option<Vec<u32>> {
        if dims.0.trim().is_empty() {
            return None;
        }

        let mut values = Vec::new();
        for it in dims.0.split(',') {
            let Ok(r) = it.trim().parse::<u32>() else {
                warn!("Invalid array dimensions: {}", dims.0);
                continue;
            };
            values.push(r);
        }
        if values.is_empty() {
            None
        } else {
            Some(values)
        }
    }

    fn make_data_type_def(
        &self,
        def: &ua_node_set::DataTypeDefinition,
        ctx: &XmlContext<'_>,
    ) -> DataTypeDefinition {
        let is_enum = def.fields.first().is_some_and(|f| f.value != -1);
        if is_enum {
            let fields = def
                .fields
                .iter()
                .map(|field| EnumField {
                    value: field.value,
                    display_name: self
                        .select_localized_text(&field.display_names)
                        .unwrap_or_default(),
                    description: self
                        .select_localized_text(&field.descriptions)
                        .unwrap_or_default(),
                    name: field.name.clone().into(),
                })
                .collect();
            DataTypeDefinition::Enum(EnumDefinition {
                fields: Some(fields),
            })
        } else {
            let mut any_optional = false;
            let mut fields = Vec::with_capacity(def.fields.len());
            for field in &def.fields {
                any_optional |= field.is_optional;
                fields.push(StructureField {
                    name: field.name.clone().into(),
                    description: self
                        .select_localized_text(&field.descriptions)
                        .unwrap_or_default(),
                    data_type: self.make_node_id(&field.data_type, ctx).unwrap_or_default(),
                    value_rank: field.value_rank.0,
                    array_dimensions: self.make_array_dimensions(&field.array_dimensions),
                    max_string_length: field.max_string_length as u32,
                    is_optional: field.is_optional,
                });
            }
            DataTypeDefinition::Structure(StructureDefinition {
                default_encoding_id: NodeId::null(),
                base_data_type: NodeId::null(),
                structure_type: if def.is_union {
                    StructureType::Union
                } else if any_optional {
                    StructureType::StructureWithOptionalFields
                } else {
                    StructureType::Structure
                },
                fields: Some(fields),
            })
        }
    }

    fn make_base(
        &self,
        ctx: &XmlContext<'_>,
        base: &ua_node_set::UANodeBase,
        node_class: NodeClass,
    ) -> Option<Base> {
        Some(Base::new_full(
            self.make_node_id(&base.node_id, ctx)?,
            node_class,
            self.make_qualified_name(&base.browse_name, ctx)?,
            self.select_localized_text(&base.display_names)
                .unwrap_or_default(),
            self.select_localized_text(&base.description),
            Some(base.write_mask.0),
            Some(base.user_write_mask.0),
        ))
    }

    fn make_references(
        &self,
        ctx: &XmlContext<'_>,
        base: &Base,
        refs: &Option<ListOfReferences>,
    ) -> Vec<ImportedReference> {
        let Some(refs) = refs.as_ref() else {
            return Vec::new();
        };
        let mut res = Vec::with_capacity(refs.references.len());
        for rf in &refs.references {
            let Some(target_id) = self.make_node_id(&rf.node_id, ctx) else {
                warn!(
                    "Invalid target ID {} on reference from node {}",
                    rf.node_id.0, base.node_id
                );
                continue;
            };
            let Some(type_id) = self.make_node_id(&rf.reference_type, ctx) else {
                warn!(
                    "Invalid reference type ID {} on reference from node {}",
                    rf.node_id.0, base.node_id
                );
                continue;
            };
            res.push(ImportedReference {
                target_id,
                type_id,
                is_forward: rf.is_forward,
            });
        }
        res
    }

    fn make_object(&self, ctx: &XmlContext<'_>, node: &UAObject) -> Option<ImportedItem> {
        let base = self.make_base(ctx, &node.base.base, NodeClass::Object)?;
        Some(ImportedItem {
            references: self.make_references(&ctx, &base, &node.base.base.references),
            node: Object::new_full(
                base,
                EventNotifier::from_bits_truncate(node.event_notifier.0),
            )
            .into(),
        })
    }

    fn make_variable(&self, ctx: &XmlContext<'_>, node: &UAVariable) -> Option<ImportedItem> {
        let base = self.make_base(ctx, &node.base.base, NodeClass::Variable)?;
        Some(ImportedItem {
            references: self.make_references(ctx, &base, &node.base.base.references),
            node: Variable::new_full(
                base,
                self.make_node_id(&node.data_type, ctx)?,
                node.historizing,
                node.value_rank.0,
                node.value
                    .as_ref()
                    .map(|v| DataValue::new_now(Variant::from_nodeset(&v.0, ctx)))
                    .unwrap_or_else(|| DataValue::null()),
                node.access_level.0,
                node.user_access_level.0,
                self.make_array_dimensions(&node.array_dimensions),
                Some(node.minimum_sampling_interval.0),
            )
            .into(),
        })
    }

    fn make_method(&self, ctx: &XmlContext<'_>, node: &UAMethod) -> Option<ImportedItem> {
        let base = self.make_base(ctx, &node.base.base, NodeClass::Method)?;
        Some(ImportedItem {
            references: self.make_references(ctx, &base, &node.base.base.references),
            node: Method::new_full(base, node.executable, node.user_executable).into(),
        })
    }

    fn make_view(&self, ctx: &XmlContext<'_>, node: &UAView) -> Option<ImportedItem> {
        let base = self.make_base(ctx, &node.base.base, NodeClass::View)?;
        Some(ImportedItem {
            references: self.make_references(ctx, &base, &node.base.base.references),
            node: View::new_full(
                base,
                EventNotifier::from_bits_truncate(node.event_notifier.0),
                node.contains_no_loops,
            )
            .into(),
        })
    }

    fn make_object_type(&self, ctx: &XmlContext<'_>, node: &UAObjectType) -> Option<ImportedItem> {
        let base = self.make_base(ctx, &node.base.base, NodeClass::ObjectType)?;
        Some(ImportedItem {
            references: self.make_references(ctx, &base, &node.base.base.references),
            node: ObjectType::new_full(base, node.base.is_abstract).into(),
        })
    }

    fn make_variable_type(
        &self,
        ctx: &XmlContext<'_>,
        node: &UAVariableType,
    ) -> Option<ImportedItem> {
        let base = self.make_base(ctx, &node.base.base, NodeClass::VariableType)?;
        Some(ImportedItem {
            references: self.make_references(ctx, &base, &node.base.base.references),
            node: VariableType::new_full(
                base,
                self.make_node_id(&node.data_type, ctx)?,
                node.base.is_abstract,
                node.value_rank.0,
                node.value
                    .as_ref()
                    .map(|v| DataValue::new_now(Variant::from_nodeset(&v.0, ctx))),
                self.make_array_dimensions(&node.array_dimensions),
            )
            .into(),
        })
    }

    fn make_data_type(&self, ctx: &XmlContext<'_>, node: &UADataType) -> Option<ImportedItem> {
        let base = self.make_base(ctx, &node.base.base, NodeClass::DataType)?;
        Some(ImportedItem {
            references: self.make_references(ctx, &base, &node.base.base.references),
            node: DataType::new_full(
                base,
                node.base.is_abstract,
                node.definition
                    .as_ref()
                    .map(|v| self.make_data_type_def(v, ctx)),
            )
            .into(),
        })
    }

    fn make_reference_type(
        &self,
        ctx: &XmlContext<'_>,
        node: &UAReferenceType,
    ) -> Option<ImportedItem> {
        let base = self.make_base(ctx, &node.base.base, NodeClass::ReferenceType)?;
        Some(ImportedItem {
            references: self.make_references(ctx, &base, &node.base.base.references),
            node: ReferenceType::new_full(
                base,
                node.symmetric,
                node.base.is_abstract,
                self.select_localized_text(&node.inverse_names),
            )
            .into(),
        })
    }
}

impl NodeSetImport for NodeSet2Import {
    fn register_namespaces(&self, namespaces: &mut opcua_types::NodeSetNamespaceMapper) {
        let nss = self.get_own_namespaces();
        // If the root namespace is in the namespace array, use absolute indexes,
        // else, start at 1
        let mut offset = 1;
        for (idx, ns) in self
            .dependent_namespaces
            .iter()
            .chain(nss.iter())
            .enumerate()
        {
            if ns == "http://opcfoundation.org/UA/" {
                offset = 0;
                continue;
            }
            println!("Adding new namespace: {} {}", idx, ns);
            namespaces.add_namespace(ns, idx as u16 + offset);
        }
    }

    fn get_own_namespaces(&self) -> Vec<String> {
        self.file
            .namespace_uris
            .as_ref()
            .map(|n| n.uris.clone())
            .unwrap_or_default()
    }

    fn load<'a>(
        &'a self,
        namespaces: &'a opcua_types::NodeSetNamespaceMapper,
    ) -> Box<dyn Iterator<Item = crate::ImportedItem> + 'a> {
        let aliases = self
            .file
            .aliases
            .iter()
            .flat_map(|i| i.aliases.iter())
            .map(|alias| (alias.alias.clone(), alias.id.0.clone()))
            .collect();
        let ctx = XmlContext {
            namespaces,
            aliases,
            loaders: self.type_loaders.clone(),
        };
        Box::new(
            self.file
                .nodes
                .iter()
                .filter_map(move |raw_node| match raw_node {
                    opcua_xml::schema::ua_node_set::UANode::Object(node) => {
                        self.make_object(&ctx, node)
                    }
                    opcua_xml::schema::ua_node_set::UANode::Variable(node) => {
                        self.make_variable(&ctx, node)
                    }
                    opcua_xml::schema::ua_node_set::UANode::Method(node) => {
                        self.make_method(&ctx, node)
                    }
                    opcua_xml::schema::ua_node_set::UANode::View(node) => {
                        self.make_view(&ctx, node)
                    }
                    opcua_xml::schema::ua_node_set::UANode::ObjectType(node) => {
                        self.make_object_type(&ctx, node)
                    }
                    opcua_xml::schema::ua_node_set::UANode::VariableType(node) => {
                        self.make_variable_type(&ctx, node)
                    }
                    opcua_xml::schema::ua_node_set::UANode::DataType(node) => {
                        self.make_data_type(&ctx, node)
                    }
                    opcua_xml::schema::ua_node_set::UANode::ReferenceType(node) => {
                        self.make_reference_type(&ctx, node)
                    }
                }),
        )
    }
}
