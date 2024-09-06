use chrono::{DateTime, Utc};
use roxmltree::{Document, Node};

use crate::{
    ext::{
        children_with_name, first_child_with_name_opt, value_from_attr, value_from_attr_opt,
        value_from_contents, NodeExt,
    },
    FromValue, XmlError, XmlLoad,
};

use super::opc_ua_types::Variant;

#[derive(Debug)]
pub struct NodeSet2 {
    pub node_set: Option<UANodeSet>,
    pub node_set_changes: Option<UANodeSetChanges>,
    pub node_set_changes_status: Option<UANodeSetChangesStatus>,
}

pub fn load_nodeset2_file(document: &str) -> Result<NodeSet2, XmlError> {
    let document = Document::parse(document).map_err(|e| XmlError {
        span: 0..1,
        error: crate::error::XmlErrorInner::Xml(e),
    })?;
    let root = document.root();
    Ok(NodeSet2 {
        node_set: first_child_with_name_opt(&root, "UANodeSet")?,
        node_set_changes: first_child_with_name_opt(&root, "UANodeSetChanges")?,
        node_set_changes_status: first_child_with_name_opt(&root, "UANodeSetChangesStatus")?,
    })
}

#[derive(Debug)]
pub enum UANode {
    Object(UAObject),
    Variable(UAVariable),
    Method(UAMethod),
    View(UAView),
    ObjectType(UAObjectType),
    VariableType(UAVariableType),
    DataType(UADataType),
    ReferenceType(UAReferenceType),
}

impl UANode {
    pub fn from_node(node: &Node<'_, '_>) -> Result<Option<Self>, XmlError> {
        Ok(Some(match node.tag_name().name() {
            "UAObject" => Self::Object(XmlLoad::load(node)?),
            "UAVariable" => Self::Variable(XmlLoad::load(node)?),
            "UAMethod" => Self::Method(XmlLoad::load(node)?),
            "UAView" => Self::View(XmlLoad::load(node)?),
            "UAObjectType" => Self::ObjectType(XmlLoad::load(node)?),
            "UAVariableType" => Self::VariableType(XmlLoad::load(node)?),
            "UADataType" => Self::DataType(XmlLoad::load(node)?),
            "UAReferenceType" => Self::ReferenceType(XmlLoad::load(node)?),
            _ => return Ok(None),
        }))
    }

    pub fn base(&self) -> &UANodeBase {
        match self {
            UANode::Object(n) => &n.base.base,
            UANode::Variable(n) => &n.base.base,
            UANode::Method(n) => &n.base.base,
            UANode::View(n) => &n.base.base,
            UANode::ObjectType(n) => &n.base.base,
            UANode::VariableType(n) => &n.base.base,
            UANode::DataType(n) => &n.base.base,
            UANode::ReferenceType(n) => &n.base.base,
        }
    }
}

#[derive(Debug)]
pub struct UANodeSet {
    pub namespace_uris: Option<UriTable>,
    pub server_uris: Option<UriTable>,
    pub models: Option<ModelTable>,
    pub aliases: Option<AliasTable>,
    pub nodes: Vec<UANode>,
    pub last_modified: Option<DateTime<Utc>>,
}

impl<'input> XmlLoad<'input> for UANodeSet {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        // Special case this one, we really want to avoid seeking through the entire
        // document for the optional elements. It's fine elsewhere since most nodes
        // have few children.
        let mut namespace_uris = None;
        let mut server_uris = None;
        let mut models = None;
        let mut aliases = None;
        let mut nodes = Vec::new();
        for child in node.children() {
            match child.tag_name().name() {
                "NamespaceUris" => namespace_uris = Some(XmlLoad::load(&child)?),
                "ServerUris" => server_uris = Some(XmlLoad::load(&child)?),
                "Models" => models = Some(XmlLoad::load(&child)?),
                "Aliases" => aliases = Some(XmlLoad::load(&child)?),
                _ => {
                    if let Some(node) = UANode::from_node(&child)? {
                        nodes.push(node);
                    }
                }
            }
        }

        Ok(Self {
            namespace_uris,
            server_uris,
            models,
            aliases,
            nodes,
            last_modified: value_from_attr_opt(node, "LastModified")?,
        })
    }
}

