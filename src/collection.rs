//! Contains all custom selectors for the Vec<Item>
use crate::item::Item;
use crate::Bookmark;
use crate::Folder;

/// Declares the selectors that collects all nested item of a specified type.
/// It implements a way to collect every items of type from the [Netscape] structure.
pub trait NestedCollection {
    /// Collects all nested [Bookmark]s of the folder
    fn shortcuts(&self) -> Vec<&Bookmark>;

    /// Collects all nested [Folder]s of the folder
    fn subfolders(&self) -> Vec<&Folder>;
}

impl NestedCollection for Vec<Item> {
    fn shortcuts(&self) -> Vec<&Bookmark> {
        let mut all = vec![];

        for item in self.iter() {
            match item {
                Item::Subfolder(f) => all.append(&mut f.children.shortcuts()),
                Item::Shortcut(b) => all.push(b),
            }
        }

        all
    }

    fn subfolders(&self) -> Vec<&Folder> {
        let mut all = vec![];

        for item in self.iter() {
            if let Item::Subfolder(f) = item {
                all.push(f);
                all.append(&mut f.children.subfolders());
            }
        }

        all
    }
}

#[test]
fn should_get_all_nested_items() {
    use crate::bookmark::BookmarkBuilder;
    use crate::folder::FolderBuilder;

    let b0 = BookmarkBuilder::default()
        .href(String::from("test0"))
        .title(String::from("test0"))
        .build()
        .unwrap();

    let b1 = BookmarkBuilder::default()
        .href(String::from("test1"))
        .title(String::from("test1"))
        .build()
        .unwrap();

    let f1 = FolderBuilder::default()
        .children(vec![Item::Shortcut(b1.clone())])
        .build()
        .unwrap();

    let f0 = FolderBuilder::default()
        .children(vec![Item::Subfolder(f1.clone())])
        .build()
        .unwrap();

    let folder = FolderBuilder::default()
        .children(vec![
            Item::Shortcut(b0.clone()),
            Item::Subfolder(f0.clone()),
        ])
        .build()
        .unwrap();

    assert_eq!(folder.children.shortcuts(), vec![&b0, &b1]);
    assert_eq!(folder.children.subfolders(), vec![&f0, &f1]);
}
