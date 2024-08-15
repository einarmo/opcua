use roxmltree::Node;

use crate::{error::XmlError, FromValue, XmlLoad};

pub trait NodeExt<'a, 'input: 'a> {
    fn first_child_with_name(&self, name: &str) -> Result<Node<'a, 'input>, XmlError>;

    fn with_name(&self, name: &str) -> impl Iterator<Item = Node<'a, 'input>>;

    fn try_attribute(&self, name: &str) -> Result<&'a str, XmlError>;

    fn try_contents(&self) -> Result<&'a str, XmlError>;
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

    fn try_contents(&self) -> Result<&'a str, XmlError> {
        self.text().ok_or_else(|| XmlError::missing_content(self))
    }
}

pub fn children_with_name<'input, T: XmlLoad<'input>>(
    node: &Node<'_, 'input>,
    name: &str,
) -> Result<Vec<T>, XmlError> {
    node.with_name(name).map(|e| T::load(&e)).collect()
}

#[allow(unused)]
pub fn first_child_with_name<'input, T: XmlLoad<'input>>(
    node: &Node<'_, 'input>,
    name: &str,
) -> Result<T, XmlError> {
    T::load(&node.first_child_with_name(name)?)
}

pub fn first_child_with_name_opt<'input, T: XmlLoad<'input>>(
    node: &Node<'_, 'input>,
    name: &str,
) -> Result<Option<T>, XmlError> {
    let Ok(child) = node.first_child_with_name(name) else {
        return Ok(None);
    };
    T::load(&child).map(|v| Some(v))
}

pub fn uint_attr<'input>(node: &Node<'_, 'input>, name: &str) -> Result<Option<u64>, XmlError> {
    node.attribute(name)
        .map(|a| a.parse())
        .transpose()
        .map_err(|e| XmlError::parse_int(node, name, e))
}

pub fn int_attr<'input>(node: &Node<'_, 'input>, name: &str) -> Result<Option<i64>, XmlError> {
    node.attribute(name)
        .map(|a| a.parse())
        .transpose()
        .map_err(|e| XmlError::parse_int(node, name, e))
}

pub fn value_from_contents<T: FromValue>(node: &Node<'_, '_>) -> Result<T, XmlError> {
    T::from_value(node, "content", node.try_contents()?)
}

pub fn value_from_attr<T: FromValue>(node: &Node<'_, '_>, attr: &str) -> Result<T, XmlError> {
    T::from_value(node, attr, node.try_attribute(attr)?)
}

#[allow(unused)]
pub fn value_from_contents_opt<T: FromValue>(node: &Node<'_, '_>) -> Result<Option<T>, XmlError> {
    let Some(c) = node.text() else {
        return Ok(None);
    };

    T::from_value(node, "content", c).map(Some)
}

pub fn value_from_attr_opt<T: FromValue>(
    node: &Node<'_, '_>,
    attr: &str,
) -> Result<Option<T>, XmlError> {
    let Some(c) = node.attribute(attr) else {
        return Ok(None);
    };

    T::from_value(node, attr, c).map(Some)
}