#[derive(Debug)]
pub struct UANodeSetChanges {
    pub namespace_uris: Option<UriTable>,
    pub server_uris: Option<UriTable>,
    pub aliases: Option<AliasTable>,
    pub nodes_to_add: Option<NodesToAdd>,
    pub references_to_add: Option<ReferencesToChange>,
    pub nodes_to_delete: Option<NodesToDelete>,
    pub references_to_delete: Option<ReferencesToChange>,
    pub last_modified: Option<DateTime<Utc>>,
    pub transaction_id: String,
    pub accept_all_or_nothing: bool,
}

impl<'input> XmlLoad<'input> for UANodeSetChanges {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            namespace_uris: first_child_with_name_opt(node, "NamespaceUris")?,
            server_uris: first_child_with_name_opt(node, "ServerUris")?,
            aliases: first_child_with_name_opt(node, "Aliases")?,
            nodes_to_add: first_child_with_name_opt(node, "NodesToAdd")?,
            references_to_add: first_child_with_name_opt(node, "ReferencesToAdd")?,
            nodes_to_delete: first_child_with_name_opt(node, "NodesToDelete")?,
            references_to_delete: first_child_with_name_opt(node, "ReferencesToDelete")?,
            last_modified: value_from_attr_opt(node, "LastModified")?,
            transaction_id: value_from_attr(node, "TransactionId")?,
            accept_all_or_nothing: value_from_attr_opt(node, "AcceptAllOrNothing")?
                .unwrap_or(false),
        })
    }
}

#[derive(Debug)]
pub struct UANodeSetChangesStatus {
    pub nodes_to_add: Option<NodeSetStatusList>,
    pub references_to_add: Option<NodeSetStatusList>,
    pub nodes_to_delete: Option<NodeSetStatusList>,
    pub references_to_delete: Option<NodeSetStatusList>,
    pub last_modified: Option<DateTime<Utc>>,
    pub transaction_id: String,
}

impl<'input> XmlLoad<'input> for UANodeSetChangesStatus {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            nodes_to_add: first_child_with_name_opt(node, "NodesToAdd")?,
            references_to_add: first_child_with_name_opt(node, "ReferencesToAdd")?,
            nodes_to_delete: first_child_with_name_opt(node, "NodesToDelete")?,
            references_to_delete: first_child_with_name_opt(node, "ReferencesToDelete")?,
            last_modified: value_from_attr_opt(node, "LastModified")?,
            transaction_id: value_from_attr(node, "TransactionId")?,
        })
    }
}

#[derive(Debug)]
pub struct NodesToAdd {
    pub nodes: Vec<UANode>,
}

impl<'input> XmlLoad<'input> for NodesToAdd {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            nodes: node
                .children()
                .filter_map(|n| UANode::from_node(&n).transpose())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug)]
pub struct NodeToDelete {
    pub node_id: NodeId,
    pub delete_reverse_references: bool,
}

impl<'input> XmlLoad<'input> for NodeToDelete {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            node_id: NodeId::load(node)?,
            delete_reverse_references: value_from_attr_opt(node, "DeleteReverseReferences")?
                .unwrap_or(true),
        })
    }
}

#[derive(Debug)]
pub struct NodesToDelete {
    pub nodes: Vec<NodeToDelete>,
}

impl<'input> XmlLoad<'input> for NodesToDelete {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            nodes: children_with_name(node, "Node")?,
        })
    }
}

#[derive(Debug)]
pub struct ReferenceChange {
    pub node_id: NodeId,
    pub source: NodeId,
    pub reference_type: NodeId,
    pub is_forward: bool,
}

impl<'input> XmlLoad<'input> for ReferenceChange {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            node_id: NodeId::load(node)?,
            source: value_from_attr(node, "Source")?,
            reference_type: value_from_attr(node, "ReferenceType")?,
            is_forward: value_from_attr_opt(node, "IsForward")?.unwrap_or(true),
        })
    }
}

#[derive(Debug)]
pub struct ReferencesToChange {
    pub references: Vec<ReferenceChange>,
}

impl<'input> XmlLoad<'input> for ReferencesToChange {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            references: children_with_name(node, "Reference")?,
        })
    }
}

#[derive(Debug)]
pub struct NodeSetStatus {
    pub status: String,
    pub code: u64,
}

impl<'input> XmlLoad<'input> for NodeSetStatus {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            status: String::load(node)?,
            code: value_from_attr_opt(node, "Code")?.unwrap_or(0),
        })
    }
}

#[derive(Debug)]
pub struct NodeSetStatusList {
    pub statuses: Vec<NodeSetStatus>,
}

