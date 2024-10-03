//! This mod provide a DOM implementation with arena memory managerment.

use std::{borrow::Cow, slice::Iter};

use crate::{DOMObject, Error, ExceptionCode, NodeType, QName, Result};

/// Use by gc process.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ArenaGcState {
    Unmark,
    Marked,
}

impl Default for ArenaGcState {
    fn default() -> Self {
        Self::Unmark
    }
}

/// A DOM node is allocated by and belongs to one [`DOMArena`]
#[derive(Default)]
struct ArenaNode {
    gc_state: ArenaGcState,
    #[allow(unused)]
    parent: Option<DOMObject>,
    children: Vec<DOMObject>,
}

impl ArenaNode {
    fn append_child(&mut self, child: DOMObject) {
        self.children.push(child);
    }
    fn remove_child(&mut self, child: &DOMObject) {
        if let Some(index) =
            self.children.iter().enumerate().find_map(
                |(index, obj)| {
                    if obj == child {
                        Some(index)
                    } else {
                        None
                    }
                },
            )
        {
            self.children.remove(index);
        }
    }

    fn first_child_of(&self, node_type: &NodeType) -> Option<&DOMObject> {
        self.children
            .iter()
            .find(|obj| obj.node_type() == *node_type)
    }

    fn gc_mark(&mut self) {
        self.gc_state = ArenaGcState::Marked;
    }

    fn check_gc_state(&mut self) -> bool {
        if self.gc_state == ArenaGcState::Marked {
            self.gc_state = ArenaGcState::Unmark;
            true
        } else {
            false
        }
    }
}

/// This corresponds to the namespace extension.
pub struct ArenaNamespace<'a> {
    /// object reference.
    object: DOMObject,
    /// mxin node.
    node: ArenaNode,
    /// The namespace prefix
    prefix: Cow<'a, str>,
    /// The namespace href
    href: Cow<'a, str>,
}

#[allow(unused)]
impl<'a> ArenaNamespace<'a> {
    fn new(object: DOMObject, prefix: Cow<'a, str>, href: Cow<'a, str>) -> Self {
        Self {
            prefix,
            href,
            object,
            node: Default::default(),
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

/// Attribute of one element.
pub struct ArenaAttr<'a> {
    object: DOMObject,
    node: ArenaNode,
    name: QName<'a>,
    value: Cow<'a, str>,
}

impl<'a> ArenaAttr<'a> {
    fn new(object: DOMObject, name: QName<'a>, value: Cow<'a, str>) -> Self {
        Self {
            node: Default::default(),
            object,
            name,
            value,
        }
    }

    /// Returns the name of this attribute.
    pub fn name(&self) -> &QName<'a> {
        &self.name
    }

    /// Returns the this attribute as string.
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// `Element` allocated by one `ArenaDocument`.
pub struct ArenaElement<'a> {
    object: DOMObject,
    node: ArenaNode,
    tag: QName<'a>,
}

impl<'a> AsRef<DOMObject> for ArenaElement<'a> {
    fn as_ref(&self) -> &DOMObject {
        &self.object
    }
}

impl<'a> ArenaElement<'a> {
    fn new(object: DOMObject, tag: QName<'a>) -> Self {
        Self {
            object,
            tag,
            node: Default::default(),
        }
    }

    fn append_child(&mut self, child: DOMObject) -> Result<()> {
        self.remove_child(&child)?;
        self.node.append_child(child);

        Ok(())
    }

    fn remove_child(&mut self, child: &DOMObject) -> Result<()> {
        match child.node_type() {
            NodeType::Element
            | NodeType::Text
            | NodeType::Comment
            | NodeType::CData
            | NodeType::ProcessingInstruction
            | NodeType::EntityReference
            | NodeType::Attribute
            | NodeType::Namespace => {
                self.node.remove_child(child);
                Ok(())
            }
            _ => return Err(Error::DOMException(ExceptionCode::HIERARCHY_REQUEST_ERR)),
        }
    }

    /// Returns the element's tag name.
    pub fn tag(&self) -> &QName<'a> {
        &self.tag
    }
}

/// `ProcessingInstruction` allocated by one `ArenaDocument`.
pub struct ArenaProcessingInstruction<'a> {
    object: DOMObject,
    node: ArenaNode,
    target: Cow<'a, str>,
    data: Cow<'a, str>,
}

impl<'a> AsRef<DOMObject> for ArenaProcessingInstruction<'a> {
    fn as_ref(&self) -> &DOMObject {
        &self.object
    }
}

impl<'a> ArenaProcessingInstruction<'a> {
    fn new(object: DOMObject, target: Cow<'a, str>, data: Cow<'a, str>) -> Self {
        Self {
            object,
            node: Default::default(),
            target,
            data,
        }
    }

