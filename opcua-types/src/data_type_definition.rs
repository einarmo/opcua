//! Implementation of the [`DataTypeDefinition`] enum, and some utilities related to this.

use crate::{match_extension_object_owned, DataTypeId, MessageInfo};

use super::{EnumDefinition, ExtensionObject, ObjectId, StatusCode, StructureDefinition, Variant};

#[derive(Debug, Clone)]
/// Type for an OPC UA data type definition.
pub enum DataTypeDefinition {
    /// Structure definition.
    Structure(StructureDefinition),
    /// Enum definition.
    Enum(EnumDefinition),
}

impl From<StructureDefinition> for DataTypeDefinition {
    fn from(value: StructureDefinition) -> Self {
        Self::Structure(value)
    }
}

impl From<EnumDefinition> for DataTypeDefinition {
    fn from(value: EnumDefinition) -> Self {
        Self::Enum(value)
    }
}

// TODO: Figure out why we don't auto generate these.
impl MessageInfo for StructureDefinition {
    fn type_id(&self) -> ObjectId {
        ObjectId::StructureDefinition_Encoding_DefaultBinary
    }

    fn json_type_id(&self) -> ObjectId {
        ObjectId::StructureDefinition_Encoding_DefaultJson
    }

    fn xml_type_id(&self) -> ObjectId {
        ObjectId::StructureDefinition_Encoding_DefaultXml
    }

    fn data_type_id(&self) -> crate::DataTypeId {
        DataTypeId::StructureDefinition
    }
}

impl MessageInfo for EnumDefinition {
    fn type_id(&self) -> ObjectId {
        ObjectId::EnumDefinition_Encoding_DefaultBinary
    }

    fn json_type_id(&self) -> ObjectId {
        ObjectId::EnumDefinition_Encoding_DefaultJson
    }

    fn xml_type_id(&self) -> ObjectId {
        ObjectId::EnumDefinition_Encoding_DefaultXml
    }

    fn data_type_id(&self) -> DataTypeId {
        DataTypeId::EnumDefinition
    }
}

impl DataTypeDefinition {
    /// Try to get a data type definition from the body of an extension object.
    pub fn from_extension_object(obj: ExtensionObject) -> Result<Self, StatusCode> {
        match_extension_object_owned!(obj,
            v: StructureDefinition => Ok(Self::Structure(v)),
            v: EnumDefinition => Ok(Self::Enum(v)),
            _ => Err(StatusCode::BadDataTypeIdUnknown)
        )
    }

    /// Create an extension object from this.
    pub fn into_extension_object(self) -> ExtensionObject {
        match self {
            DataTypeDefinition::Structure(s) => ExtensionObject::from_message(s),
            DataTypeDefinition::Enum(s) => ExtensionObject::from_message(s),
        }
    }
}

impl From<DataTypeDefinition> for Variant {
    fn from(value: DataTypeDefinition) -> Self {
        value.into_extension_object().into()
    }
}