impl<'input> XmlLoad<'input> for NodeSetStatusList {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            statuses: children_with_name(node, "Status")?,
        })
    }
}

#[derive(Debug)]
pub struct UriTable {
    pub uris: Vec<String>,
}

impl<'input> XmlLoad<'input> for UriTable {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            uris: node
                .with_name("Uri")
                .map(|v| v.try_contents().map(|v| v.to_owned()))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

macro_rules! value_wrapper {
    ($key:ident, $ty:ident) => {
        #[derive(Debug, Default, Clone)]
        pub struct $key(pub $ty);

        impl FromValue for $key {
            fn from_value(node: &Node<'_, '_>, attr: &str, v: &str) -> Result<Self, XmlError> {
                Ok(Self($ty::from_value(node, attr, v)?))
            }
        }
    };
}

#[derive(Debug)]
pub struct ModelTableEntry {
    pub role_permissions: Option<ListOfRolePermissions>,
    pub required_model: Vec<ModelTableEntry>,
    pub model_uri: String,
    pub version: Option<String>,
    pub publication_date: Option<DateTime<Utc>>,
    pub access_restrictions: AccessRestriction,
}

impl<'input> XmlLoad<'input> for ModelTableEntry {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            role_permissions: first_child_with_name_opt(node, "RolePermissions")?,
            required_model: children_with_name(node, "RequiredModel")?,
            model_uri: node.try_attribute("ModelUri")?.to_owned(),
            version: node.attribute("Version").map(|v| v.to_owned()),
            publication_date: value_from_attr_opt(node, "PublicationDate")?,
            access_restrictions: value_from_attr_opt(node, "AccessRestrictions")?
                .unwrap_or(AccessRestriction(0)),
        })
    }
}

#[derive(Debug)]
pub struct ModelTable {
    pub models: Vec<ModelTableEntry>,
}

impl<'input> XmlLoad<'input> for ModelTable {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            models: children_with_name(node, "Model")?,
        })
    }
}

value_wrapper!(NodeId, String);
value_wrapper!(QualifiedName, String);
value_wrapper!(Locale, String);
value_wrapper!(WriteMask, u32);
value_wrapper!(EventNotifier, u8);
value_wrapper!(ValueRank, i32);
value_wrapper!(AccessRestriction, u8);
value_wrapper!(ArrayDimensions, String);
value_wrapper!(Duration, f64);
value_wrapper!(AccessLevel, u8);

impl FromValue for chrono::DateTime<Utc> {
    fn from_value(node: &Node<'_, '_>, attr: &str, v: &str) -> Result<Self, XmlError> {
        let v = chrono::DateTime::parse_from_rfc3339(v)
            .map_err(|e| XmlError::parse_date_time(node, attr, e))?;
        Ok(v.with_timezone(&Utc))
    }
}

#[derive(Debug)]
pub struct NodeIdAlias {
    pub id: NodeId,
    pub alias: String,
}

impl<'input> XmlLoad<'input> for NodeIdAlias {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            id: NodeId::load(node)?,
            alias: node.try_attribute("Alias")?.to_owned(),
        })
    }
}

#[derive(Debug, Default)]
pub struct AliasTable {
    pub aliases: Vec<NodeIdAlias>,
}

impl<'input> XmlLoad<'input> for AliasTable {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            aliases: children_with_name(node, "Alias")?,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct LocalizedText {
    pub text: String,
    pub locale: Locale,
}
impl<'input> XmlLoad<'input> for LocalizedText {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            text: node.text().map(|v| v.to_owned()).unwrap_or_default(),
            locale: value_from_attr_opt(node, "Locale")?.unwrap_or_else(|| Locale("".to_owned())),
        })
    }
}

#[derive(Debug)]
pub struct SymbolicName {
    pub names: Vec<String>,
}

impl FromValue for SymbolicName {
    fn from_value(_node: &Node<'_, '_>, _attr: &str, v: &str) -> Result<Self, XmlError> {
        Ok(Self {
            names: v.split_whitespace().map(|v| v.to_owned()).collect(),
        })
    }
}

#[derive(Debug)]
pub struct Reference {
    pub node_id: NodeId,
    pub reference_type: NodeId,
    pub is_forward: bool,
}

impl<'input> XmlLoad<'input> for Reference {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            node_id: value_from_contents(node)?,
            reference_type: value_from_attr(node, "ReferenceType")?,
            is_forward: value_from_attr_opt(node, "IsForward")?.unwrap_or(true),
        })
    }
}