    /// Returns the target as str
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Returns the data as str
    pub fn data(&self) -> &str {
        &self.data
    }
}

/// `Comment` allocated by one `ArenaDocument`.
pub struct ArenaComment<'a> {
    object: DOMObject,
    node: ArenaNode,
    data: Cow<'a, str>,
}

impl<'a> AsRef<DOMObject> for ArenaComment<'a> {
    fn as_ref(&self) -> &DOMObject {
        &self.object
    }
}

impl<'a> ArenaComment<'a> {
    fn new(object: DOMObject, data: Cow<'a, str>) -> Self {
        Self {
            object,
            node: Default::default(),

            data,
        }
    }

    /// Returns the data as str
    pub fn data(&self) -> &str {
        &self.data
    }
}

/// A DOM `Document` implementation with arena memory managerment.
#[derive(Default)]
pub struct ArenaDocument<'a> {
    this_node: ArenaNode,
    els: Vec<ArenaElement<'a>>,
    attrs: Vec<ArenaAttr<'a>>,
    nss: Vec<ArenaNamespace<'a>>,
    pis: Vec<ArenaProcessingInstruction<'a>>,
    cms: Vec<ArenaComment<'a>>,
}

impl<'a> ArenaDocument<'a> {
    fn gc_mark(&mut self, object: &DOMObject) {
        match object.node_type() {
            NodeType::Element => {
                self.element_mut(object).unwrap().node.gc_mark();
            }
            _ => unimplemented!(),
        }
    }

    /// Append a new child to the `Document` node.
    fn append_to_document(&mut self, child: DOMObject) -> Result<()> {
        match child.node_type() {
            NodeType::Element | NodeType::DocumentType => {
                if self.this_node.first_child_of(&child.node_type()).is_some() {
                    return Err(Error::DOMException(ExceptionCode::HIERARCHY_REQUEST_ERR));
                }

                self.this_node.remove_child(&child);
                self.this_node.append_child(child);

                Ok(())
            }
            _ => {
                return Err(Error::DOMException(ExceptionCode::HIERARCHY_REQUEST_ERR));
            }
        }
    }

    fn append_child_check(&mut self, child: &DOMObject) -> Result<()> {
        match child.node_type() {
            NodeType::Element => {
                if self
                    .element(child)
                    .ok_or(Error::DOMException(ExceptionCode::NOT_FOUND_ERR))?
                    .node
                    .parent
                    .is_some()
                {
                    return Err(Error::DOMException(ExceptionCode::HIERARCHY_REQUEST_ERR));
                }
            }
            NodeType::Attribute => {
                if self
                    .attr(child)
                    .ok_or(Error::DOMException(ExceptionCode::NOT_FOUND_ERR))?
                    .node
                    .parent
                    .is_some()
                {
                    return Err(Error::DOMException(ExceptionCode::HIERARCHY_REQUEST_ERR));
                }
            }
            NodeType::Namespace => {
                if self
                    .ns(child)
                    .ok_or(Error::DOMException(ExceptionCode::NOT_FOUND_ERR))?
                    .node
                    .parent
                    .is_some()
                {
                    return Err(Error::DOMException(ExceptionCode::HIERARCHY_REQUEST_ERR));
                }
            }
            NodeType::Comment => {
                if self
                    .comment(child)
                    .ok_or(Error::DOMException(ExceptionCode::NOT_FOUND_ERR))?
                    .node
                    .parent
                    .is_some()
                {
                    return Err(Error::DOMException(ExceptionCode::HIERARCHY_REQUEST_ERR));
                }
            }
            NodeType::ProcessingInstruction => {
                if self
                    .pi(child)
                    .ok_or(Error::DOMException(ExceptionCode::NOT_FOUND_ERR))?
                    .node
                    .parent
                    .is_some()
                {
                    return Err(Error::DOMException(ExceptionCode::HIERARCHY_REQUEST_ERR));
                }
            }
            _ => unimplemented!(),
        }

        Ok(())
    }

    /// Attach a new child to one element node.
    fn append_element(&mut self, parent: &DOMObject, child: DOMObject) -> Result<()> {
        assert_eq!(parent.node_type(), NodeType::Element);

        self.els
            .iter_mut()
            .find(|el| el.object == *parent)
            .ok_or(Error::DOMException(ExceptionCode::NOT_FOUND_ERR))?
            .append_child(child)?;

        Ok(())
    }
}

impl<'a> ArenaDocument<'a> {
    /// Returns nodes allocated by this `Document` includes unused ones.
    pub fn allocated(&self) -> usize {
        self.els.len() + self.nss.len() + self.attrs.len()
    }

