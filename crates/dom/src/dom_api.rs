//! Document Object Model (DOM) Level 2 Core interfaces.

use std::borrow::Cow;

use crate::{NodeType, QName};

/// A `Node` interface for Document Object Model (DOM) Level 2 Core
pub trait Node<'doc> {
    /// Node reference type.
    type Ref: 'static;

    /// NodeList returns by [`children`](Node::children) function.
    type NodeList<'a>: Iterator<Item = &'a Self::Ref>
    where
        Self: 'a;

    /// NamedNodeMap returns by [`attributes`](Node::attributes) function.
    type NamedNodeMap<'a>
    where
        Self: 'a;

    /// Returns the [`type`](NodeType) of this node.
    fn node_type(&self) -> NodeType;

    /// Returns the `Ref` of parent node.
    fn parent(&self) -> Option<&Self::Ref>;

    /// Returns the iterator over children node list.
    fn children(&self) -> Self::NodeList<'_>;

    /// The first child of this node. If there is no such node, this returns null.
    fn first_child(&self) -> Option<&Self::Ref>;

    /// The last child of this node. If there is no such node, this returns null.
    fn last_child(&self) -> Option<&Self::Ref>;

    /// The node immediately preceding this node. If there is no such node, this returns null.
    fn previous_sibling(&self) -> Option<&Self::Ref>;

    /// The node immediately following this node. If there is no such node, this returns null.
    fn next_sibling(&self) -> Option<&Self::Ref>;

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
    fn create_element<T>(&mut self, tag: T) -> Result<Self::Ref, Self::Error>
    where
        T: TryInto<QName<'doc>>;

    /// Get immutable `Element` node by reference.
    fn element(&self, of: Self::Ref) -> Option<&Self::Element<'_>>;

    /// Get mutable `Element` node by reference.
    fn element_mut(&mut self, of: Self::Ref) -> Option<&mut Self::Element<'_>>;

    /// Create a new `Attr` node associated with this document.
    fn create_attr<T, V>(&mut self, tag: T, value: V) -> Result<Self::Ref, Self::Error>;

    /// Get immutable `Attr` node by reference.
    fn attr(&self, of: Self::Ref) -> Option<&Self::Attr<'_>>;

    /// Get mutable `Attr` node by reference.
    fn attr_mut(&mut self, of: Self::Ref) -> Option<&mut Self::Attr<'_>>;

    /// Create a new `Namespace` node associated with this document.
    fn create_ns<P, H>(&mut self, prefix: P, href: H) -> Result<Self::Ref, Self::Error>
    where
        P: Into<Cow<'doc, str>>,
        H: Into<Cow<'doc, str>>;

    /// Get immutable `Namespace` node by reference.
    fn ns(&self, of: Self::Ref) -> Option<&Self::Namespace<'_>>;

    /// Get mutable `Namespace` node by reference.
    fn ns_mut(&mut self, of: Self::Ref) -> Option<&mut Self::Namespace<'_>>;

    /// Create a new `ProcessingInstruction` node associated with this document.
    fn create_pi<T, D>(&mut self, target: T, data: D) -> Result<Self::Ref, Self::Error>
    where
        T: Into<Cow<'doc, str>>,
        D: Into<Cow<'doc, str>>;

    /// Get immutable `ProcessingInstruction` node by reference.
    fn pi(&self, of: Self::Ref) -> Option<&Self::ProcessingInstruction<'_>>;

    /// Get mutable `ProcessingInstruction` node by reference.
    fn pi_mut(&mut self, of: Self::Ref) -> Option<&mut Self::ProcessingInstruction<'_>>;

    /// Create a new `Notation` node associated with this document.
    fn create_notation<P, S>(
        &mut self,
        public_id: P,
        system_id: S,
    ) -> Result<Self::Ref, Self::Error>
    where
        P: Into<Cow<'doc, str>>,
        S: Into<Cow<'doc, str>>;

    /// Get immutable `Notation` node by reference.
    fn notation(&self, of: Self::Ref) -> Option<&Self::Notation<'_>>;

    /// Get mutable `Notation` node by reference.
    fn notation_mut(&mut self, of: Self::Ref) -> Option<&mut Self::Notation<'_>>;

    /// Create a new `Comment` node associated with this document.
    fn create_comment<D>(&mut self, data: D) -> Result<Self::Ref, Self::Error>
    where
        D: Into<Cow<'doc, str>>;

    /// Get immutable `Comment` node by reference.
    fn comment(&self, of: Self::Ref) -> Option<&Self::Comment<'_>>;

    /// Get mutable `Comment` node by reference.
    fn comment_mut(&mut self, of: Self::Ref) -> Option<&mut Self::Comment<'_>>;

    /// Create a new `Entity` node associated with this document.
    fn create_entity<P, S>(
        &mut self,
        public_id: P,
        system_id: S,
        notation_name: Option<Cow<'_, str>>,
    ) -> Result<Self::Ref, Self::Error>
    where
        P: Into<Cow<'doc, str>>,
        S: Into<Cow<'doc, str>>;

    /// Get immutable `Entity` node by reference.
    fn entity(&self, of: Self::Ref) -> Option<&Self::Entity<'_>>;

    /// Get mutable `Entity` node by reference.
    fn entity_mut(&mut self, of: Self::Ref) -> Option<&mut Self::Entity<'_>>;

    /// Create a new `CData` node associated with this document.
    fn create_cdata<D>(&mut self, data: D) -> Result<Self::Ref, Self::Error>
    where
        D: Into<Cow<'doc, str>>;

    /// Get immutable `Entity` node by reference.
    fn cdata(&self, of: Self::Ref) -> Option<&Self::CData<'_>>;

    /// Get mutable `Entity` node by reference.
    fn cdata_mut(&mut self, of: Self::Ref) -> Option<&mut Self::CData<'_>>;

    /// Create a new `Text` node associated with this document.
    fn create_text<D>(&mut self, data: D) -> Result<Self::Ref, Self::Error>
    where
        D: Into<Cow<'doc, str>>;

    /// Get immutable `Text` node by reference.
    fn text(&self, of: Self::Ref) -> Option<&Self::Text<'_>>;

    /// Get mutable `Text` node by reference.
    fn text_mut(&mut self, of: Self::Ref) -> Option<&mut Self::Text<'_>>;

    /// Create a new `DocumentType` node.
    fn create_doctype<P, S, I>(
        &mut self,
        public_id: P,
        system_id: S,
        internal_subset: I,
    ) -> Result<Self::Ref, Self::Error>
    where
        P: Into<Cow<'doc, str>>,
        S: Into<Cow<'doc, str>>,
        I: Into<Cow<'doc, str>>;

    /// Get immutable `DocumentType` node by reference.
    fn doctype(&self, of: Self::Ref) -> Option<&Self::DocumentType<'_>>;

    /// Get mutable `DocumentType` node by reference.
    fn doctype_mut(&mut self, of: Self::Ref) -> Option<&mut Self::DocumentType<'_>>;
}
