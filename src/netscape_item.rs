use kuchiki::NodeRef;
use serde::Serialize;

use crate::bookmark::Bookmark;
use crate::folder::Folder;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum NetscapeItem {
    Subfolder(Folder),
    Shortcut(Bookmark),
}

impl NetscapeItem {
    pub fn from_node(node: &NodeRef) -> Option<Self> {
        if let Some(bookmark) = Bookmark::from_node(node) {
            Some(NetscapeItem::Shortcut(bookmark))
        } else if let Some(folder) = Folder::from_node(node) {
            Some(NetscapeItem::Subfolder(folder))
        } else {
            None
        }
    }
}

impl PartialEq for NetscapeItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NetscapeItem::Subfolder(f1), NetscapeItem::Subfolder(f2)) => f1 == f2,
            (NetscapeItem::Shortcut(b1), NetscapeItem::Shortcut(b2)) => b1 == b2,
            _ => false,
        }
    }
}
