use crate::enums::fs_node::FsNode;

/// Holds the root node and tracks the current working directory
/// as a list of path segments from root.
pub struct FileSystem {
    pub root: FsNode,
    /// Stack of directory names from root to cwd (root itself is not included).
    pub cwd:  Vec<String>,
}

impl FileSystem {
    /// Create a filesystem with an empty root directory.
    pub fn new() -> Self {
        FileSystem { root: FsNode::new_dir("/"), cwd: Vec::new() }
    }

    /// Resolve a shared reference to the current working directory node.
    pub fn current_dir(&self) -> &FsNode {
        let mut node = &self.root;
        for segment in &self.cwd {
            node = node.children.get(segment)
                .expect("cwd path is broken (shared)");
        }
        node
    }

    /// Resolve a mutable reference to the current working directory node.
    pub fn current_dir_mut(&mut self) -> &mut FsNode {
        let mut node = &mut self.root;
        for segment in &self.cwd {
            node = node.children.get_mut(segment)
                .expect("cwd path is broken (mut)");
        }
        node
    }

    /// Absolute path string for the cwd, e.g. "/home/user".
    pub fn pwd_string(&self) -> String {
        if self.cwd.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", self.cwd.join("/"))
        }
    }
}