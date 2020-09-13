use kuchiki::NodeRef;
use std::io::Error;

use crate::node_ref_ext::*;

#[derive(Debug, Default)]
pub struct Bookmark {
    href: String,
    add_date: String,
    last_visit: String,
    last_modified: String,
    name: String,
}

impl Bookmark {
    fn from_node(node: &NodeRef) -> Result<Self, Error> {
        let mut href = String::new();
        let mut add_date = String::new();
        let mut last_visit = String::new();
        let mut last_modified = String::new();
        let mut name = String::new();

        if node.is_element("DT") {
            if let Ok(data) = node.select_first("A") {
                let a = data.as_node();

                if let Some(attribute) = a.get_attribute("HREF") {
                    href = attribute.value
                }

                if let Some(attribute) = a.get_attribute("ADD_DATE") {
                    add_date = attribute.value
                }

                if let Some(attribute) = a.get_attribute("LAST_VISIT") {
                    last_visit = attribute.value
                }

                if let Some(attribute) = a.get_attribute("LAST_MODIFIED") {
                    last_modified = attribute.value
                }

                name = a.text_contents();
            }
        }

        Ok(Bookmark {
            href: href,
            add_date: add_date,
            last_visit: last_visit,
            last_modified: last_modified,
            name: name,
        })
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