    /// Free unused allocated nodes.
    pub fn gc(&mut self) {
        // mark used nodes.

        let mut stack = self.this_node.children.clone();

        // process DFS
        while let Some(top) = stack.pop() {
            self.gc_mark(&top);

            let mut children = self.children(Some(&top)).cloned().collect::<Vec<_>>();

            stack.append(&mut children);
        }

        // check elements.
        let mut els = vec![];

        for mut el in self.els.drain(..) {
            if el.node.check_gc_state() {
                els.push(el);
            }
        }

        self.els = els;

        // check attrs.
        let mut attrs = vec![];

        for mut attr in self.attrs.drain(..) {
            if attr.node.check_gc_state() {
                attrs.push(attr);
            }
        }

        self.attrs = attrs;

        // check namespaces.
        let mut nss = vec![];

        for mut ns in self.nss.drain(..) {
            if ns.node.check_gc_state() {
                nss.push(ns);
            }
        }

        self.nss = nss;

        // check ProcessingInstructions.
        let mut pis = vec![];

        for mut pi in self.pis.drain(..) {
            if pi.node.check_gc_state() {
                pis.push(pi);
            }
        }

        self.pis = pis;

        // check Comment list.
        let mut cms = vec![];

        for mut cm in self.cms.drain(..) {
            if cm.node.check_gc_state() {
                cms.push(cm);
            }
        }

        self.cms = cms;
    }

    /// Create a new `Element` node.
    pub fn create_element<T>(&mut self, tag: T) -> Result<DOMObject>
    where
        T: TryInto<QName<'a>>,
        Error: From<T::Error>,
    {
        let object = DOMObject::new(self.els.len(), NodeType::Element);

        let el = ArenaElement::new(object.clone(), tag.try_into()?);

        self.els.push(el);

        Ok(object)
    }

    /// Create a new `Attr` node.
    pub fn create_attr<T, V>(&mut self, tag: T, value: V) -> Result<DOMObject>
    where
        T: TryInto<QName<'a>>,
        Error: From<T::Error>,
        V: Into<Cow<'a, str>>,
    {
        let object = DOMObject::new(self.els.len(), NodeType::Attribute);

        let attr = ArenaAttr::new(object.clone(), tag.try_into()?, value.into());

        self.attrs.push(attr);

        Ok(object)
    }

    /// Create a new `Namespace` node.
    pub fn create_ns<P, H>(&mut self, prefix: P, href: H) -> Result<DOMObject>
    where
        P: Into<Cow<'a, str>>,
        H: Into<Cow<'a, str>>,
    {
        let object = DOMObject::new(self.els.len(), NodeType::Namespace);

        let ns = ArenaNamespace::new(object.clone(), prefix.into(), href.into());

        self.nss.push(ns);

        Ok(object)
    }

    /// Create a new `ProcessingInstruction` node.
    pub fn create_pi<T, D>(&mut self, target: T, data: D) -> Result<DOMObject>
    where
        T: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>,
    {
        let object = DOMObject::new(self.els.len(), NodeType::ProcessingInstruction);

        let pi = ArenaProcessingInstruction::new(object.clone(), target.into(), data.into());

        self.pis.push(pi);

        Ok(object)
    }

    /// Create a new `Comment` node.
    pub fn create_comment<D>(&mut self, data: D) -> Result<DOMObject>
    where
        D: Into<Cow<'a, str>>,
    {
        let object = DOMObject::new(self.els.len(), NodeType::Comment);

        let cm = ArenaComment::new(object.clone(), data.into());

        self.cms.push(cm);

        Ok(object)
    }

    /// Attach a new child to the parent node.
    ///
    /// If the parent is none, the new child will be attached to root node.
    pub fn append_child(&mut self, parent: Option<&DOMObject>, child: DOMObject) -> Result<()> {
        self.append_child_check(&child)?;

        if parent.is_none() {
            return self.append_to_document(child);
        }

        let parent = parent.unwrap();

        match parent.node_type() {
            NodeType::Element => {
                return self.append_element(parent, child);
            }
            _ => unimplemented!(),
        }
    }

