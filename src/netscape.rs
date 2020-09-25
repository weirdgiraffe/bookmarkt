use askama::Template;
use std::io::Error;
use std::path::Path;

use kuchiki::parse_html;
use kuchiki::traits::TendrilSink;
use kuchiki::NodeRef;

use serde::Serialize;

use crate::item::Item;
use crate::node_ref_ext::*;

/// Implements the [Netscape Bookmark File format].
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
/// [Netscape Bookmark File format]: https://docs.microsoft.com/en-us/previous-versions/windows/internet-explorer/ie-developer/platform-apis/aa753582(v=vs.85)?redirectedfrom=MSDN
#[derive(Serialize, Debug, Template)]
#[template(path = "netscape.j2", escape = "none")]
pub struct Netscape {
    pub title: String,
    pub h1: String,
    pub children: Vec<Item>,
}

impl Netscape {
    pub fn from_node(node: &NodeRef) -> Result<Self, Error> {
        let mut title = String::new();
        let mut h1 = String::new();
        let mut children = vec![];

        let mut head = None;
        let mut body = None;

        let html = node.children().find(|n| n.is_element("HTML"));

        if let Some(root) = html {
            for child in root.children() {
                if child.is_element("HEAD") {
                    head = Some(child);
                } else if child.is_element("BODY") {
                    body = Some(child);
                }
            }
        }

        if let Some(root) = head {
            for child in root.children() {
                if child.is_element("TITLE") {
                    title = child.text_contents();
                }
            }
        }

        if let Some(root) = body {
            for child in root.children() {
                if child.is_element("H1") {
                    h1 = child.text_contents();
                } else if child.is_element("DL") {
                    for sub in child.children() {
                        if let Some(item) = Item::from_node(&sub) {
                            children.push(item);
                        }
                    }
                }
            }
        }

        Ok(Netscape {
            title: title,
            h1: h1,
            children: children,
        })
    }

    pub fn from_html(raw: &str) -> Result<Self, Error> {
        let node = parse_html().one(raw);
        Netscape::from_node(&node)
    }

    pub fn from_file(path: &Path) -> Result<Self, Error> {
        parse_html()
            .from_utf8()
            .from_file(path)
            .and_then(|node| Netscape::from_node(&node))
    }

    pub fn to_html(&self) -> Result<String, askama::Error> {
        self.render()
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl PartialEq for Netscape {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.h1 == other.h1 && self.children == other.children
    }
}

#[allow(dead_code)]
fn sanitize_file(path: &Path) -> String {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(path).unwrap();
    let mut contents = String::new();

    for line in BufReader::new(file).lines() {
        if let Ok(content) = line {
            contents.push_str(content.trim());
        }
    }

    contents
}

#[allow(dead_code)]
fn sanitize_string(string: String) -> String {
    let mut contents = String::new();

    for line in string.lines() {
        contents.push_str(line.trim());
    }

    contents
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
    let netscape = Netscape::from_html(html).unwrap();

    assert_eq!(netscape.title, "Collection Title");
    assert_eq!(netscape.h1, "Collection Head");
}

#[test]
fn parse_netscape_file() {
    use crate::bookmark::BookmarkBuilder;

    let path = Path::new("./res/netscape.html");
    let label = String::from("Bookmarks");

    let b1 = BookmarkBuilder::default()
        .href("https://framasoft.org/")
        .add_date("1466009059")
        .title("Framasoft ~ Page portail du réseau")
        .build()
        .unwrap();

    let b2 = BookmarkBuilder::default()
        .href("https://www.kernel.org/")
        .add_date("1466009167")
        .title("The Linux Kernel Archives")
        .build()
        .unwrap();

    let netscape = Netscape {
        title: label.clone(),
        h1: label,
        children: vec![Item::Shortcut(b1), Item::Shortcut(b2)],
    };

    assert_eq!(Netscape::from_file(path).unwrap(), netscape);
}

#[test]
fn serialize_json_netscape() {
    let b1 = r#"{"href":"https://framasoft.org/","title":"Framasoft ~ Page portail du réseau","add_date":"1466009059","last_visit":"","last_modified":"","icon":""}"#;
    let b2 = r#"{"href":"https://www.kernel.org/","title":"The Linux Kernel Archives","add_date":"1466009167","last_visit":"","last_modified":"","icon":""}"#;

    let json = format!(
        r#"{{"title":"Bookmarks","h1":"Bookmarks","children":[{},{}]}}"#,
        b1, b2
    );

    let path = Path::new("./res/netscape.html");
    let netscape = Netscape::from_file(path).unwrap();

    assert_eq!(netscape.to_json().unwrap(), json)
}

#[test]
fn render_netscape_html() {
    use crate::bookmark::BookmarkBuilder;
    use std::fs;

    let label = String::from("Bookmarks");
    let path = Path::new("./res/netscape.html");

    let b1 = BookmarkBuilder::default()
        .href("https://framasoft.org/")
        .add_date("1466009059")
        .title("Framasoft ~ Page portail du réseau")
        .build()
        .unwrap();

    let b2 = BookmarkBuilder::default()
        .href("https://www.kernel.org/")
        .add_date("1466009167")
        .title("The Linux Kernel Archives")
        .build()
        .unwrap();

    let netscape = Netscape {
        title: label.clone(),
        h1: label,
        children: vec![Item::Shortcut(b1), Item::Shortcut(b2)],
    };

    assert_eq!(
        netscape.to_html().unwrap(),
        fs::read_to_string(path).unwrap().trim()
    )
}

#[test]
fn roundtrip_chromium_html() {
    let path = Path::new("./res/chromium.html");
    let chromium = Netscape::from_file(path).unwrap();

    assert_eq!(
        sanitize_string(chromium.to_html().unwrap()),
        sanitize_file(path)
    )
}
