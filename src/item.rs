//! Contains the [Item] enum that dispatches all the implementations of the items.
use kuchiki::NodeRef;
use serde::Serialize;

use crate::bookmark::Bookmark;
use crate::folder::Folder;

/// Represents all available item types of the Microsoft's Netscape Bookmark File format.
///
/// * TODO The `feed` item should represent a RSS feed.
/// * TODO The `web slice` item represents a legacy [Web Slice] object.
///
/// [Web Slice]: https://docs.microsoft.com/en-us/previous-versions/windows/desktop/cc956158(v=vs.85)
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Item {
    /// The `Subfolder` item is implemented by the [Folder] struct.
    Subfolder(Folder),

    /// The `Shortcut` item is represented by a [Bookmark].
    Shortcut(Bookmark),
}

impl Item {
    /// Creates a [Folder] or a [Bookmark] models from any given item.
    pub fn from_node(node: &NodeRef) -> Option<Self> {
        if let Some(bookmark) = Bookmark::from_node(node) {
            Some(Item::Shortcut(bookmark))
        } else if let Some(folder) = Folder::from_node(node) {
            Some(Item::Subfolder(folder))
        } else {
            None
        }
    }

    /// Checks if the item is a shortcut
    pub fn is_shortcut(&self) -> bool {
        match self {
            Item::Subfolder(_) => false,
            Item::Shortcut(_) => true,
        }
    }

    /// Takes a [Bookmark] out of the shortcut item
    pub fn take_shortcut(&self) -> Option<&Bookmark> {
        match self {
            Item::Subfolder(_) => None,
            Item::Shortcut(bookmark) => Some(bookmark),
        }
    }

    /// Checks if the item is a subfolder
    pub fn is_subfolder(&self) -> bool {
        match self {
            Item::Subfolder(_) => true,
            Item::Shortcut(_) => false,
        }
    }

    /// Takes a [Folder] out of the subfolder item
    pub fn take_subfolder(&self) -> Option<&Folder> {
        match self {
            Item::Subfolder(folder) => Some(folder),
            Item::Shortcut(_) => None,
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
