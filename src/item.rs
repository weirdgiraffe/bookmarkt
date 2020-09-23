use kuchiki::NodeRef;
use serde::Serialize;

use crate::bookmark::Bookmark;
use crate::folder::Folder;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Item {
    Subfolder(Folder),
    Shortcut(Bookmark),
}

impl Item {
    pub fn from_node(node: &NodeRef) -> Option<Self> {
        if let Some(bookmark) = Bookmark::from_node(node) {
            Some(Item::Shortcut(bookmark))
        } else if let Some(folder) = Folder::from_node(node) {
            Some(Item::Subfolder(folder))
        } else {
            None
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::Subfolder(f1), Item::Subfolder(f2)) => f1 == f2,
            (Item::Shortcut(b1), Item::Shortcut(b2)) => b1 == b2,
            _ => false,
        }
    }
}
