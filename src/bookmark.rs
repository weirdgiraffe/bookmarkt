use kuchiki::NodeRef;
use std::io::{Error, ErrorKind};

use crate::node_ref_ext::*;

#[derive(Builder, Debug, Default)]
#[builder(setter(into))]
pub struct Bookmark {
    href: String,
    name: String,
    #[builder(default)]
    add_date: String,
    #[builder(default)]
    last_visit: String,
    #[builder(default)]
    last_modified: String,
}

impl Bookmark {
    pub fn from_node(node: &NodeRef) -> Result<Self, Error> {
        let mut builder = BookmarkBuilder::default();

        if node.is_element("DT") {
            if let Ok(data) = node.select_first("A") {
                let a = data.as_node();

                if let Some(attribute) = a.select_attribute("HREF") {
                    builder.href(attribute.value);
                }

                if let Some(attribute) = a.select_attribute("ADD_DATE") {
                    builder.add_date(attribute.value);
                }

                if let Some(attribute) = a.select_attribute("LAST_VISIT") {
                    builder.last_visit(attribute.value);
                }

                if let Some(attribute) = a.select_attribute("LAST_MODIFIED") {
                    builder.last_modified(attribute.value);
                }

                builder.name(a.text_contents());
            }
        }

        builder
            .build()
            .or_else(|msg| Err(Error::new(ErrorKind::InvalidInput, msg)))
    }
}

impl PartialEq for Bookmark {
    fn eq(&self, other: &Self) -> bool {
        self.href == other.href
            && self.add_date == other.add_date
            && self.last_visit == other.last_visit
            && self.last_modified == other.last_modified
            && self.name == other.name
    }
}

#[test]
fn parse_netscape_item() {
    use kuchiki::parse_html;
    use kuchiki::traits::TendrilSink;

    let item = r#"
<DT><A HREF="url" ADD_DATE="date" LAST_VISIT="date"
LAST_MODIFIED="date">name</A>"#;
    let dl = parse_html().one(item).select_first("DT").unwrap();

    assert_eq!(
        Bookmark::from_node(&dl.as_node()).unwrap(),
        Bookmark {
            href: String::from("url"),
            add_date: String::from("date"),
            last_visit: String::from("date"),
            last_modified: String::from("date"),
            name: String::from("name")
        }
    )
}
