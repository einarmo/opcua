use ext::NodeExt;
use roxmltree::Node;

mod error;
mod ext;
pub mod schema;

pub use error::XmlError;
pub use schema::opc_binary_schema::load_bsd_file;
pub use schema::ua_node_set::load_nodeset2_file;

pub trait XmlLoad<'input>: Sized {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError>;
}

pub trait FromValue: Sized {
    fn from_value(node: &Node<'_, '_>, attr: &str, v: &str) -> Result<Self, XmlError>;
}

macro_rules! from_int {
    ($ty:ident) => {
        impl FromValue for $ty {
            fn from_value(node: &Node<'_, '_>, attr: &str, v: &str) -> Result<Self, XmlError> {
                v.parse().map_err(|e| XmlError::parse_int(node, attr, e))
            }
        }
    };
}

from_int!(i64);
from_int!(u64);
from_int!(i32);
from_int!(u32);
from_int!(i16);
from_int!(u16);
from_int!(i8);
from_int!(u8);

impl FromValue for String {
    fn from_value(_node: &Node<'_, '_>, _attr: &str, v: &str) -> Result<Self, XmlError> {
        Ok(v.to_owned())
    }
}

impl FromValue for f64 {
    fn from_value(node: &Node<'_, '_>, attr: &str, v: &str) -> Result<Self, XmlError> {
        v.parse().map_err(|e| XmlError::parse_float(node, attr, e))
    }
}

impl FromValue for f32 {
    fn from_value(node: &Node<'_, '_>, attr: &str, v: &str) -> Result<Self, XmlError> {
        v.parse().map_err(|e| XmlError::parse_float(node, attr, e))
    }
}

impl FromValue for bool {
    fn from_value(node: &Node<'_, '_>, attr: &str, v: &str) -> Result<Self, XmlError> {
        v.parse().map_err(|e| XmlError::parse_bool(node, attr, e))
    }
}

impl<'input, T> XmlLoad<'input> for T
where
    T: FromValue,
{
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        T::from_value(node, "content", node.try_contents()?)
    }
}
