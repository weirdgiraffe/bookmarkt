//! Contains the [Bookmark] model and its associated tests.
use askama::Template;
use kuchiki::NodeRef;
use serde::Serialize;

use crate::node_ref_ext::*;

/// Implements the specification of the `shortcut` item.
#[derive(Serialize, Builder, Clone, Debug, Default, Template)]
#[builder(setter(into))]
#[template(path = "bookmark.j2", escape = "none")]
pub struct Bookmark {
    /// The `href` attribute stores the url to the page of the shortcut.
    pub href: String,

    /// The `title` attribute stores the shortcut's title, it is the `<A/>` content.
    pub title: String,

    /// The `add_date` attribute is the date when the item was created (in UNIX time).
    #[builder(default)]
    pub add_date: String,

    /// The `last_visit` attribute is the date of the last visit to the site of the shortcut (in UNIX time).
    #[builder(default)]
    pub last_visit: String,

    /// The `last_modified` attribute is the date of the last modification of the item (in UNIX time).
    #[builder(default)]
    pub last_modified: String,

    /// The `icon_uri` attribute represents the url of the `favicon.ico` of the domain.
    #[builder(default)]
    pub icon_uri: String,

    /// The `icon` encodes an image with the format `data:image/{format};{encoding}`.
    /// It is usually a png encoded in base64.
    #[builder(default)]
    pub icon: String,
}

impl Bookmark {
    /// Creates a [Bookmark] model from a parsed HTML DOM.
    pub fn from_node(node: &NodeRef) -> Option<Self> {
        let mut bookmark = None;
        let mut builder = BookmarkBuilder::default();

        if node.is_element("DT") {
            let a = node.children().find(|n| n.is_element("A"));

            if let Some(node) = a {
                bookmark = Bookmark::from_node(&node);
            }
        } else if node.is_element("A") {
            if let Some(attribute) = node.select_attribute("HREF") {
                builder.href(attribute.value);
            }

            if let Some(attribute) = node.select_attribute("ADD_DATE") {
                builder.add_date(attribute.value);
            }

            if let Some(attribute) = node.select_attribute("LAST_VISIT") {
                builder.last_visit(attribute.value);
            }

            if let Some(attribute) = node.select_attribute("LAST_MODIFIED") {
                builder.last_modified(attribute.value);
            }

            if let Some(attribute) = node.select_attribute("ICON_URI") {
                builder.icon_uri(attribute.value);
            }

            if let Some(attribute) = node.select_attribute("ICON") {
                builder.icon(attribute.value);
            }

            builder.title(node.text_contents());

            if let Ok(built) = builder.build() {
                bookmark = Some(built);
            }
        }

        bookmark
    }
}

impl PartialEq for Bookmark {
    fn eq(&self, other: &Self) -> bool {
        self.href == other.href
            && self.add_date == other.add_date
            && self.last_visit == other.last_visit
            && self.last_modified == other.last_modified
            && self.title == other.title
    }
}

#[allow(dead_code)]
fn mock_bookmark() -> Bookmark {
    Bookmark {
        href: String::from("url"),
        add_date: String::from("date"),
        last_visit: String::from("date"),
        last_modified: String::from("date"),
        title: String::from("name"),
        icon_uri: String::from(""),
        icon: String::from("icon"),
    }
}

#[test]
fn should_render_bookmark_html() {
    let rendered = r#"<DT><A HREF="url" ADD_DATE="date" LAST_VISIT="date" LAST_MODIFIED="date" ICON="icon">name</A>"#;
    assert_eq!(mock_bookmark().render().unwrap(), rendered);
}

#[test]
fn should_parse_netscape_bookmark() {
    use kuchiki::parse_html;
    use kuchiki::traits::TendrilSink;

    let item = r#"
<DT><A HREF="url" ADD_DATE="date" LAST_VISIT="date"
LAST_MODIFIED="date" ICON="icon">name</A>"#;
    let a = parse_html().one(item).select_first("A").unwrap();

    assert_eq!(Bookmark::from_node(&a.as_node()).unwrap(), mock_bookmark())
}

#[test]
fn should_serialize_json_bookmark() {
    let json = r#"{"href":"url","title":"name","add_date":"date","last_visit":"date","last_modified":"date","icon_uri":"","icon":"icon"}"#;
    let bookmark = mock_bookmark();

    assert_eq!(serde_json::to_string(&bookmark).unwrap(), json)
}