#[derive(Debug)]
pub struct ListOfReferences {
    pub references: Vec<Reference>,
}
impl<'input> XmlLoad<'input> for ListOfReferences {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            references: children_with_name(node, "Reference")?,
        })
    }
}

#[derive(Debug)]
pub struct RolePermission {
    pub node_id: NodeId,
    pub permissions: u64,
}

impl<'input> XmlLoad<'input> for RolePermission {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            node_id: value_from_contents(node)?,
            permissions: value_from_attr_opt(node, "Permissions")?.unwrap_or(0),
        })
    }
}

#[derive(Debug)]
pub struct ListOfRolePermissions {
    pub role_permissions: Vec<RolePermission>,
}
impl<'input> XmlLoad<'input> for ListOfRolePermissions {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            role_permissions: children_with_name(node, "RolePermission")?,
        })
    }
}

#[derive(Debug)]
pub enum ReleaseStatus {
    Released,
    Draft,
    Deprecated,
}

impl FromValue for ReleaseStatus {
    fn from_value(node: &Node<'_, '_>, attr: &str, v: &str) -> Result<Self, XmlError> {
        match v {
            "Released" => Ok(Self::Released),
            "Draft" => Ok(Self::Draft),
            "Deprecated" => Ok(Self::Deprecated),
            r => Err(XmlError::other(
                node,
                &format!("Unexpected value for {attr}: {r}"),
            )),
        }
    }
}

#[derive(Debug)]
pub struct UANodeBase {
    pub display_names: Vec<LocalizedText>,
    pub description: Vec<LocalizedText>,
    pub category: Vec<String>,
    pub documentation: Option<String>,
    pub references: Option<ListOfReferences>,
    pub role_permissions: Option<ListOfRolePermissions>,
    pub node_id: NodeId,
    pub browse_name: QualifiedName,
    pub write_mask: WriteMask,
    pub user_write_mask: WriteMask,
    pub access_restrictions: AccessRestriction,
    pub symbolic_name: Option<SymbolicName>,
    pub release_status: ReleaseStatus,
}

impl<'input> XmlLoad<'input> for UANodeBase {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            display_names: children_with_name(node, "DisplayName")?,
            description: children_with_name(node, "Description")?,
            category: children_with_name(node, "Category")?,
            documentation: first_child_with_name_opt(node, "Documentation")?,
            references: first_child_with_name_opt(node, "References")?,
            role_permissions: first_child_with_name_opt(node, "RolePermissions")?,
            node_id: value_from_attr(node, "NodeId")?,
            browse_name: value_from_attr(node, "BrowseName")?,
            write_mask: value_from_attr_opt(node, "WriteMask")?.unwrap_or(WriteMask(0)),
            user_write_mask: value_from_attr_opt(node, "UserWriteMask")?.unwrap_or(WriteMask(0)),
            access_restrictions: value_from_attr_opt(node, "AccessRestrictions")?
                .unwrap_or(AccessRestriction(0)),
            symbolic_name: value_from_attr_opt(node, "SymbolicName")?,
            release_status: value_from_attr_opt(node, "ReleaseStatus")?
                .unwrap_or(ReleaseStatus::Released),
        })
    }
}

#[derive(Debug)]
pub struct UAInstance {
    pub base: UANodeBase,
    pub parent_node_id: Option<NodeId>,
}

impl<'input> XmlLoad<'input> for UAInstance {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            base: UANodeBase::load(node)?,
            parent_node_id: value_from_attr_opt(node, "ParentNodeId")?,
        })
    }
}

#[derive(Debug)]
pub struct UAObject {
    pub base: UAInstance,
    pub event_notifier: EventNotifier,
}

impl<'input> XmlLoad<'input> for UAObject {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            base: UAInstance::load(node)?,
            event_notifier: value_from_attr_opt(node, "EventNotifier")?.unwrap_or(EventNotifier(0)),
        })
    }
}

#[derive(Debug)]
pub struct Value(pub Variant);

impl<'input> XmlLoad<'input> for Value {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self(
            node.children()
                .find(|n| !n.tag_name().name().is_empty())
                .map(|n| Variant::load(&n))
                .transpose()?
                .ok_or_else(|| XmlError::other(node, "Empty value, expected variant"))?,
        ))
    }
}

