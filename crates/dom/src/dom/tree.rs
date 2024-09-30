use crate::NodeRef;

#[derive(Default, Debug)]
pub struct XmlNode {
    /// parent node.
    parent: Option<NodeRef>,
    /// children nodes.
    children: Vec<NodeRef>,
}

impl XmlNode {
    /// The parent of this node. All nodes, except `Attr``, `Document`, `DocumentFragment`, `Entity`, and `Notation` may have a parent.
    pub fn parent(&self) -> Option<&NodeRef> {
        self.parent.as_ref()
    }

    /// A `Iterator` over all children of this node.
    pub fn children(&self) -> impl Iterator<Item = &NodeRef> {
        self.children.iter()
    }

    /// Removes a child from this Node.
    pub fn remove_child(&mut self, node: &NodeRef) {
        if let Some(index) =
            self.children
                .iter()
                .enumerate()
                .find_map(|(index, c)| if *c == *node { Some(index) } else { None })
        {
            self.children.swap_remove(index);
        }
    }

    /// Adds the node newChild to the end of the list of children of this node.
    /// If the newChild is already in the tree, it is first removed.
    pub fn append_child(&mut self, node: NodeRef) -> bool {
        self.remove_child(&node);
        self.children.push(node);
        true
    }

    /// Replaces the child node `old` with `new` in the list of children, and returns true if successful replaced.
    pub fn replace_child(&mut self, old: &NodeRef, new: NodeRef) -> bool {
        if let Some(index) =
            self.children
                .iter()
                .enumerate()
                .find_map(|(index, c)| if *c == *old { Some(index) } else { None })
        {
            self.children[index] = new;
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

macro_rules! impl_xml_node {
    ($ident: ident) => {
        impl<'a> std::ops::Deref for $ident<'a> {
            type Target = XmlNode;

            fn deref(&self) -> &Self::Target {
                &self.tree
            }
        }

        impl<'a> std::ops::DerefMut for $ident<'a> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.tree
            }
        }
    };
}

pub(super) use impl_xml_node;

#[derive(Debug, Default)]
pub struct XmlLeaf(Option<NodeRef>);

impl XmlLeaf {
    pub fn parent(&self) -> Option<&NodeRef> {
        self.0.as_ref()
    }
}

macro_rules! impl_xml_leaf {
    ($ident: ident) => {
        impl<'a> std::ops::Deref for $ident<'a> {
            type Target = XmlLeaf;

            fn deref(&self) -> &Self::Target {
                &self.leaf
            }
        }

        impl<'a> std::ops::DerefMut for $ident<'a> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.leaf
            }
        }
    };
}

pub(super) use impl_xml_leaf;
