use super::{
    extension_object::ExtensionObject,
    service_types::{
        DataTypeAttributes, GenericAttributes, MethodAttributes, ObjectAttributes,
        ObjectTypeAttributes, ReferenceTypeAttributes, VariableAttributes, VariableTypeAttributes,
        ViewAttributes,
    },
    status_code::StatusCode,
};

#[derive(Clone, Debug)]
pub enum AddNodeAttributes {
    Object(ObjectAttributes),
    Variable(VariableAttributes),
    Method(MethodAttributes),
    ObjectType(ObjectTypeAttributes),
    VariableType(VariableTypeAttributes),
    ReferenceType(ReferenceTypeAttributes),
    DataType(DataTypeAttributes),
    View(ViewAttributes),
    Generic(GenericAttributes),
    None,
}

impl AddNodeAttributes {
    pub fn from_extension_object(obj: ExtensionObject) -> Result<Self, StatusCode> {
        if obj.is_null() {
            return Ok(Self::None);
        }
        if let Some(v) = obj.inner_as() {
            Ok(Self::Object(*v))
        } else if let Some(v) = obj.inner_as() {
            Ok(Self::Method(*v))
        } else if let Some(v) = obj.inner_as() {
            Ok(Self::ObjectType(*v))
        } else if let Some(v) = obj.inner_as() {
            Ok(Self::VariableType(*v))
        } else if let Some(v) = obj.inner_as() {
            Ok(Self::ReferenceType(*v))
        } else if let Some(v) = obj.inner_as() {
            Ok(Self::DataType(*v))
        } else if let Some(v) = obj.inner_as() {
            Ok(Self::View(*v))
        } else if let Some(v) = obj.inner_as() {
            Ok(Self::Generic(*v))
        } else {
            Err(StatusCode::BadNodeAttributesInvalid)
        }
    }

    pub fn as_extension_object(&self) -> ExtensionObject {
        match self.clone() {
            AddNodeAttributes::Object(o) => ExtensionObject::from_message(o),
            AddNodeAttributes::Variable(o) => ExtensionObject::from_message(o),
            AddNodeAttributes::Method(o) => ExtensionObject::from_message(o),
            AddNodeAttributes::ObjectType(o) => ExtensionObject::from_message(o),
            AddNodeAttributes::VariableType(o) => ExtensionObject::from_message(o),
            AddNodeAttributes::ReferenceType(o) => ExtensionObject::from_message(o),
            AddNodeAttributes::DataType(o) => ExtensionObject::from_message(o),
            AddNodeAttributes::View(o) => ExtensionObject::from_message(o),
            AddNodeAttributes::Generic(o) => ExtensionObject::from_message(o),
            AddNodeAttributes::None => ExtensionObject::null(),
        }
    }
}
