use super::{
    DecodingOptions, EnumDefinition, ExtensionObject, ObjectId, StatusCode, StructureDefinition,
    Variant,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
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

impl DataTypeDefinition {
    pub fn from_extension_object(
        obj: ExtensionObject,
        options: &DecodingOptions,
    ) -> Result<Self, StatusCode> {
        match obj.node_id.as_object_id() {
            Ok(ObjectId::StructureDefinition_Encoding_DefaultBinary) => {
                Ok(Self::Structure(obj.decode_inner(options)?))
            }
            Ok(ObjectId::EnumDefinition_Encoding_DefaultBinary) => {
                Ok(Self::Enum(obj.decode_inner(options)?))
            }
            _ => Err(StatusCode::BadDataTypeIdUnknown),
        }
    }

    pub fn as_extension_object(&self) -> ExtensionObject {
        match self {
            DataTypeDefinition::Structure(s) => ExtensionObject::from_encodable(
                ObjectId::StructureDefinition_Encoding_DefaultBinary,
                s,
            ),
            DataTypeDefinition::Enum(s) => {
                ExtensionObject::from_encodable(ObjectId::EnumDefinition_Encoding_DefaultBinary, s)
            }
        }
    }
}

impl From<&DataTypeDefinition> for Variant {
    fn from(value: &DataTypeDefinition) -> Self {
        value.as_extension_object().into()
    }
}
