use askama::Template;
use kuchiki::NodeRef;
use serde::Serialize;

use crate::node_ref_ext::*;

#[derive(Serialize, Builder, Clone, Debug, Default, Template)]
#[builder(setter(into))]
#[template(path = "bookmark.j2")]
pub struct Bookmark {
    href: String,
    title: String,
    #[builder(default)]
    add_date: String,
    #[builder(default)]
    last_visit: String,
    #[builder(default)]
    last_modified: String,
}

impl Bookmark {
    pub fn from_node(node: &NodeRef) -> Option<Self> {
        let mut bookmark = None;
        let mut builder = BookmarkBuilder::default();

        if node.is_element("DT") {
            if let Ok(a) = node.select_first("A") {
                bookmark = Bookmark::from_node(&a.as_node());
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

#[test]
fn render_bookmark_html() {
    let rendered =
        r#"<DT><A HREF="url" ADD_DATE="date" LAST_VISIT="date" LAST_MODIFIED="date">name</A>"#;
    let bookmark = Bookmark {
        href: String::from("url"),
        add_date: String::from("date"),
        last_visit: String::from("date"),
        last_modified: String::from("date"),
        title: String::from("name"),
    };

    assert_eq!(bookmark.render().unwrap(), rendered);
}

#[test]
fn parse_netscape_bookmark() {
    use kuchiki::parse_html;
    use kuchiki::traits::TendrilSink;

    let item = r#"
<DT><A HREF="url" ADD_DATE="date" LAST_VISIT="date"
LAST_MODIFIED="date">name</A>"#;
    let a = parse_html().one(item).select_first("A").unwrap();

    assert_eq!(
        Bookmark::from_node(&a.as_node()).unwrap(),
        Bookmark {
            href: String::from("url"),
            add_date: String::from("date"),
            last_visit: String::from("date"),
            last_modified: String::from("date"),
            title: String::from("name")
        }
    )
}

#[test]
fn serialize_json_bookmark() {
    let json = r#"{"href":"url","title":"name","add_date":"date","last_visit":"date","last_modified":"date"}"#;
    let bookmark = Bookmark {
        href: String::from("url"),
        add_date: String::from("date"),
        last_visit: String::from("date"),
        last_modified: String::from("date"),
        title: String::from("name"),
    };

    assert_eq!(serde_json::to_string(&bookmark).unwrap(), json)
}
