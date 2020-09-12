use kuchiki::parse_html;
use kuchiki::traits::TendrilSink;
use kuchiki::NodeRef;

use std::fs;
use std::io::Error;
use std::path::Path;

#[derive(Debug)]
pub struct Bookmarkt {
    pub dom: NodeRef,
}

impl Bookmarkt {
    pub fn new(handle: NodeRef) -> Self {
        Bookmarkt { dom: handle }
    }

    pub fn from_file(path: &Path) -> Result<Self, Error> {
        parse_html()
            .from_utf8()
            .from_file(path)
            .and_then(|dom| Ok(Bookmarkt::new(dom)))
    }

    pub fn to_string(&self) -> String {
        self.dom.to_string()
    }
}

const NETSCAPE_FILE: &str = "./res/netscape.html";

fn main() {
    let path = Path::new(NETSCAPE_FILE);
    let bookmarkt = Bookmarkt::from_file(path).unwrap();
    println!("{:?}", bookmarkt.to_string());

    let contents = fs::read_to_string(path).unwrap();
    println!("{:?}", contents)
}

#[test]
fn load_file_with_kuchiki() {
    let path = Path::new(NETSCAPE_FILE);
    let bookmarkt = Bookmarkt::from_file(path).unwrap();
    assert_eq!(bookmarkt.to_string().is_empty(), false);
}
