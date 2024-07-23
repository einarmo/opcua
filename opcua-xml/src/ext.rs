use roxmltree::Node;

use crate::error::XmlError;

pub trait NodeExt<'a, 'input: 'a> {
    fn first_child_with_name(&self, name: &str) -> Result<Node<'a, 'input>, XmlError>;

    fn with_name(&self, name: &str) -> impl Iterator<Item = Node<'a, 'input>>;

    fn try_attribute(&self, name: &str) -> Result<&'a str, XmlError>;
}

impl<'a, 'input: 'a> NodeExt<'a, 'input> for Node<'a, 'input> {
    fn first_child_with_name(&self, name: &str) -> Result<Node<'a, 'input>, XmlError> {
        self.with_name(name)
            .next()
            .ok_or_else(|| XmlError::missing_field(self, name))
    }

    fn with_name(&self, name: &str) -> impl Iterator<Item = Node<'a, 'input>> {
        self.children().filter(move |n| n.has_tag_name(name))
    }

    fn try_attribute(&self, name: &str) -> Result<&'a str, XmlError> {
        self.attribute(name)
            .ok_or_else(|| XmlError::missing_attribute(self, name))
    }
}