#[derive(Debug)]
pub struct UAVariable {
    pub base: UAInstance,
    pub value: Option<Value>,
    pub data_type: NodeId,
    pub value_rank: ValueRank,
    pub array_dimensions: ArrayDimensions,
    pub access_level: AccessLevel,
    pub user_access_level: AccessLevel,
    pub minimum_sampling_interval: Duration,
    pub historizing: bool,
}

impl<'input> XmlLoad<'input> for UAVariable {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            base: UAInstance::load(node)?,
            value: first_child_with_name_opt(node, "Value")?,
            data_type: value_from_attr_opt(node, "DataType")?
                .unwrap_or_else(|| NodeId("i=24".to_owned())),
            value_rank: value_from_attr_opt(node, "ValueRank")?.unwrap_or(ValueRank(-1)),
            array_dimensions: value_from_attr_opt(node, "ArrayDimensions")?
                .unwrap_or_else(|| ArrayDimensions("".to_owned())),
            access_level: value_from_attr_opt(node, "AccessLevel")?.unwrap_or(AccessLevel(1)),
            user_access_level: value_from_attr_opt(node, "UserAccessLevel")?
                .unwrap_or(AccessLevel(1)),
            minimum_sampling_interval: value_from_attr_opt(node, "MinimumSamplingInterval")?
                .unwrap_or(Duration(0.0)),
            historizing: value_from_attr_opt(node, "Historizing")?.unwrap_or(false),
        })
    }
}

#[derive(Debug)]
pub struct UAMethodArgument {
    pub name: Option<String>,
    pub descriptions: Vec<LocalizedText>,
}

impl<'input> XmlLoad<'input> for UAMethodArgument {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            name: first_child_with_name_opt(node, "Name")?,
            descriptions: children_with_name(node, "Description")?,
        })
    }
}

#[derive(Debug)]
pub struct UAMethod {
    pub base: UAInstance,
    pub arguments: Vec<UAMethodArgument>,
    pub executable: bool,
    pub user_executable: bool,
    pub method_declaration_id: Option<NodeId>,
}

impl<'input> XmlLoad<'input> for UAMethod {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            base: UAInstance::load(node)?,
            arguments: children_with_name(node, "ArgumentDescription")?,
            executable: value_from_attr_opt(node, "Executable")?.unwrap_or(false),
            user_executable: value_from_attr_opt(node, "UserExecutable")?.unwrap_or(false),
            method_declaration_id: value_from_attr_opt(node, "MethodDeclarationId")?,
        })
    }
}

#[derive(Debug)]
pub struct StructureTranslationType {
    pub text: Vec<LocalizedText>,
    pub name: String,
}

impl<'input> XmlLoad<'input> for StructureTranslationType {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            text: children_with_name(node, "Text")?,
            name: value_from_attr(node, "Name")?,
        })
    }
}

#[derive(Debug)]
pub enum TranslationType {
    Text(Vec<LocalizedText>),
    Field(Vec<StructureTranslationType>),
    None,
}

impl<'input> XmlLoad<'input> for TranslationType {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        let texts = children_with_name(node, "Text")?;
        if !texts.is_empty() {
            return Ok(Self::Text(texts));
        }
        let fields = children_with_name(node, "Field")?;
        if !fields.is_empty() {
            return Ok(Self::Field(fields));
        }
        Ok(Self::None)
    }
}

#[derive(Debug)]
pub struct UAView {
    pub base: UAInstance,
    pub contains_no_loops: bool,
    pub event_notifier: EventNotifier,
}

impl<'input> XmlLoad<'input> for UAView {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            base: UAInstance::load(node)?,
            contains_no_loops: value_from_attr_opt(node, "ContainsNoLoops")?.unwrap_or(false),
            event_notifier: value_from_attr(node, "EventNotifier")?,
        })
    }
}

#[derive(Debug)]
pub struct UAType {
    pub base: UANodeBase,
    pub is_abstract: bool,
}

impl<'input> XmlLoad<'input> for UAType {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            base: UANodeBase::load(node)?,
            is_abstract: value_from_attr_opt(node, "IsAbstract")?.unwrap_or(false),
        })
    }
}

#[derive(Debug)]
pub struct UAObjectType {
    pub base: UAType,
}

impl<'input> XmlLoad<'input> for UAObjectType {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            base: UAType::load(node)?,
        })
    }
}

#[derive(Debug)]
pub struct UAVariableType {
    pub base: UAType,
    pub value: Option<Value>,
    pub data_type: NodeId,
    pub value_rank: ValueRank,
    pub array_dimensions: ArrayDimensions,
}

