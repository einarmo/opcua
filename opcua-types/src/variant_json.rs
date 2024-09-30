use core::f32;

use serde::{
    de::{self, Visitor},
    ser::{SerializeSeq, SerializeStruct},
    Deserialize, Serialize,
};
use serde_json::{from_value, Value};

use crate::{Variant, VariantScalarTypeId};

const VALUE_INFINITY: &str = "Infinity";
const VALUE_NEG_INFINITY: &str = "-Infinity";
const VALUE_NAN: &str = "NaN";

macro_rules! ser_float {
    ($v: expr, $t: ty, $ser: expr) => {
        if *$v == <$t>::INFINITY {
            $ser.serialize_field("Body", VALUE_INFINITY)
        } else if *$v == <$t>::NEG_INFINITY {
            $ser.serialize_field("Body", VALUE_NEG_INFINITY)
        } else if $v.is_nan() {
            $ser.serialize_field("Body", VALUE_NAN)
        } else {
            $ser.serialize_field("Body", $v)
        }
    };
}

impl Serialize for Variant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Technically, null variants are supposed to be omitted if part of a JSON
        // array, but there isn't any way we can know that at this stage, so we just always
        // render them as null. The solution is that any type containing a variant
        // add skip_serialize_if(Variant::is_null) to the field in question.
        let mut len = 2;
        let type_id = match self.type_id() {
            crate::VariantTypeId::Empty => return serializer.serialize_none(),
            crate::VariantTypeId::Scalar(s) => s,
            crate::VariantTypeId::Array(s, dims) => {
                if dims.is_some_and(|d| d.len() > 1) {
                    len += 1;
                }
                s
            }
        };

        let mut struct_ser = serializer.serialize_struct("Variant", len)?;
        struct_ser.serialize_field("Type", &(type_id as u32))?;

        match self {
            Variant::Empty => unreachable!(),
            Variant::Boolean(b) => struct_ser.serialize_field("Body", b)?,
            Variant::SByte(b) => struct_ser.serialize_field("Body", b)?,
            Variant::Byte(b) => struct_ser.serialize_field("Body", b)?,
            Variant::Int16(b) => struct_ser.serialize_field("Body", b)?,
            Variant::UInt16(b) => struct_ser.serialize_field("Body", b)?,
            Variant::Int32(b) => struct_ser.serialize_field("Body", b)?,
            Variant::UInt32(b) => struct_ser.serialize_field("Body", b)?,
            Variant::Int64(b) => struct_ser.serialize_field("Body", b)?,
            Variant::UInt64(b) => struct_ser.serialize_field("Body", b)?,
            Variant::Float(b) => ser_float!(b, f32, struct_ser)?,
            Variant::Double(b) => ser_float!(b, f64, struct_ser)?,
            Variant::String(b) => struct_ser.serialize_field("Body", b)?,
            Variant::DateTime(b) => struct_ser.serialize_field("Body", b)?,
            Variant::Guid(b) => struct_ser.serialize_field("Body", b)?,
            Variant::StatusCode(b) => struct_ser.serialize_field("Body", b)?,
            Variant::ByteString(b) => struct_ser.serialize_field("Body", b)?,
            Variant::XmlElement(b) => struct_ser.serialize_field("Body", b)?,
            Variant::QualifiedName(b) => struct_ser.serialize_field("Body", b)?,
            Variant::LocalizedText(b) => struct_ser.serialize_field("Body", b)?,
            Variant::NodeId(b) => struct_ser.serialize_field("Body", b)?,
            Variant::ExpandedNodeId(b) => struct_ser.serialize_field("Body", b)?,
            Variant::ExtensionObject(b) => struct_ser.serialize_field("Body", b)?,
            Variant::Variant(b) => struct_ser.serialize_field("Body", b)?,
            Variant::DataValue(b) => struct_ser.serialize_field("Body", b)?,
            Variant::DiagnosticInfo(b) => struct_ser.serialize_field("Body", b)?,
            Variant::Array(array) => {
                if let Some(dims) = array.dimensions.as_ref() {
                    if dims.len() > 1 {
                        struct_ser.serialize_field("Dimensions", &dims)?;
                    }
                }

                struct_ser.serialize_field("Body", &VariantValueArray(&array.values))?;
            }
        };

        struct_ser.end()
    }
}

struct VariantValueArray<'a>(&'a [Variant]);

