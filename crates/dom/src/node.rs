#![allow(unused)]

use crate::DOMObject;

/// Corresponds to one DOM `Node`.
#[derive(Debug, Default)]
pub(crate) struct DOMNode {
    /// parent object reference.
    parent: Option<DOMObject>,
    /// children node list.
    children: Vec<DOMObject>,
}

impl DOMNode {
    /// Get the parent object of this node.
    pub fn parent(&self) -> Option<&DOMObject> {
        self.parent.as_ref()
    }

    /// Set the parent object of this node.
    pub fn set_parent(&mut self, parent: DOMObject) {
        self.parent = Some(parent);
    }

    /// Returns the iterator over the children list.
    pub fn children(&self) -> impl Iterator<Item = &DOMObject> {
        self.children.iter()
    }

    /// Append new child object.
    ///
    /// Returns false, if the new object is is already in the tree.
    pub fn append_child(&mut self, new: DOMObject) -> bool {
        if self.remove_child(&new) {
            self.children.push(new);
            false
        } else {
            self.children.push(new);
            true
        }
    }

    /// Remove object from the list of children.
    pub fn remove_child(&mut self, old: &DOMObject) -> bool {
        if let Some(index) =
            self.children
                .iter()
                .enumerate()
                .find_map(|(index, obj)| if *obj == *old { Some(index) } else { None })
        {
            self.children.swap_remove(index);
            true
        } else {
            false
        }
    }

    /// Returns whether this node has any children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

/// Corresponds to one DOM `Leaf`.
#[derive(Debug, Default)]
pub(crate) struct DOMLeaf(Option<DOMObject>);

impl DOMLeaf {
    /// Get the parent object of this node.
    pub fn parent(&self) -> Option<&DOMObject> {
        self.0.as_ref()
    }

    /// Set the parent object of this node.
    pub fn set_parent(&mut self, parent: DOMObject) {
        self.0 = Some(parent);
    }
}
