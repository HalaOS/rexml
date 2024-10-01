use std::borrow::Cow;

use crate::{
    comment::DOMComment, namespace::DOMNamespace, DOMExceptionCode, DOMObject, Error, NodeType,
    Result,
};

/// This corresponds to DOM `Document`.
pub struct Document<'a> {
    /// node list for comments.
    comments: Vec<DOMComment<'a>>,
    /// node list for namespaces.
    namespaces: Vec<DOMNamespace<'a>>,
}

impl<'a> Document<'a> {
    /// Returns a new owning Document from the given existing one.
    pub fn into_owned(self) -> Document<'static> {
        Document {
            comments: self
                .comments
                .into_iter()
                .map(|item| item.into_owned())
                .collect(),
            namespaces: self
                .namespaces
                .into_iter()
                .map(|item| item.into_owned())
                .collect(),
        }
    }
}

/// factory methods.
impl<'a> Document<'a> {
    /// Create a new comment node but does not attach to any parent node.
    pub fn create_comment<D: Into<Cow<'a, str>>>(&mut self, data: D) -> DOMObject {
        let dom_object = DOMObject::new(self.comments.len(), NodeType::Comment);

        let comment = DOMComment::new(data.into());

        self.comments.push(comment);

        dom_object
    }

    /// Create a new namespace node.
    pub fn create_namespace<P, H>(&mut self, prefix: P, href: H) -> DOMObject
    where
        P: Into<Cow<'a, str>>,
        H: Into<Cow<'a, str>>,
    {
        let dom_object = DOMObject::new(self.comments.len(), NodeType::Comment);

        let namespace = DOMNamespace::new(prefix.into(), href.into());

        self.namespaces.push(namespace);

        dom_object
    }
}

impl<'a> Document<'a> {
    /// Attach child to one node.
    pub fn append_child(&mut self, parent: &DOMObject, _child: DOMObject) -> Result<'a, ()> {
        if parent.node_type.is_leaf() {
            return Err(Error::DOMException(DOMExceptionCode::HIERARCHY_REQUEST_ERR));
        }

        match parent.node_type {
            _ => {
                unimplemented!()
            }
        }
    }
}
