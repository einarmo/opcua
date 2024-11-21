use crate::{match_extension_object_owned, MessageInfo};

use super::{EnumDefinition, ExtensionObject, ObjectId, StatusCode, StructureDefinition, Variant};

#[derive(Debug, Clone)]
pub enum DataTypeDefinition {
    Structure(StructureDefinition),
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
}

impl DataTypeDefinition {
    pub fn from_extension_object(obj: ExtensionObject) -> Result<Self, StatusCode> {
        match_extension_object_owned!(obj,
            v: StructureDefinition => Ok(Self::Structure(v)),
            v: EnumDefinition => Ok(Self::Enum(v)),
            _ => Err(StatusCode::BadDataTypeIdUnknown)
        )
    }

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