macro_rules! ser_float_seq {
    ($v: expr, $t: ty, $ser: expr) => {
        if *$v == <$t>::INFINITY {
            $ser.serialize_element(VALUE_INFINITY)
        } else if *$v == <$t>::NEG_INFINITY {
            $ser.serialize_element(VALUE_NEG_INFINITY)
        } else if $v.is_nan() {
            $ser.serialize_element(VALUE_NAN)
        } else {
            $ser.serialize_element($v)
        }
    };
}

impl<'a> Serialize for VariantValueArray<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq_ser = serializer.serialize_seq(Some(self.0.len()))?;
        for elem in self.0 {
            match elem {
                Variant::Empty => seq_ser.serialize_element(&None::<i32>)?,
                Variant::Boolean(b) => seq_ser.serialize_element(b)?,
                Variant::SByte(b) => seq_ser.serialize_element(b)?,
                Variant::Byte(b) => seq_ser.serialize_element(b)?,
                Variant::Int16(b) => seq_ser.serialize_element(b)?,
                Variant::UInt16(b) => seq_ser.serialize_element(b)?,
                Variant::Int32(b) => seq_ser.serialize_element(b)?,
                Variant::UInt32(b) => seq_ser.serialize_element(b)?,
                Variant::Int64(b) => seq_ser.serialize_element(b)?,
                Variant::UInt64(b) => seq_ser.serialize_element(b)?,
                Variant::Float(b) => ser_float_seq!(b, f32, seq_ser)?,
                Variant::Double(b) => ser_float_seq!(b, f64, seq_ser)?,
                Variant::String(b) => seq_ser.serialize_element(b)?,
                Variant::DateTime(b) => seq_ser.serialize_element(b)?,
                Variant::Guid(b) => seq_ser.serialize_element(b)?,
                Variant::StatusCode(b) => seq_ser.serialize_element(b)?,
                Variant::ByteString(b) => seq_ser.serialize_element(b)?,
                Variant::XmlElement(b) => seq_ser.serialize_element(b)?,
                Variant::QualifiedName(b) => seq_ser.serialize_element(b)?,
                Variant::LocalizedText(b) => seq_ser.serialize_element(b)?,
                Variant::NodeId(b) => seq_ser.serialize_element(b)?,
                Variant::ExpandedNodeId(b) => seq_ser.serialize_element(b)?,
                Variant::ExtensionObject(b) => seq_ser.serialize_element(b)?,
                Variant::Variant(b) => seq_ser.serialize_element(b)?,
                Variant::DataValue(b) => seq_ser.serialize_element(b)?,
                Variant::DiagnosticInfo(b) => seq_ser.serialize_element(b)?,
                Variant::Array(_) => {
                    // Should be impossible, just write null
                    seq_ser.serialize_element(&None::<i32>)?
                }
            }
        }
        seq_ser.end()
    }
}

struct VariantVisitor;

#[derive(Serialize, Deserialize)]
struct JsonVariant {
    #[serde(rename = "Type")]
    variant_type: u32,
    #[serde(rename = "Body")]
    body: Option<serde_json::Value>,
    #[serde(rename = "Dimensions")]
    dimensions: Option<Vec<u32>>,
}

macro_rules! from_value {
    ($body:expr, $t:ident, $dims:expr, $m:ident) => {
        match $body {
            Some(serde_json::Value::Array(arr)) => {
                let values = arr
                    .into_iter()
                    .map(|v| $m(v).map_err(de::Error::custom))
                    .collect::<Result<Vec<_>, _>>()?;
                Variant::Array(Box::new(crate::Array {
                    value_type: VariantScalarTypeId::$t,
                    values: values.into_iter().map(Variant::$t).collect(),
                    dimensions: $dims,
                }))
            }
            Some(r) => {
                if $dims.is_some() {
                    return Err(de::Error::custom("Unexpected dimensions for scalar value"));
                }
                Variant::$t($m(r).map_err(de::Error::custom)?)
            }
            None => Variant::$t(Default::default()),
        }
    };
}

fn parse_f32(value: Value) -> Result<f32, String> {
    match value {
        Value::Number(number) => {
            let Some(v) = number.as_f64() else {
                return Err("Invalid float".to_string());
            };
            Ok(v as f32)
        }
        Value::String(s) => match s.as_str() {
            VALUE_INFINITY => Ok(f32::INFINITY),
            VALUE_NEG_INFINITY => Ok(f32::NEG_INFINITY),
            VALUE_NAN => Ok(f32::NAN),
            r => Err(format!("Unexpected value for float: {r}")),
        },
        _ => Err("Expected string or float".to_owned()),
    }
}

