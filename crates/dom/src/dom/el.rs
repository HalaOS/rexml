use crate::{NodeRef, QName};

use super::{impl_xml_node, Attribute, XmlNode};

///  This corresponds to the DOM element interface.
pub struct Element<'a> {
    tree: XmlNode,
    /// qualified name of tag.
    qname: QName<'a>,
    /// The attribute list.
    attrs: Vec<Attribute<'a>>,
}

impl_xml_node!(Element);

impl<'a> Element<'a> {
    #[allow(unused)]
    fn new(qname: QName<'a>, parent: Option<NodeRef>) -> Self {
        Self {
            tree: Default::default(),
            qname,
            attrs: Default::default(),
        }
    }

    /// Get the element tag's qualified name.
    pub fn qname(&self) -> &QName<'a> {
        &self.qname
    }

    /// Returns a iterator over the attributes list.
    pub fn attrs(&self) -> impl Iterator<Item = &Attribute<'a>> {
        self.attrs.iter()
    }
}
