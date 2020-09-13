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
