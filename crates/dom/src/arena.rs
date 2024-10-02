//! This mod provide a DOM implementation with arena memory managerment.

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
        self.children.iter().find(|obj| obj.node_type == *node_type)
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

pub(crate) struct ArenaElement<'a> {
    object: DOMObject,
    node: ArenaNode,
    #[allow(unused)]
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
        match child.node_type {
            NodeType::Element
            | NodeType::Text
            | NodeType::Comment
            | NodeType::CData
            | NodeType::ProcessingInstruction
            | NodeType::EntityReference => {
                self.node.remove_child(child);
                Ok(())
            }
            _ => return Err(Error::DOMException(ExceptionCode::HIERARCHY_REQUEST_ERR)),
        }
    }
}

/// A DOM `Document` implementation with arena memory managerment.
#[derive(Default)]
pub struct Document<'a> {
    this_node: ArenaNode,
    els: Vec<ArenaElement<'a>>,
}

impl<'a> Document<'a> {
    fn gc_mark(&mut self, object: &DOMObject) {
        match object.node_type {
            NodeType::Element => {
                self.element_mut(object).unwrap().node.gc_mark();
            }
            _ => unimplemented!(),
        }
    }

    fn element(&self, object: &DOMObject) -> Option<&ArenaElement<'a>> {
        assert_eq!(object.node_type, NodeType::Element);

        self.els.iter().find(|el| el.object == *object)
    }

    fn element_mut(&mut self, object: &DOMObject) -> Option<&mut ArenaElement<'a>> {
        assert_eq!(object.node_type, NodeType::Element);

        self.els.iter_mut().find(|el| el.object == *object)
    }
}

impl<'a> Document<'a> {
    /// Dealloc unused nodes.
    pub fn gc(&mut self) {
        // mark used nodes.

        let mut stack = self.this_node.children.clone();

        // process DFS
        while let Some(top) = stack.pop() {
            self.gc_mark(&top);
            if let Some(mut children) = self.children(&top) {
                stack.append(&mut children);
            }
        }

        // check elements.
        let mut els = vec![];

        for mut el in self.els.drain(..) {
            if el.node.check_gc_state() {
                els.push(el);
            }
        }

        self.els = els;
    }

    /// Create a new `Element` node.
    pub fn create_element<T>(&mut self, tag: QName<'a>) -> DOMObject {
        let object = DOMObject::new(self.els.len(), NodeType::Element);

        let el = ArenaElement::new(object.clone(), tag);

        self.els.push(el);

        object
    }

    /// Append a new child to the `Document` node.
    pub fn append(&mut self, child: DOMObject) -> Result<()> {
        match child.node_type {
            NodeType::Element | NodeType::DocumentType => {
                if self.this_node.first_child_of(&child.node_type).is_some() {
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

    /// Attach a new child to the parent node.
    ///
    /// If the parent is none, the new child will be attached to root node.
    pub fn append_child(&mut self, parent: Option<&DOMObject>, child: DOMObject) -> Result<()> {
        if parent.is_none() {
            return self.append(child);
        }

        let parent = parent.unwrap();

        match parent.node_type {
            NodeType::Element => {
                return self.append_element(parent, child);
            }
            _ => unimplemented!(),
        }
    }

    /// Attach a new child to one element node.
    pub fn append_element(&mut self, parent: &DOMObject, child: DOMObject) -> Result<()> {
        assert_eq!(parent.node_type, NodeType::Element);

        self.els
            .iter_mut()
            .find(|el| el.object == *parent)
            .ok_or(Error::DOMException(ExceptionCode::NOT_FOUND_ERR))?
            .append_child(child)
    }

    /// Returns one node's children list.
    pub fn children(&self, parent: &DOMObject) -> Option<Vec<DOMObject>> {
        match parent.node_type {
            NodeType::Element => self.element(parent).map(|el| el.node.children.clone()),
            _ => unimplemented!(),
        }
    }
}
