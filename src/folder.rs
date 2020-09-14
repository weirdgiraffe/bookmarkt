use kuchiki::NodeRef;

use crate::bookmark::Bookmark;
use crate::node_ref_ext::*;

#[derive(Clone, Builder, Debug, Default)]
#[builder(setter(into))]
pub struct Folder {
    title: String,
    #[builder(default)]
    add_date: String,
    #[builder(default)]
    bookmarks: Vec<Bookmark>,
    #[builder(default)]
    folders: Vec<Folder>,
}

impl Folder {
    pub fn from_node(node: &NodeRef) -> Option<Self> {
        let mut folder = None;

        if node.is_element("DT") {
            if let Ok(h3) = node.select_first("H3") {
                folder = Folder::from_node(&h3.as_node());
            }
        } else if node.is_element("H3") {
            let mut bookmarks = vec![];
            let mut builder = FolderBuilder::default();
            let mut folders = vec![];

            if let Some(attribute) = node.select_attribute("ADD_DATE") {
                builder.add_date(attribute.value);
            }

            builder.title(node.text_contents());

            for sibling in node.following_siblings() {
                if sibling.is_element("DL") {
                    for child in sibling.children() {
                        if let Ok(item) = Bookmark::from_node(&child) {
                            bookmarks.push(item)
                        } else if let Some(item) = Folder::from_node(&child) {
                            folders.push(item)
                        }
                    }
                }
            }

            builder.bookmarks(bookmarks).folders(folders);

            if let Ok(built) = builder.build() {
                folder = Some(built);
            }
        }

        folder
    }
}

impl PartialEq for Folder {
    fn eq(&self, other: &Self) -> bool {
        self.add_date == other.add_date
            && self.title == other.title
            && self.bookmarks == other.bookmarks
            && self.folders == other.folders
    }
}

#[test]
fn parse_netscape_empty_folder() {
    use kuchiki::parse_html;
    use kuchiki::traits::TendrilSink;

    let item = r#"
    <DT><H3 FOLDED ADD_DATE="date">title</H3>
    <DL><p>
    </DL><p>"#;
    let h3 = parse_html().one(item).select_first("H3").unwrap();

    assert_eq!(
        Folder::from_node(&h3.as_node()).unwrap(),
        Folder {
            title: String::from("title"),
            add_date: String::from("date"),
            bookmarks: vec![],
            folders: vec![]
        }
    )
}

#[test]
fn parse_netscape_nested_folders() {
    use kuchiki::parse_html;
    use kuchiki::traits::TendrilSink;

    let item = r#"
    <DT><H3>nested0</H3>
    <DL><p>
    <DT><H3>nested1</H3>
    <DL><p>
    <DT><H3>nested2</H3>
    <DL><p>
    <DT><H3>nested3</H3>
    </DL><p>
    </DL><p>
    </DL><p>"#;
    let dt = parse_html().one(item).select_first("DT").unwrap();

    assert_eq!(
        Folder::from_node(&dt.as_node()).unwrap(),
        FolderBuilder::default()
            .title("nested0")
            .folders(vec![FolderBuilder::default()
                .title("nested1")
                .folders(vec![FolderBuilder::default()
                    .title("nested2")
                    .folders(vec![FolderBuilder::default()
                        .title("nested3")
                        .folders(vec![])
                        .build()
                        .unwrap()])
                    .build()
                    .unwrap()])
                .build()
                .unwrap()])
            .build()
            .unwrap()
    )
}
