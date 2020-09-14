use kuchiki::{Attribute, NodeRef};

/// Augments the [NodeRef] struct with conveniant helpers
pub trait NodeRefExt {
    fn select_text(&self, selector: &str) -> Option<String>;
    fn is_element(&self, local_name: &str) -> bool;
    fn select_attribute(&self, tag_name: &str) -> Option<Attribute>;
}

impl NodeRefExt for NodeRef {
    fn select_text(&self, selector: &str) -> Option<String> {
        let mut content = None;

        if let Ok(selection) = self.select(selector) {
            let nodes = selection.collect::<Vec<_>>();

            if let Some(child) = nodes[0].as_node().first_child() {
                content = Some(child.text_contents());
            }
        }

        content
    }

    fn is_element(&self, tag_name: &str) -> bool {
        let mut is_element = false;

        if let Some(element) = self.as_element() {
            let local_name = &element.name.local;

            if local_name.to_ascii_uppercase() == tag_name.to_uppercase() {
                is_element = true
            }
        }

        is_element
    }

    fn select_attribute(&self, attribute_name: &str) -> Option<Attribute> {
        let mut attribute = None;

        if let Some(element) = self.as_element() {
            let attributes = element.attributes.borrow();

            for (exp_name, attr) in &attributes.map {
                if exp_name.local.to_ascii_uppercase() == attribute_name.to_uppercase() {
                    attribute = Some(attr.clone());
                }
            }
        }

        attribute
    }
}

#[test]
fn check_dl_element() {
    use kuchiki::parse_html;
    use kuchiki::traits::TendrilSink;

    let dl = parse_html().one(r"<DL></DL>").select_first("DL").unwrap();

    assert_eq!(dl.as_node().is_element("DL"), true);
    assert_eq!(dl.as_node().is_element("DT"), false);
}

#[test]
fn select_href_attribute() {
    use kuchiki::parse_html;
    use kuchiki::traits::TendrilSink;

    let a = parse_html()
        .one(r#"<A HREF="Test"/>"#)
        .select_first("A")
        .unwrap();

    let attribute = a.as_node().select_attribute("HREF").unwrap();
    assert_eq!(attribute.value, "Test")
}

#[test]
fn ignore_selector_case() {
    use kuchiki::parse_html;
    use kuchiki::traits::TendrilSink;

    let selector = "TITLE";

    let upper = r"<TITLE>Test</Title>";
    let capital = r"<Title>Test</Title>";
    let lower = r"<title>Test</title>";

    let from_upper = parse_html().one(upper).select_text(selector).unwrap();
    let from_capital = parse_html().one(capital).select_text(selector).unwrap();
    let from_lower = parse_html().one(lower).select_text(selector).unwrap();

    assert_eq!(from_upper, from_capital);
    assert_eq!(from_capital, from_lower);
}
