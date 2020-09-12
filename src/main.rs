use kuchiki::parse_html;
use kuchiki::traits::TendrilSink;
use kuchiki::NodeRef;

use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct BookmarkImport {
    pub dom: NodeRef,
}

impl BookmarkImport {
    pub fn new(handle: NodeRef) -> Self {
        BookmarkImport { dom: handle }
    }

    pub fn from_file(path: &Path) -> NodeRef {
        parse_html().from_utf8().from_file(path).unwrap()
    }
}

fn main() {
    println!(
        "{:?}",
        BookmarkImport::from_file(Path::new("./res/netscape.html")).to_string()
    );
}
