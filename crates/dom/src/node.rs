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
}

/// A ecs entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    pub id: usize,
    pub node_type: NodeType,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            id: 0,
            node_type: NodeType::Document,
        }
    }
}

impl From<(usize, NodeType)> for Node {
    fn from(value: (usize, NodeType)) -> Self {
        Self {
            id: value.0,
            node_type: value.1,
        }
    }
}
