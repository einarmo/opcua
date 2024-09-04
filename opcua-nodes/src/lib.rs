use bitflags::bitflags;

mod events;
mod generic;
mod import;
mod namespaces;
mod type_tree;

pub use base::Base;
pub use data_type::{DataType, DataTypeBuilder};
pub use events::*;
pub use generic::new_node_from_attributes;
pub use import::{ImportedItem, ImportedReference, NodeSetImport, NodeSetNamespaceMapper};
pub use method::{Method, MethodBuilder};
pub use namespaces::NamespaceMap;
pub use node::{HasNodeId, Node, NodeBase, NodeType};
pub use object::{Object, ObjectBuilder};
pub use object_type::{ObjectType, ObjectTypeBuilder};
use opcua_types::NodeId;
pub use reference_type::{ReferenceType, ReferenceTypeBuilder};
pub use type_tree::{
    DefaultTypeTree, TypeProperty, TypePropertyInverseRef, TypeTree, TypeTreeNode,
};
pub use variable::{Variable, VariableBuilder};
pub use variable_type::{VariableType, VariableTypeBuilder};
pub use view::{View, ViewBuilder};

pub use opcua_macros::{Event, EventField};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ReferenceDirection {
    Forward,
    Inverse,
}

#[derive(Debug)]
pub enum FromAttributesError {
    InvalidMask,
    MissingMandatoryValues,
}

pub trait NodeInsertTarget {
    fn insert<'a>(
        &mut self,
        node: impl Into<NodeType>,
        references: Option<&'a [(&'a NodeId, &NodeId, ReferenceDirection)]>,
    ) -> bool;
}

// A macro for creating builders. Builders can be used for more conveniently creating objects,
// variables etc.
macro_rules! node_builder_impl {
    ( $node_builder_ty:ident, $node_ty:ident ) => {
        use log::trace;
        use opcua_types::{LocalizedText, NodeId, QualifiedName, ReferenceTypeId};
        use $crate::ReferenceDirection;
        // use $crate::{address_space::AddressSpace, ReferenceDirection};

        /// A builder for constructing a node of same name. This can be used as an easy way
        /// to create a node and the references it has to another node in a simple fashion.
        pub struct $node_builder_ty {
            node: $node_ty,
            references: Vec<(NodeId, NodeId, ReferenceDirection)>,
        }

        impl $node_builder_ty {
            /// Creates a builder for a node. All nodes are required to su
            pub fn new<T, S>(node_id: &NodeId, browse_name: T, display_name: S) -> Self
            where
                T: Into<QualifiedName>,
                S: Into<LocalizedText>,
            {
                trace!("Creating a node using a builder, node id {}", node_id);
                Self {
                    node: $node_ty::default(),
                    references: Vec::with_capacity(10),
                }
                .node_id(node_id.clone())
                .browse_name(browse_name)
                .display_name(display_name)
            }

            pub fn get_node_id(&self) -> &NodeId {
                self.node.node_id()
            }

            fn node_id(mut self, node_id: NodeId) -> Self {
                let _ = self.node.base.set_node_id(node_id);
                self
            }

            fn browse_name<V>(mut self, browse_name: V) -> Self
            where
                V: Into<QualifiedName>,
            {
                let _ = self.node.base.set_browse_name(browse_name);
                self
            }

            fn display_name<V>(mut self, display_name: V) -> Self
            where
                V: Into<LocalizedText>,
            {
                self.node.set_display_name(display_name.into());
                self
            }

            /// Tests that the builder is in a valid state to build or insert the node.
            pub fn is_valid(&self) -> bool {
                self.node.is_valid()
            }

            /// Sets the description of the node
            pub fn description<V>(mut self, description: V) -> Self
            where
                V: Into<LocalizedText>,
            {
                self.node.set_description(description.into());
                self
            }

            /// Adds a reference to the node
            pub fn reference<T>(
                mut self,
                node_id: T,
                reference_type_id: ReferenceTypeId,
                reference_direction: ReferenceDirection,
            ) -> Self
            where
                T: Into<NodeId>,
            {
                self.references.push((
                    node_id.into(),
                    reference_type_id.into(),
                    reference_direction,
                ));
                self
            }

            /// Indicates this node organizes another node by its id.
            pub fn organizes<T>(self, organizes_id: T) -> Self
            where
                T: Into<NodeId>,
            {
                self.reference(
                    organizes_id,
                    ReferenceTypeId::Organizes,
                    ReferenceDirection::Forward,
                )
            }

            /// Indicates this node is organised by another node by its id
            pub fn organized_by<T>(self, organized_by_id: T) -> Self
            where
                T: Into<NodeId>,
            {
                self.reference(
                    organized_by_id,
                    ReferenceTypeId::Organizes,
                    ReferenceDirection::Inverse,
                )
            }

            /// Yields a built node. This function will panic if the node is invalid. Note that
            /// calling this function discards any references for the node, so there is no purpose
            /// in adding references if you intend to call this method.
            pub fn build(self) -> $node_ty {
                if self.is_valid() {
                    self.node
                } else {
                    panic!(
                        "The node is not valid, node id = {:?}",
                        self.node.base.node_id()
                    );
                }
            }

            /// Inserts the node into the address space, including references. This function
            /// will panic if the node is in an invalid state.
            pub fn insert(self, address_space: &mut impl crate::NodeInsertTarget) -> bool {
                if self.is_valid() {
                    if !self.references.is_empty() {
                        let references = self
                            .references
                            .iter()
                            .map(|v| (&v.0, &v.1, v.2))
                            .collect::<Vec<_>>();
                        address_space.insert(self.node, Some(references.as_slice()))
                    } else {
                        address_space.insert(self.node, None)
                    }
                } else {
                    panic!(
                        "The node is not valid, node id = {:?}",
                        self.node.base.node_id()
                    );
                }
            }
        }
    };
}

