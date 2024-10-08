//! Document Object Model (DOM) Level 2 Core interfaces.

use std::borrow::Cow;

use crate::{DOMObject, NodeType, QName};

/// A `Node` interface for Document Object Model (DOM) Level 2 Core
pub trait Node<'doc> {
    /// NodeList returns by [`children`](Node::children) function.
    type NodeList<'a>: Iterator<Item = &'a DOMObject>
    where
        Self: 'a;

    /// NamedNodeMap returns by [`attributes`](Node::attributes) function.
    type NamedNodeMap<'a>
    where
        Self: 'a;

    /// Returns the [`type`](NodeType) of this node.
    fn node_type(&self) -> NodeType;

    /// Returns the [`DOMObject`] of parent node.
    fn parent(&self) -> Option<DOMObject>;

    /// Returns the iterator over children node list.
    fn children(&self) -> Self::NodeList<'_>;

    /// The first child of this node. If there is no such node, this returns null.
    fn first_child(&self) -> Option<&DOMObject>;

    /// The last child of this node. If there is no such node, this returns null.
    fn last_child(&self) -> Option<&DOMObject>;

    /// The node immediately preceding this node. If there is no such node, this returns null.
    fn previous_sibling(&self) -> Option<&DOMObject>;

    /// The node immediately following this node. If there is no such node, this returns null.
    fn next_sibling(&self) -> Option<&DOMObject>;

    /// For nodes of any type other than ELEMENT_NODE and ATTRIBUTE_NODE and nodes created with a DOM Level 1 method, such as createElement from the Document interface, this is always null.
    fn qname(&self) -> Option<&QName<'_>>;

    /// The name of this node, depending on its type;
    fn node_name(&self) -> &str;

    /// A NamedNodeMap containing the attributes of this node (if it is an Element) or null otherwise.
    fn attributes(&self) -> Self::NamedNodeMap<'_>;
}

/// A document node must implement this trait.
pub trait Document<'doc>: Node<'doc> {
    /// Error type returns by this trait's functions.
    type Error: std::error::Error;

    /// `Element` type returns by this trait.
    type Element<'a>
    where
        Self: 'a;

    /// `Attr` type returns by this document.
    type Attr<'a>
    where
        Self: 'a;

    /// `Namespace` type returns by this document.
    type Namespace<'a>
    where
        Self: 'a;

    /// `ProcessingInstruction` type returns by this document.
    type ProcessingInstruction<'a>
    where
        Self: 'a;

    /// `Notation` type returns by this document.
    type Notation<'a>
    where
        Self: 'a;

    /// `Comment` type returns by this document.
    type Comment<'a>
    where
        Self: 'a;

    /// `CData` type returns by this document.
    type CData<'a>
    where
        Self: 'a;

    /// `Entity` type returns by this document.
    type Entity<'a>
    where
        Self: 'a;

    /// `Text` type returns by this document.
    type Text<'a>
    where
        Self: 'a;

    /// `DocumentType` type returns by this document.
    type DocumentType<'a>
    where
        Self: 'a;

    /// Create a new `Element` node associated with this document.
    fn create_element<T>(&mut self, tag: T) -> Result<DOMObject, Self::Error>
    where
        T: TryInto<QName<'doc>>;

    /// Create a new `Attr` node associated with this document.
    fn create_attr<T, V>(&mut self, tag: T, value: V) -> Result<DOMObject, Self::Error>;

    /// Create a new `Namespace` node associated with this document.
    fn create_ns<P, H>(&mut self, prefix: P, href: H) -> Result<DOMObject, Self::Error>
    where
        P: Into<Cow<'doc, str>>,
        H: Into<Cow<'doc, str>>;

    /// Create a new `ProcessingInstruction` node associated with this document.
    fn create_pi<T, D>(&mut self, target: T, data: D) -> Result<DOMObject, Self::Error>
    where
        T: Into<Cow<'doc, str>>,
        D: Into<Cow<'doc, str>>;

    /// Create a new `Notation` node associated with this document.
    fn create_notation<P, S>(
        &mut self,
        public_id: P,
        system_id: S,
    ) -> Result<DOMObject, Self::Error>
    where
        P: Into<Cow<'doc, str>>,
        S: Into<Cow<'doc, str>>;

    /// Create a new `Comment` node associated with this document.
    fn create_comment<D>(&mut self, data: D) -> Result<DOMObject, Self::Error>
    where
        D: Into<Cow<'doc, str>>;

    /// Create a new `Entity` node associated with this document.
    fn create_entity<P, S>(
        &mut self,
        public_id: P,
        system_id: S,
        notation_name: Option<Cow<'_, str>>,
    ) -> Result<DOMObject, Self::Error>
    where
        P: Into<Cow<'doc, str>>,
        S: Into<Cow<'doc, str>>;

    /// Create a new `CData` node associated with this document.
    fn create_cdata<D>(&mut self, data: D) -> Result<DOMObject, Self::Error>
    where
        D: Into<Cow<'doc, str>>;

    /// Create a new `Text` node associated with this document.
    fn create_text<D>(&mut self, data: D) -> Result<DOMObject, Self::Error>
    where
        D: Into<Cow<'doc, str>>;

    /// Create a new `DocumentType` node.
    fn create_document_type<P, S, I>(
        &mut self,
        public_id: P,
        system_id: S,
        internal_subset: I,
    ) -> Result<DOMObject, Self::Error>
    where
        P: Into<Cow<'doc, str>>,
        S: Into<Cow<'doc, str>>,
        I: Into<Cow<'doc, str>>;
}
