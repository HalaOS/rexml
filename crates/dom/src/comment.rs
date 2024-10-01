use std::borrow::Cow;

use crate::node::DOMLeaf;

pub(crate) struct DOMComment<'a> {
    leaf: DOMLeaf,
    data: Cow<'a, str>,
}

impl<'a> DOMComment<'a> {
    pub(crate) fn new(data: Cow<'a, str>) -> Self {
        Self {
            leaf: Default::default(),
            data,
        }
    }
}

impl<'a> DOMComment<'a> {
    /// Returns a new owning DOMComment from the given existing one.
    pub fn into_owned(self) -> DOMComment<'static> {
        DOMComment::<'static> {
            leaf: self.leaf,
            data: self.data.into_owned().into(),
        }
    }
}