fn parse_f64(value: Value) -> Result<f64, String> {
    match value {
        Value::Number(number) => {
            let Some(v) = number.as_f64() else {
                return Err("Invalid float".to_string());
            };
            Ok(v as f64)
        }
        Value::String(s) => match s.as_str() {
            VALUE_INFINITY => Ok(f64::INFINITY),
            VALUE_NEG_INFINITY => Ok(f64::NEG_INFINITY),
            VALUE_NAN => Ok(f64::NAN),
            r => Err(format!("Unexpected value for float: {r}")),
        },
        _ => Err("Expected string or float".to_owned()),
    }
}

impl<'de> Visitor<'de> for VariantVisitor {
    type Value = Variant;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a variant value or null")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Variant::Empty)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = JsonVariant::deserialize(deserializer)?;

        if v.variant_type == 0 {
            if v.body.is_some() {
                return Err(de::Error::custom("Unexpected body with type id 0"));
            }
            return Ok(Variant::Empty);
        }

        let type_id = VariantScalarTypeId::try_from(v.variant_type).map_err(|_| {
            de::Error::custom(&format!("Unexpected variant type {}", v.variant_type))
        })?;

        Ok(match type_id {
            VariantScalarTypeId::Boolean => from_value!(v.body, Boolean, v.dimensions, from_value),
            VariantScalarTypeId::SByte => from_value!(v.body, SByte, v.dimensions, from_value),
            VariantScalarTypeId::Byte => from_value!(v.body, Byte, v.dimensions, from_value),
            VariantScalarTypeId::Int16 => from_value!(v.body, Int16, v.dimensions, from_value),
            VariantScalarTypeId::UInt16 => from_value!(v.body, UInt16, v.dimensions, from_value),
            VariantScalarTypeId::Int32 => from_value!(v.body, Int32, v.dimensions, from_value),
            VariantScalarTypeId::UInt32 => from_value!(v.body, UInt32, v.dimensions, from_value),
            VariantScalarTypeId::Int64 => from_value!(v.body, Int64, v.dimensions, from_value),
            VariantScalarTypeId::UInt64 => from_value!(v.body, UInt64, v.dimensions, from_value),
            VariantScalarTypeId::Float => from_value!(v.body, Float, v.dimensions, parse_f32),
            VariantScalarTypeId::Double => from_value!(v.body, Double, v.dimensions, parse_f64),
            VariantScalarTypeId::String => from_value!(v.body, String, v.dimensions, from_value),
            VariantScalarTypeId::DateTime => {
                from_value!(v.body, DateTime, v.dimensions, from_value)
            }
            VariantScalarTypeId::Guid => from_value!(v.body, Guid, v.dimensions, from_value),
            VariantScalarTypeId::ByteString => {
                from_value!(v.body, ByteString, v.dimensions, from_value)
            }
            VariantScalarTypeId::XmlElement => {
                from_value!(v.body, XmlElement, v.dimensions, from_value)
            }
            VariantScalarTypeId::NodeId => from_value!(v.body, NodeId, v.dimensions, from_value),
            VariantScalarTypeId::ExpandedNodeId => {
                from_value!(v.body, ExpandedNodeId, v.dimensions, from_value)
            }
            VariantScalarTypeId::StatusCode => {
                from_value!(v.body, StatusCode, v.dimensions, from_value)
            }
            VariantScalarTypeId::QualifiedName => {
                from_value!(v.body, QualifiedName, v.dimensions, from_value)
            }
            VariantScalarTypeId::LocalizedText => {
                from_value!(v.body, LocalizedText, v.dimensions, from_value)
            }
            VariantScalarTypeId::ExtensionObject => {
                from_value!(v.body, ExtensionObject, v.dimensions, from_value)
            }
            VariantScalarTypeId::DataValue => {
                from_value!(v.body, DataValue, v.dimensions, from_value)
            }
            VariantScalarTypeId::Variant => from_value!(v.body, Variant, v.dimensions, from_value),
            VariantScalarTypeId::DiagnosticInfo => {
                from_value!(v.body, DiagnosticInfo, v.dimensions, from_value)
            }
        })
    }
}

impl<'de> Deserialize<'de> for Variant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_option(VariantVisitor)
    }
}
