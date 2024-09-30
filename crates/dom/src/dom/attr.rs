use std::borrow::Cow;

use crate::QName;

use super::{impl_xml_leaf, XmlLeaf};

/// This corresponds to the DOM `Attribute` interface.
#[derive(Debug)]
pub struct Attribute<'a> {
    /// Mixin DOM leaf.
    leaf: XmlLeaf,
    /// Attribute qualified name,
    name: QName<'a>,
    /// Value part of this attribute.
    value: Cow<'a, str>,
}

impl<'a> Attribute<'a> {
    #[allow(unused)]
    pub(super) fn new(name: QName<'a>, value: Cow<'a, str>) -> Self {
        Self {
            leaf: Default::default(),
            name,
            value,
        }
    }

    /// Returns the node name,
    pub fn name(&self) -> &QName<'a> {
        &self.name
    }

    /// Returns this attribute value part.
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl_xml_leaf!(Attribute);
