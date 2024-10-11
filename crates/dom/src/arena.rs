//! This mod provide a DOM implementation with  memory managerment.

use std::{borrow::Cow, slice::Iter};

use crate::{DOMObject, Error, ExceptionCode, NodeType, QName, Result};

/// Use by gc process.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum GcState {
    Unmark,
    Marked,
}

impl Default for GcState {
    fn default() -> Self {
        Self::Unmark
    }
}

/// A DOM node is allocated by and belongs to one [`DOM`]
#[derive(Default)]
struct Node {
    gc_state: GcState,
    #[allow(unused)]
    parent: Option<DOMObject>,
    children: Vec<DOMObject>,
}

impl Node {
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
        self.gc_state = GcState::Marked;
    }

    fn check_gc_state(&mut self) -> bool {
        if self.gc_state == GcState::Marked {
            self.gc_state = GcState::Unmark;
            true
        } else {
            false
        }
    }
}

/// This corresponds to the namespace extension.
pub struct Namespace<'a> {
    /// object reference.
    object: DOMObject,
    /// mxin node.
    node: Node,
    /// The namespace prefix
    prefix: Cow<'a, str>,
    /// The namespace href
    href: Cow<'a, str>,
}

#[allow(unused)]
impl<'a> Namespace<'a> {
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
pub struct Attr<'a> {
    object: DOMObject,
    node: Node,
    name: QName<'a>,
    value: Cow<'a, str>,
}

impl<'a> Attr<'a> {
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

    fn append_child(&mut self, child: DOMObject) -> Result<()> {
        self.remove_child(&child)?;
        self.node.append_child(child);

        Ok(())
    }

    fn remove_child(&mut self, child: &DOMObject) -> Result<()> {
        match child.node_type() {
            NodeType::Text | NodeType::EntityReference => {
                self.node.remove_child(child);
                Ok(())
            }
            _ => return Err(Error::DOMException(ExceptionCode::HIERARCHY_REQUEST_ERR)),
        }
    }
}

/// `Element` allocated by one `Document`.
pub struct Element<'a> {
    object: DOMObject,
    node: Node,
    tag: QName<'a>,
}

impl<'a> AsRef<DOMObject> for Element<'a> {
    fn as_ref(&self) -> &DOMObject {
        &self.object
    }
}

impl<'a> Element<'a> {
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
            | NodeType::Namespace
            | NodeType::DocumentType => {
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

/// `ProcessingInstruction` allocated by one `Document`.
pub struct ProcessingInstruction<'a> {
    object: DOMObject,
    node: Node,
    target: Cow<'a, str>,
    data: Cow<'a, str>,
}

impl<'a> AsRef<DOMObject> for ProcessingInstruction<'a> {
    fn as_ref(&self) -> &DOMObject {
        &self.object
    }
}

impl<'a> ProcessingInstruction<'a> {
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

/// `Comment` allocated by one `Document`.
pub struct Comment<'a> {
    object: DOMObject,
    node: Node,
    data: Cow<'a, str>,
}

impl<'a> AsRef<DOMObject> for Comment<'a> {
    fn as_ref(&self) -> &DOMObject {
        &self.object
    }
}

impl<'a> Comment<'a> {
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

/// `CData` allocated by one `Document`.
pub struct CData<'a> {
    object: DOMObject,
    node: Node,
    data: Cow<'a, str>,
}

impl<'a> AsRef<DOMObject> for CData<'a> {
    fn as_ref(&self) -> &DOMObject {
        &self.object
    }
}

impl<'a> CData<'a> {
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

/// `Text` allocated by one `Document`.
pub struct Text<'a> {
    object: DOMObject,
    node: Node,
    data: Cow<'a, str>,
}

impl<'a> AsRef<DOMObject> for Text<'a> {
    fn as_ref(&self) -> &DOMObject {
        &self.object
    }
}

impl<'a> Text<'a> {
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

/// `Notation` allocated by one `Document`.
pub struct Notation<'a> {
    object: DOMObject,
    node: Node,
    public_id: Cow<'a, str>,
    system_id: Cow<'a, str>,
}

impl<'a> AsRef<DOMObject> for Notation<'a> {
    fn as_ref(&self) -> &DOMObject {
        &self.object
    }
}

impl<'a> Notation<'a> {
    fn new(object: DOMObject, target: Cow<'a, str>, data: Cow<'a, str>) -> Self {
        Self {
            object,
            node: Default::default(),
            public_id: target,
            system_id: data,
        }
    }

