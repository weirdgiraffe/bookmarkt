use std::io::Error;
use std::path::Path;

use kuchiki::parse_html;
use kuchiki::traits::TendrilSink;
use kuchiki::NodeRef;

use crate::node_ref_ext::*;

#[derive(Debug)]
pub struct NetscapeBookmark {
    pub title: String,
    pub h1: String,
}

impl NetscapeBookmark {
    pub fn new(title: String, h1: String) -> Self {
        NetscapeBookmark {
            title: title,
            h1: h1,
        }
    }

    pub fn from_node(node: NodeRef) -> Result<Self, Error> {
        let mut title = String::new();
        let mut h1 = String::new();

        if let Some(content) = node.select_text("TITLE") {
            title = content
        }

        if let Some(content) = node.select_text("H1") {
            h1 = content
        }

        Ok(NetscapeBookmark::new(title, h1))
    }

    pub fn from_string(raw: &str) -> Result<Self, Error> {
        let node = parse_html().one(raw);
        NetscapeBookmark::from_node(node)
    }

    pub fn from_file(path: &Path) -> Result<Self, Error> {
        parse_html()
            .from_utf8()
            .from_file(path)
            .and_then(|node| NetscapeBookmark::from_node(node))
            .and_then(|netscape| Ok(netscape))
    }

    pub fn to_string(&self) -> String {
        String::new()
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
    <H1>Collection Head</H1>
";
    let netscape = NetscapeBookmark::from_string(html).unwrap();
    assert_eq!(netscape.title, "Collection Title");
    assert_eq!(netscape.h1, "Collection Head");
}
