use std::io::Error;
use std::path::Path;

use kuchiki::parse_html;
use kuchiki::traits::TendrilSink;
use kuchiki::NodeRef;

use crate::node_ref_ext::*;

/// Implements the [`Netscape Bookmark File format`].
///
/// The [Netscape] parses the header of a Bookmark file, it gets the content of the tags
/// `title` and `h1` that are expected to the first tags of a bookmark document.
///
/// This specification is implemented by most of the common browser :
/// - [Firefox](https://support.mozilla.org/en-US/kb/export-firefox-bookmarks-to-backup-or-transfer)
/// - [Chrome](https://support.google.com/chrome/answer/96816?hl=en)
/// - [Edge](https://support.microsoft.com/en-ph/help/4077936/microsoft-edge-import-favorites)
///
/// This parser isn't strict and will not fail if the specification isn't respected : it implements [Default] trait.
///
/// [`Netscape Bookmark File format`]: https://docs.microsoft.com/en-us/previous-versions/windows/internet-explorer/ie-developer/platform-apis/aa753582(v=vs.85)?redirectedfrom=MSDN
#[derive(Debug, Default)]
pub struct Netscape {
    pub title: String,
    pub h1: String,
}

impl Netscape {
    pub fn from_node(node: &NodeRef) -> Result<Self, Error> {
        let mut title = String::new();
        let mut h1 = String::new();

        if let Some(content) = node.select_text("TITLE") {
            title = content
        }

        if let Some(content) = node.select_text("H1") {
            h1 = content
        }

        Ok(Netscape {
            title: title,
            h1: h1,
        })
    }

    pub fn from_string(raw: &str) -> Result<Self, Error> {
        let node = parse_html().one(raw);
        Netscape::from_node(&node)
    }

    pub fn from_file(path: &Path) -> Result<Self, Error> {
        parse_html()
            .from_utf8()
            .from_file(path)
            .and_then(|node| Netscape::from_node(&node))
    }

    pub fn to_string(&self) -> String {
        String::new()
    }
}

impl PartialEq for Netscape {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.h1 == other.h1
    }
}

#[test]
fn parse_netscape_header() {
    let html = r"
<!DOCTYPE NETSCAPE-Bookmark-file-1>
    <!--This is an automatically generated file.
    It will be read and overwritten.
    Do Not Edit! -->
    <Title>Collection Title</Title>
    <H1>Collection Head</H1>";
    let netscape = Netscape::from_string(html).unwrap();

    assert_eq!(netscape.title, "Collection Title");
    assert_eq!(netscape.h1, "Collection Head");
}

#[test]
fn parse_netscape_file() {
    use std::path::Path;

    let path = Path::new("./res/netscape.html");
    let label = String::from("Bookmarks");

    assert_eq!(
        Netscape::from_file(path).unwrap(),
        Netscape {
            title: label.clone(),
            h1: label
        }
    );
}
