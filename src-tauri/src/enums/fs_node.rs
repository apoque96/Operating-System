use std::collections::HashMap;

use crate::enums::fs_node_kind::FsNodeKind;

#[derive(Debug, Clone)]
pub struct FsNode {
    pub name:     String,
    pub kind:     FsNodeKind,
    /// Only populated for Dir nodes.
    pub children: HashMap<String, FsNode>,
}

impl FsNode {
    pub fn new_dir(name: impl Into<String>) -> Self {
        FsNode { name: name.into(), kind: FsNodeKind::Dir, children: HashMap::new() }
    }

    pub fn new_file(name: impl Into<String>) -> Self {
        FsNode { name: name.into(), kind: FsNodeKind::File, children: HashMap::new() }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self.kind, FsNodeKind::Dir)
    }
}