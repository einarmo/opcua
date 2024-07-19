use roxmltree::Node;

use crate::error::CodeGenError;

pub trait NodeExt<'a, 'input: 'a> {
    fn first_child_with_name<'b>(
        &self,
        name: &'b str,
    ) -> Result<Node<'a, 'input>, CodeGenError<'b>>;

    fn with_name<'b>(&self, name: &'b str) -> impl Iterator<Item = Node<'a, 'input>>;

    fn try_attribute<'b>(&self, name: &'b str) -> Result<&'a str, CodeGenError<'b>>;

    fn try_child_contents<'b>(&self, name: &'b str) -> Result<&'a str, CodeGenError<'b>>;

    fn child_contents<'b>(&self, name: &'b str) -> Option<&'a str>;
}

impl<'a, 'input: 'a> NodeExt<'a, 'input> for Node<'a, 'input> {
    fn first_child_with_name<'b>(
        &self,
        name: &'b str,
    ) -> Result<Node<'a, 'input>, CodeGenError<'b>> {
        self.with_name(name)
            .next()
            .ok_or_else(|| CodeGenError::MissingField(name))
    }

    fn with_name<'b>(&self, name: &'b str) -> impl Iterator<Item = Node<'a, 'input>> {
        self.children().filter(move |n| n.has_tag_name(name))
    }

    fn try_attribute<'b>(&self, name: &'b str) -> Result<&'a str, CodeGenError<'b>> {
        self.attribute(name)
            .ok_or_else(|| CodeGenError::MissingAttribute(name))
    }

    fn try_child_contents<'b>(&self, name: &'b str) -> Result<&'a str, CodeGenError<'b>> {
        self.child_contents(name)
            .ok_or_else(|| CodeGenError::MissingField(name))
    }

    fn child_contents<'b>(&self, name: &'b str) -> Option<&'a str> {
        self.first_child_with_name(name).ok().and_then(|r| r.text())
    }
}
