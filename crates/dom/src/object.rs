use std::fmt::Display;

/// This corresponds to the DOM NodeType set of constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeType {
    Element = 1,
    Attribute = 2,
    Text = 3,
    CData = 4,
    EntityReference = 5,
    Entity = 6,
    ProcessingInstruction = 7,
    Comment = 8,
    Document = 9,
    DocumentType = 10,
    DocumentFragment = 11,
    Notation = 12,
    /// This is an extension type and is not part of the DOM standard.
    Namespace = 13,
}

impl NodeType {
    /// Returns true, if the node type is a leaf node.
    pub fn is_leaf(&self) -> bool {
        match self {
            NodeType::Element => false,
            NodeType::Attribute => false,
            NodeType::Text => true,
            NodeType::CData => true,
            NodeType::EntityReference => true,
            NodeType::Entity => false,
            NodeType::ProcessingInstruction => true,
            NodeType::Comment => true,
            NodeType::Document => false,
            NodeType::DocumentType => true,
            NodeType::DocumentFragment => false,
            NodeType::Notation => true,
            NodeType::Namespace => true,
        }
    }
}

/// A reference to a node of one `Document`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DOMObject {
    /// The reference id of the memory manager of the document to which this node belongs.
    id: usize,
    /// The [`node_type`](NodeType) of this node.
    pub node_type: NodeType,
}

impl DOMObject {
    #[allow(unused)]
    pub(crate) fn new(id: usize, node_type: NodeType) -> Self {
        Self { id, node_type }
    }
}

impl Display for DOMObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.node_type, self.id,)
    }
}

#[cfg(test)]
mod tests {

    use super::{DOMObject, NodeType};

    #[test]
    fn test_node() {
        println!("{}", DOMObject::new(1, NodeType::Attribute));
    }
}
