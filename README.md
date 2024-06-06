# A Netscape Bookmark File parser

That is a fork of [bookmarkt](https://crates.io/crates/bookmarkt) crate.
Forked from the [original repository](https://git.sr.ht/~vlnk/bookmarkt).

I needed to deduplicate my bookmarks, but unfortunately, original crate did not
exposed `item::Item` struct, which made my task of deduplication quite
problematic. 

Here is the example of how to deduplicate your browser bookmarks

dependencies:

```toml
[dependencies]
anyhow = "1.0.86"

[dependencies.bookmarkt]
git = "https://github.com/weirdgiraffe/bookmarkt"
version = "0.0.4"
```

and the source code:

```rust
use anyhow::Result;
use bookmarkt::{Item, Netscape};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn filter_bookmark(uniq: &mut HashSet<String>, item: Item) -> Option<Item> {
    let bookmark = item.take_shortcut()?;
    if uniq.insert(bookmark.href.clone()) {
        return Some(item.to_owned());
    }
    None
}

fn filter_subfolder(uniq: &mut HashSet<String>, item: Item) -> Option<Item> {
    let mut folder = item.take_subfolder()?.to_owned();
    let l: Vec<_> = folder
        .children
        .into_iter()
        .filter_map(|item| {
            if item.is_shortcut() {
                return filter_bookmark(uniq, item);
            }
            filter_subfolder(uniq, item)
        })
        .collect();
    folder.children = l;
    Some(Item::Subfolder(folder))
}

fn main() -> Result<()> {
    let path = Path::new("bookmarks.html");
    let mut doc = Netscape::from_file(path)?;
    let mut uniq: HashSet<String> = HashSet::new();

    let l: Vec<_> = doc
        .children
        .into_iter()
        .filter_map(|item| {
            if item.is_shortcut() {
                return filter_bookmark(&mut uniq, item);
            }
            filter_subfolder(&mut uniq, item)
        })
        .collect();
    doc.children = l;

    let mut file = File::create("fixed.html")?;
    file.write_all(doc.to_html()?.as_bytes())?;
    Ok(())
}
```