    /// Returns one node's children list.
    pub fn children(&self, parent: Option<&DOMObject>) -> NodeIterator<'_> {
        if let Some(parent) = parent {
            match parent.node_type() {
                NodeType::Element => {
                    if let Some(el) = self.element(parent) {
                        NodeIterator::Iter(el.node.children.iter())
                    } else {
                        NodeIterator::Empty
                    }
                }
                _ => unimplemented!(),
            }
        } else {
            NodeIterator::Iter(self.this_node.children.iter())
        }
    }

    /// Returns a immutable reference to [`ArenaElement`]
    pub fn element(&self, object: &DOMObject) -> Option<&ArenaElement<'a>> {
        assert_eq!(object.node_type(), NodeType::Element);

        self.els.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`ArenaElement`]
    pub fn element_mut(&mut self, object: &DOMObject) -> Option<&mut ArenaElement<'a>> {
        assert_eq!(object.node_type(), NodeType::Element);

        self.els.iter_mut().find(|el| el.object == *object)
    }

    /// Returns a immutable reference to [`ArenaAttr`]
    pub fn attr(&self, object: &DOMObject) -> Option<&ArenaAttr<'a>> {
        assert_eq!(object.node_type(), NodeType::Attribute);

        self.attrs.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`ArenaAttr`]
    pub fn attr_mut(&mut self, object: &DOMObject) -> Option<&mut ArenaAttr<'a>> {
        assert_eq!(object.node_type(), NodeType::Attribute);

        self.attrs.iter_mut().find(|el| el.object == *object)
    }

    /// Returns a immutable reference to [`ArenaNamespace`]
    pub fn ns(&self, object: &DOMObject) -> Option<&ArenaNamespace<'a>> {
        assert_eq!(object.node_type(), NodeType::Namespace);

        self.nss.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`ArenaNamespace`]
    pub fn ns_mut(&mut self, object: &DOMObject) -> Option<&mut ArenaNamespace<'a>> {
        assert_eq!(object.node_type(), NodeType::Namespace);

        self.nss.iter_mut().find(|el| el.object == *object)
    }

    /// Returns a immutable reference to [`ArenaProcessingInstruction`]
    pub fn pi(&self, object: &DOMObject) -> Option<&ArenaProcessingInstruction<'a>> {
        assert_eq!(object.node_type(), NodeType::ProcessingInstruction);

        self.pis.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`ArenaProcessingInstruction`]
    pub fn pi_mut(&mut self, object: &DOMObject) -> Option<&mut ArenaProcessingInstruction<'a>> {
        assert_eq!(object.node_type(), NodeType::ProcessingInstruction);

        self.pis.iter_mut().find(|el| el.object == *object)
    }

    /// Returns a immutable reference to [`ArenaProcessingInstruction`]
    pub fn comment(&self, object: &DOMObject) -> Option<&ArenaComment<'a>> {
        assert_eq!(object.node_type(), NodeType::Comment);

        self.cms.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`ArenaProcessingInstruction`]
    pub fn comment_mut(&mut self, object: &DOMObject) -> Option<&mut ArenaComment<'a>> {
        assert_eq!(object.node_type(), NodeType::Comment);

        self.cms.iter_mut().find(|el| el.object == *object)
    }
}

/// An Iterator over one node's children.
pub enum NodeIterator<'a> {
    Iter(Iter<'a, DOMObject>),

    Empty,
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a DOMObject;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            NodeIterator::Iter(iter) => iter.next(),
            NodeIterator::Empty => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArenaDocument;

    #[test]
    fn test_gc() {
        let mut doc = ArenaDocument::default();

        let element = doc.create_element("hello").unwrap();

        doc.append_child(None, element).unwrap();

        doc.gc();

        assert_eq!(doc.allocated(), 1);

        doc.create_element("hello").unwrap();

        assert_eq!(doc.allocated(), 2);

        doc.gc();

        assert_eq!(doc.allocated(), 1);
    }

    #[test]
    fn el_append() {
        let mut doc = ArenaDocument::default();

        let element = doc.create_element("hello").unwrap();

        // append attribute
        {
            let attr = doc.create_element("color").unwrap();

            doc.append_child(Some(&element), attr).unwrap();
        }

        // append attribute
        {
            let attr = doc.create_attr("color", "#ff00ff").unwrap();

            doc.append_child(Some(&element), attr).unwrap();
        }

        // append namespace.
        {
            let ns = doc
                .create_ns("xhtml", "http://www.w3.org/1999/xhtml")
                .unwrap();

            doc.append_child(Some(&element), ns).unwrap();
        }

        // append ProcessingInstruction.
        {
            let ns = doc
                .create_pi("xml-stylesheet", r#"type="text/xsl" href="style.xsl""#)
                .unwrap();

            doc.append_child(Some(&element), ns).unwrap();
        }

        // append ProcessingInstruction.
        {
            let comment = doc.create_comment("xml-stylesheet").unwrap();

            doc.append_child(Some(&element), comment).unwrap();
        }

        doc.append_child(None, element).unwrap();
    }

    #[test]
    fn append_twice() {
        let mut doc = ArenaDocument::default();

        let element = doc.create_element("hello").unwrap();

        doc.append_child(None, element).unwrap();

        doc.append_child(None, element)
            .expect_err("twice append check");
    }
}
