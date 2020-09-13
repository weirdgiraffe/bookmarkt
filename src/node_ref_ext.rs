use kuchiki::NodeRef;

pub trait NodeRefExt {
    fn select_text(&self, selector: &str) -> Option<String>;
}

impl NodeRefExt for NodeRef {
    fn select_text(&self, selector: &str) -> Option<String> {
        let mut content = None;

        if let Ok(selection) = self.select(selector) {
            let nodes = selection.collect::<Vec<_>>();

            if let Some(child) = nodes[0].as_node().first_child() {
                if let Some(text) = child.as_text() {
                    content = Some(String::from(&**text.borrow()));
                }
            }
        }

        content
    }
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
