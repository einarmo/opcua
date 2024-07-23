use roxmltree::Node;

mod error;
mod ext;
pub mod schema;

pub use error::XmlError;
pub use schema::load_bsd_file;

pub trait XmlLoad<'input>: Sized {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError>;
}