    /// Returns the target as str
    pub fn public_id(&self) -> &str {
        &self.public_id
    }

    /// Returns the data as str
    pub fn system_id(&self) -> &str {
        &self.system_id
    }
}

/// `Entity` allocated by one `Document`.
pub struct Entity<'a> {
    object: DOMObject,
    node: Node,
    notation_name: Option<Cow<'a, str>>,
    public_id: Cow<'a, str>,
    system_id: Cow<'a, str>,
}

impl<'a> AsRef<DOMObject> for Entity<'a> {
    fn as_ref(&self) -> &DOMObject {
        &self.object
    }
}

impl<'a> Entity<'a> {
    fn new(
        object: DOMObject,
        public_id: Cow<'a, str>,
        system_id: Cow<'a, str>,
        notation_name: Option<Cow<'a, str>>,
    ) -> Self {
        Self {
            object,
            node: Default::default(),
            public_id,
            system_id,
            notation_name,
        }
    }

    /// Returns the target as str
    pub fn public_id(&self) -> &str {
        &self.public_id
    }

    /// Returns the data as str
    pub fn system_id(&self) -> &str {
        &self.system_id
    }

    /// For unparsed entities, the name of the notation for the entity. For parsed entities, this is [`None`].
    pub fn notation_name(&self) -> Option<&str> {
        self.notation_name.as_deref()
    }
}

/// `DocumentType` allocated by one `Document`.
pub struct DocumentType<'a> {
    object: DOMObject,
    node: Node,
    internal_subset: Cow<'a, str>,
    public_id: Cow<'a, str>,
    system_id: Cow<'a, str>,
}

impl<'a> AsRef<DOMObject> for DocumentType<'a> {
    fn as_ref(&self) -> &DOMObject {
        &self.object
    }
}

impl<'a> DocumentType<'a> {
    fn new(
        object: DOMObject,
        public_id: Cow<'a, str>,
        system_id: Cow<'a, str>,
        internal_subset: Cow<'a, str>,
    ) -> Self {
        Self {
            object,
            node: Default::default(),
            public_id,
            system_id,
            internal_subset,
        }
    }

    /// Returns the target as str
    pub fn public_id(&self) -> &str {
        &self.public_id
    }

    /// Returns the data as str
    pub fn system_id(&self) -> &str {
        &self.system_id
    }

    /// For unparsed entities, the name of the notation for the entity. For parsed entities, this is [`None`].
    pub fn internal_subset(&self) -> &str {
        &self.internal_subset
    }
}

/// A DOM `Document` implementation with  memory managerment.
#[derive(Default)]
pub struct Document<'a> {
    this_node: Node,
    doc_types: Vec<DocumentType<'a>>,
    els: Vec<Element<'a>>,
    attrs: Vec<Attr<'a>>,
    nss: Vec<Namespace<'a>>,
    pis: Vec<ProcessingInstruction<'a>>,
    cms: Vec<Comment<'a>>,
    texts: Vec<Text<'a>>,
    notations: Vec<Notation<'a>>,
    entities: Vec<Entity<'a>>,
    cdatas: Vec<CData<'a>>,
}

