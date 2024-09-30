use std::{borrow::Cow, fmt::Display};

use crate::DOMObject;

/// This corresponds to the namespace extension.
#[derive(Debug)]
pub struct Namespace<'a> {
    /// The node to which this namespace belongs.
    node: DOMObject,
    /// The namespace prefix
    prefix: Cow<'a, str>,
    /// The namespace href
    href: Cow<'a, str>,
}

impl<'a> Namespace<'a> {
    #[allow(unused)]
    fn new(node: DOMObject, prefix: Cow<'a, str>, href: Cow<'a, str>) -> Self {
        Self { node, prefix, href }
    }

    /// Return namespace's parent node.
    pub fn parent(&self) -> &DOMObject {
        &self.node
    }

    /// Returns the prefix part of namespace
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Returns the href part of namespace
    pub fn href(&self) -> &str {
        &self.href
    }
}

impl<'a> Display for Namespace<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Namespace({},{},{})", self.node, self.prefix, self.href,)
    }
}

#[cfg(test)]
mod tests {

    use crate::NodeType;

    use super::*;

    #[test]
    fn test_namespace() {
        println!(
            "{}",
            Namespace::new(
                DOMObject::new(1, NodeType::Element),
                "xsl".into(),
                "http://www.w3.org/1999/XSL/Transform".into()
            )
        );
    }
}