macro_rules! node_builder_impl_generates_event {
    ( $node_builder_ty:ident ) => {
        impl $node_builder_ty {
            pub fn generates_event<T>(self, event_type: T) -> Self
            where
                T: Into<NodeId>,
            {
                self.reference(
                    event_type,
                    ReferenceTypeId::GeneratesEvent,
                    ReferenceDirection::Forward,
                )
            }
        }
    };
}

macro_rules! node_builder_impl_subtype {
    ( $node_builder_ty:ident ) => {
        impl $node_builder_ty {
            pub fn subtype_of<T>(self, type_id: T) -> Self
            where
                T: Into<NodeId>,
            {
                self.reference(
                    type_id,
                    ReferenceTypeId::HasSubtype,
                    ReferenceDirection::Inverse,
                )
            }

            pub fn has_subtype<T>(self, subtype_id: T) -> Self
            where
                T: Into<NodeId>,
            {
                self.reference(
                    subtype_id,
                    ReferenceTypeId::HasSubtype,
                    ReferenceDirection::Forward,
                )
            }
        }
    };
}

macro_rules! node_builder_impl_component_of {
    ( $node_builder_ty:ident ) => {
        impl $node_builder_ty {
            pub fn component_of<T>(self, component_of_id: T) -> Self
            where
                T: Into<NodeId>,
            {
                self.reference(
                    component_of_id,
                    ReferenceTypeId::HasComponent,
                    ReferenceDirection::Inverse,
                )
            }

            pub fn has_component<T>(self, has_component_id: T) -> Self
            where
                T: Into<NodeId>,
            {
                self.reference(
                    has_component_id,
                    ReferenceTypeId::HasComponent,
                    ReferenceDirection::Forward,
                )
            }
        }
    };
}

macro_rules! node_builder_impl_property_of {
    ( $node_builder_ty:ident ) => {
        impl $node_builder_ty {
            pub fn has_property<T>(self, has_component_id: T) -> Self
            where
                T: Into<NodeId>,
            {
                self.reference(
                    has_component_id,
                    ReferenceTypeId::HasProperty,
                    ReferenceDirection::Forward,
                )
            }

            pub fn property_of<T>(self, component_of_id: T) -> Self
            where
                T: Into<NodeId>,
            {
                self.reference(
                    component_of_id,
                    ReferenceTypeId::HasProperty,
                    ReferenceDirection::Inverse,
                )
            }
        }
    };
}

/// This is a sanity saving macro that implements the NodeBase trait for nodes. It assumes the
/// node has a base: Base
macro_rules! node_base_impl {
    ( $node_struct:ident ) => {
        use crate::NodeType;
        use opcua_types::{NodeClass, WriteMask};

        impl From<$node_struct> for NodeType {
            fn from(value: $node_struct) -> Self {
                Self::$node_struct(Box::new(value))
            }
        }

        impl crate::NodeBase for $node_struct {
            fn node_class(&self) -> NodeClass {
                self.base.node_class()
            }

            fn node_id(&self) -> &NodeId {
                self.base.node_id()
            }

            fn browse_name(&self) -> &QualifiedName {
                self.base.browse_name()
            }

            fn display_name(&self) -> &LocalizedText {
                self.base.display_name()
            }

            fn set_display_name(&mut self, display_name: LocalizedText) {
                self.base.set_display_name(display_name);
            }

            fn description(&self) -> Option<&LocalizedText> {
                self.base.description()
            }

            fn set_description(&mut self, description: LocalizedText) {
                self.base.set_description(description);
            }

            fn write_mask(&self) -> Option<WriteMask> {
                self.base.write_mask()
            }

            fn set_write_mask(&mut self, write_mask: WriteMask) {
                self.base.set_write_mask(write_mask);
            }

            fn user_write_mask(&self) -> Option<WriteMask> {
                self.base.user_write_mask()
            }

            fn set_user_write_mask(&mut self, user_write_mask: WriteMask) {
                self.base.set_user_write_mask(user_write_mask)
            }
        }
    };
}

mod base;
mod data_type;
// mod generated;
mod method;
mod node;
mod object;
mod object_type;
mod reference_type;
mod variable;
mod variable_type;
mod view;

bitflags! {
    pub struct AccessLevel: u8 {
        const CURRENT_READ = 1;
        const CURRENT_WRITE = 2;
        const HISTORY_READ = 4;
        const HISTORY_WRITE = 8;
        // These can be uncommented if they become used
        // const SEMANTIC_CHANGE = 16;
        // const STATUS_WRITE = 32;
        // const TIMESTAMP_WRITE = 64;
    }
}

bitflags! {
    pub struct UserAccessLevel: u8 {
        const CURRENT_READ = 1;
        const CURRENT_WRITE = 2;
        const HISTORY_READ = 4;
        const HISTORY_WRITE = 8;
        // These can be uncommented if they become used
        // const STATUS_WRITE = 32;
        // const TIMESTAMP_WRITE = 64;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct EventNotifier: u8 {
        const SUBSCRIBE_TO_EVENTS = 1;
        const HISTORY_READ = 4;
        const HISTORY_WRITE = 8;
    }
}