impl<'a> Document<'a> {
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
            NodeType::ProcessingInstruction | NodeType::Comment => {
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
            NodeType::Text => {
                if self
                    .text(child)
                    .ok_or(Error::DOMException(ExceptionCode::NOT_FOUND_ERR))?
                    .node
                    .parent
                    .is_some()
                {
                    return Err(Error::DOMException(ExceptionCode::HIERARCHY_REQUEST_ERR));
                }
            }
            NodeType::DocumentType => {
                if self
                    .doc_type(child)
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

    /// Attach a new child to one element node.
    fn append_attr(&mut self, parent: &DOMObject, child: DOMObject) -> Result<()> {
        assert_eq!(parent.node_type(), NodeType::Attribute);

        self.attrs
            .iter_mut()
            .find(|el| el.object == *parent)
            .ok_or(Error::DOMException(ExceptionCode::NOT_FOUND_ERR))?
            .append_child(child)?;

        Ok(())
    }
}

impl<'a> Document<'a> {
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

        // check Text list.
        let mut texts = vec![];

        for mut text in self.texts.drain(..) {
            if text.node.check_gc_state() {
                texts.push(text);
            }
        }

        self.texts = texts;

        // check Notation list.
        let mut notations = vec![];

        for mut notation in self.notations.drain(..) {
            if notation.node.check_gc_state() {
                notations.push(notation);
            }
        }

        self.notations = notations;

        // check Entity list.
        let mut entities = vec![];

        for mut entity in self.entities.drain(..) {
            if entity.node.check_gc_state() {
                entities.push(entity);
            }
        }

        self.entities = entities;

        // check DocumentType list.
        let mut doc_types = vec![];

        for mut doc_type in self.doc_types.drain(..) {
            if doc_type.node.check_gc_state() {
                doc_types.push(doc_type);
            }
        }

        self.doc_types = doc_types;

        // check CData list.
        let mut cdatas = vec![];

        for mut cdata in self.cdatas.drain(..) {
            if cdata.node.check_gc_state() {
                cdatas.push(cdata);
            }
        }

        self.cdatas = cdatas;
    }

    /// Create a new `Element` node.
    pub fn create_element<T>(&mut self, tag: T) -> Result<DOMObject>
    where
        T: TryInto<QName<'a>>,
        Error: From<T::Error>,
    {
        let object = DOMObject::new(self.els.len(), NodeType::Element);

        let el = Element::new(object.clone(), tag.try_into()?);

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

        let attr = Attr::new(object.clone(), tag.try_into()?, value.into());

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

        let ns = Namespace::new(object.clone(), prefix.into(), href.into());

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

        let pi = ProcessingInstruction::new(object.clone(), target.into(), data.into());

        self.pis.push(pi);

        Ok(object)
    }

    /// Create a new `Notation` node.
    pub fn create_notation<P, S>(&mut self, public_id: P, system_id: S) -> Result<DOMObject>
    where
        P: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
    {
        let object = DOMObject::new(self.els.len(), NodeType::Notation);

        let notation = Notation::new(object.clone(), public_id.into(), system_id.into());

        self.notations.push(notation);

        Ok(object)
    }

    /// Create a new `Entity` node.
    pub fn create_entity<P, S>(
        &mut self,
        public_id: P,
        system_id: S,
        notation_name: Option<Cow<'a, str>>,
    ) -> Result<DOMObject>
    where
        P: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
    {
        let object = DOMObject::new(self.els.len(), NodeType::Entity);

        let entity = Entity::new(
            object.clone(),
            public_id.into(),
            system_id.into(),
            notation_name,
        );

        self.entities.push(entity);

        Ok(object)
    }

    /// Create a new `Comment` node.
    pub fn create_comment<D>(&mut self, data: D) -> Result<DOMObject>
    where
        D: Into<Cow<'a, str>>,
    {
        let object = DOMObject::new(self.els.len(), NodeType::Comment);

        let cm = Comment::new(object.clone(), data.into());

        self.cms.push(cm);

        Ok(object)
    }

    /// Create a new `CData` node.
    pub fn create_cdata<D>(&mut self, data: D) -> Result<DOMObject>
    where
        D: Into<Cow<'a, str>>,
    {
        let object = DOMObject::new(self.els.len(), NodeType::CData);

        let cdata = CData::new(object.clone(), data.into());

        self.cdatas.push(cdata);

        Ok(object)
    }

    /// Create a new `Text` node.
    pub fn create_text<D>(&mut self, data: D) -> Result<DOMObject>
    where
        D: Into<Cow<'a, str>>,
    {
        let object = DOMObject::new(self.els.len(), NodeType::Text);

        let text = Text::new(object.clone(), data.into());

        self.texts.push(text);

        Ok(object)
    }

    /// Create a new `DocumentType` node.
    pub fn create_document_type<P, S, I>(
        &mut self,
        public_id: P,
        system_id: S,
        internal_subset: I,
    ) -> Result<DOMObject>
    where
        P: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        I: Into<Cow<'a, str>>,
    {
        let object = DOMObject::new(self.els.len(), NodeType::DocumentType);

        let doc_type = DocumentType::new(
            object.clone(),
            public_id.into(),
            system_id.into(),
            internal_subset.into(),
        );

        self.doc_types.push(doc_type);

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
            NodeType::Attribute => {
                return self.append_attr(parent, child);
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

    /// Returns a immutable reference to [`Element`]
    pub fn element(&self, object: &DOMObject) -> Option<&Element<'a>> {
        assert_eq!(object.node_type(), NodeType::Element);

        self.els.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`Element`]
    pub fn element_mut(&mut self, object: &DOMObject) -> Option<&mut Element<'a>> {
        assert_eq!(object.node_type(), NodeType::Element);

        self.els.iter_mut().find(|el| el.object == *object)
    }

    /// Returns a immutable reference to [`Attr`]
    pub fn attr(&self, object: &DOMObject) -> Option<&Attr<'a>> {
        assert_eq!(object.node_type(), NodeType::Attribute);

        self.attrs.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`Attr`]
    pub fn attr_mut(&mut self, object: &DOMObject) -> Option<&mut Attr<'a>> {
        assert_eq!(object.node_type(), NodeType::Attribute);

