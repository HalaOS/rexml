#![allow(unused)]

use std::{borrow::Cow, fmt::Display};

use crate::node::DOMLeaf;

/// This corresponds to the namespace extension.
#[derive(Debug)]
pub(crate) struct DOMNamespace<'a> {
    leaf: DOMLeaf,
    /// The namespace prefix
    prefix: Cow<'a, str>,
    /// The namespace href
    href: Cow<'a, str>,
}

impl<'a> DOMNamespace<'a> {
    #[allow(unused)]
    pub(crate) fn new(prefix: Cow<'a, str>, href: Cow<'a, str>) -> Self {
        Self {
            leaf: Default::default(),
            prefix,
            href,
        }
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

impl<'a> Display for DOMNamespace<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Namespace({:?},{},{})",
            self.leaf, self.prefix, self.href,
        )
    }
}

impl<'a> DOMNamespace<'a> {
    /// Returns a new owning QName from the given existing one.
    pub fn into_owned(self) -> DOMNamespace<'static> {
        DOMNamespace::<'static> {
            leaf: self.leaf,
            prefix: self.prefix.into_owned().into(),
            href: self.href.into_owned().into(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_namespace() {
        println!(
            "{}",
            DOMNamespace::new("xsl".into(), "http://www.w3.org/1999/XSL/Transform".into())
        );
    }
}