impl<'input> XmlLoad<'input> for UAVariableType {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            base: UAType::load(node)?,
            value: first_child_with_name_opt(node, "Value")?,
            data_type: value_from_attr_opt(node, "DataType")?
                .unwrap_or_else(|| NodeId("i=24".to_owned())),
            value_rank: value_from_attr_opt(node, "ValueRank")?.unwrap_or(ValueRank(-1)),
            array_dimensions: value_from_attr_opt(node, "ArrayDimensions")?
                .unwrap_or_else(|| ArrayDimensions("".to_owned())),
        })
    }
}

#[derive(Debug)]
pub enum DataTypePurpose {
    Normal,
    ServicesOnly,
    CodeGenerator,
}

impl FromValue for DataTypePurpose {
    fn from_value(node: &Node<'_, '_>, attr: &str, v: &str) -> Result<Self, XmlError> {
        match v {
            "Normal" => Ok(Self::Normal),
            "ServicesOnly" => Ok(Self::ServicesOnly),
            "CodeGenerator" => Ok(Self::CodeGenerator),
            r => Err(XmlError::other(
                node,
                &format!("Unexpected value for {attr}: {r}"),
            )),
        }
    }
}

#[derive(Debug)]
pub struct DataTypeField {
    pub display_names: Vec<LocalizedText>,
    pub descriptions: Vec<LocalizedText>,
    pub name: String,
    pub symbolic_name: Option<SymbolicName>,
    pub data_type: NodeId,
    pub value_rank: ValueRank,
    pub array_dimensions: ArrayDimensions,
    pub max_string_length: u64,
    pub value: i64,
    pub is_optional: bool,
    pub allow_sub_types: bool,
}

impl<'input> XmlLoad<'input> for DataTypeField {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            display_names: children_with_name(node, "DisplayName")?,
            descriptions: children_with_name(node, "Description")?,
            name: value_from_attr(node, "Name")?,
            symbolic_name: value_from_attr_opt(node, "SymbolicName")?,
            data_type: value_from_attr_opt(node, "DataType")?
                .unwrap_or_else(|| NodeId("i=24".to_owned())),
            value_rank: value_from_attr_opt(node, "ValueRank")?.unwrap_or(ValueRank(-1)),
            array_dimensions: value_from_attr_opt(node, "ArrayDimensions")?
                .unwrap_or_else(|| ArrayDimensions("".to_owned())),
            max_string_length: value_from_attr_opt(node, "MaxStringLength")?.unwrap_or(0),
            value: value_from_attr_opt(node, "Value")?.unwrap_or(-1),
            is_optional: value_from_attr_opt(node, "IsOptional")?.unwrap_or(false),
            allow_sub_types: value_from_attr_opt(node, "AllowSubTypes")?.unwrap_or(false),
        })
    }
}

#[derive(Debug)]
pub struct DataTypeDefinition {
    pub fields: Vec<DataTypeField>,
    pub name: QualifiedName,
    pub symbolic_name: SymbolicName,
    pub is_union: bool,
    pub is_option_set: bool,
}

impl<'input> XmlLoad<'input> for DataTypeDefinition {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            fields: children_with_name(node, "Field")?,
            name: value_from_attr(node, "Name")?,
            symbolic_name: value_from_attr_opt(node, "SymbolicName")?
                .unwrap_or_else(|| SymbolicName { names: Vec::new() }),
            is_union: value_from_attr_opt(node, "IsUnion")?.unwrap_or(false),
            is_option_set: value_from_attr_opt(node, "IsOptionSet")?.unwrap_or(false),
        })
    }
}

#[derive(Debug)]
pub struct UADataType {
    pub base: UAType,
    pub definition: Option<DataTypeDefinition>,
    pub purpose: DataTypePurpose,
}

impl<'input> XmlLoad<'input> for UADataType {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            base: UAType::load(node)?,
            definition: first_child_with_name_opt(node, "Definition")?,
            purpose: value_from_attr_opt(node, "Purpose")?.unwrap_or(DataTypePurpose::Normal),
        })
    }
}

#[derive(Debug)]
pub struct UAReferenceType {
    pub base: UAType,
    pub inverse_names: Vec<LocalizedText>,
    pub symmetric: bool,
}

impl<'input> XmlLoad<'input> for UAReferenceType {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            base: UAType::load(node)?,
            inverse_names: children_with_name(node, "InverseName")?,
            symmetric: value_from_attr_opt(node, "Symmetric")?.unwrap_or(false),
        })
    }
}