        self.attrs.iter_mut().find(|el| el.object == *object)
    }

    /// Returns a immutable reference to [`Namespace`]
    pub fn ns(&self, object: &DOMObject) -> Option<&Namespace<'a>> {
        assert_eq!(object.node_type(), NodeType::Namespace);

        self.nss.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`Namespace`]
    pub fn ns_mut(&mut self, object: &DOMObject) -> Option<&mut Namespace<'a>> {
        assert_eq!(object.node_type(), NodeType::Namespace);

        self.nss.iter_mut().find(|el| el.object == *object)
    }

    /// Returns a immutable reference to [`ProcessingInstruction`]
    pub fn pi(&self, object: &DOMObject) -> Option<&ProcessingInstruction<'a>> {
        assert_eq!(object.node_type(), NodeType::ProcessingInstruction);

        self.pis.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`ProcessingInstruction`]
    pub fn pi_mut(&mut self, object: &DOMObject) -> Option<&mut ProcessingInstruction<'a>> {
        assert_eq!(object.node_type(), NodeType::ProcessingInstruction);

        self.pis.iter_mut().find(|el| el.object == *object)
    }

    /// Returns a immutable reference to [`Comment`]
    pub fn comment(&self, object: &DOMObject) -> Option<&Comment<'a>> {
        assert_eq!(object.node_type(), NodeType::Comment);

        self.cms.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`Comment`]
    pub fn comment_mut(&mut self, object: &DOMObject) -> Option<&mut Comment<'a>> {
        assert_eq!(object.node_type(), NodeType::Comment);

        self.cms.iter_mut().find(|el| el.object == *object)
    }

    /// Returns a immutable reference to [`Text`]
    pub fn text(&self, object: &DOMObject) -> Option<&Text<'a>> {
        assert_eq!(object.node_type(), NodeType::Text);

        self.texts.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`Text`]
    pub fn text_mut(&mut self, object: &DOMObject) -> Option<&mut Text<'a>> {
        assert_eq!(object.node_type(), NodeType::Text);

        self.texts.iter_mut().find(|el| el.object == *object)
    }

    /// Returns a immutable reference to [`Notation`]
    pub fn notation(&self, object: &DOMObject) -> Option<&Notation<'a>> {
        assert_eq!(object.node_type(), NodeType::Notation);

        self.notations.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`Text`]
    pub fn notation_mut(&mut self, object: &DOMObject) -> Option<&mut Notation<'a>> {
        assert_eq!(object.node_type(), NodeType::Notation);

        self.notations.iter_mut().find(|el| el.object == *object)
    }

    /// Returns a immutable reference to [`Entity`]
    pub fn entity(&self, object: &DOMObject) -> Option<&Entity<'a>> {
        assert_eq!(object.node_type(), NodeType::Entity);

        self.entities.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`Entity`]
    pub fn entity_mut(&mut self, object: &DOMObject) -> Option<&mut Entity<'a>> {
        assert_eq!(object.node_type(), NodeType::Entity);

        self.entities.iter_mut().find(|el| el.object == *object)
    }

    /// Returns a immutable reference to [`DocumentType`]
    pub fn doc_type(&self, object: &DOMObject) -> Option<&DocumentType<'a>> {
        assert_eq!(object.node_type(), NodeType::DocumentType);

        self.doc_types.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`Entity`]
    pub fn doc_type_mut(&mut self, object: &DOMObject) -> Option<&mut DocumentType<'a>> {
        assert_eq!(object.node_type(), NodeType::DocumentType);

        self.doc_types.iter_mut().find(|el| el.object == *object)
    }

    /// Returns a immutable reference to [`CData`]
    pub fn cdata(&self, object: &DOMObject) -> Option<&CData<'a>> {
        assert_eq!(object.node_type(), NodeType::DocumentType);

        self.cdatas.iter().find(|el| el.object == *object)
    }

    /// Returns a mutable reference to [`CData`]
    pub fn cdata_mut(&mut self, object: &DOMObject) -> Option<&mut CData<'a>> {
        assert_eq!(object.node_type(), NodeType::DocumentType);

        self.cdatas.iter_mut().find(|el| el.object == *object)
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
    use super::Document;

    #[test]
    fn test_gc() {
        let mut doc = Document::default();

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
    fn doc_maximum_of_one() {
        let mut doc = Document::default();

        let element = doc.create_element("hello").unwrap();

        doc.append_child(None, element).unwrap();

        doc.append_child(None, element)
            .expect_err("twice append check");

        let doc_type = doc.create_document_type("hello", "hello", "hello").unwrap();

        doc.append_child(None, doc_type).unwrap();

        doc.append_child(None, doc_type)
            .expect_err("twice append check");
    }

    #[test]
    fn doc_append() {
        let mut doc = Document::default();

        {
            let element = doc.create_element("hello").unwrap();
            doc.append_child(None, element).unwrap();
        }

        {
            let node = doc.create_pi("hello", "world").unwrap();
            doc.append_child(None, node).unwrap();
        }

        {
            let node = doc.create_comment("hello").unwrap();
            doc.append_child(None, node).unwrap();
        }
    }

    #[test]
    fn el_append() {
        let mut doc = Document::default();

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

        // append comment.
        {
            let comment = doc.create_comment("xml-stylesheet").unwrap();

            doc.append_child(Some(&element), comment).unwrap();
        }

        // append text.
        {
            let text = doc.create_text("xml-stylesheet").unwrap();

            doc.append_child(Some(&element), text).unwrap();
        }

        doc.append_child(None, element).unwrap();
    }

    #[test]
    fn attr_append() {
        let mut doc = Document::default();

        {
            let attr = doc.create_attr("color", "#ff00ff").unwrap();

            let text = doc.create_text("hello").unwrap();

            doc.append_child(Some(&attr), text).unwrap();
        }
    }
}
