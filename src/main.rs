use include_dir::{include_dir, Dir};
extern crate comrak;

use comrak::nodes::NodeValue;
use comrak::{format_html, parse_document, Arena, Options};

//static GIT_REMOTE: &str = "http://github.com/nostr-protocol/nips.git";

static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR");

fn replace_text(document: &str, orig_string: &str, replacement: &str) -> String {
    // The returned nodes are created in the supplied Arena, and are bound by its
    // lifetime.
    let arena = Arena::new();

    // Parse the document into a root `AstNode`
    let root = parse_document(&arena, document, &Options::default());

    // Iterate over all the descendants of root.
    for node in root.descendants() {
        if let NodeValue::Text(ref mut text) = node.data.borrow_mut().value {
            // If the node is a text node, perform the string replacement.
            *text = text.replace(orig_string, replacement)
        }
    }

    let mut html = vec![];
    format_html(root, &Options::default(), &mut html).unwrap();

    String::from_utf8(html).unwrap()
}

fn main() {
    // let doc = "This is my input.\n\n1. Also [my](#) input.\n2. Certainly *my*
    // input.\n"; let orig = "my";
    // let repl = "your";
    // let html = replace_text(&doc, &orig, &repl);
    // println!("{}", html);

    const BOUND: u8 = 50;
    print_entries::<{ BOUND }>();
}

fn print_entries<const BOUND: u8>() -> () {
    let mut count: u8 = 0;
    let glob = "**/*.md";
    let mut nip_vec = Vec::<String>::new(); // Create an empty Vec

    for entry in PROJECT_DIR.find(glob).unwrap() {
        count = count + 1;
        //println!("count={}", count);
        //println!("Found {}", entry.path().display());
        nip_vec.push((entry.path().display()).to_string());
        nip_vec.push("md content".to_string());
        let mut md_content = PROJECT_DIR.get_file(entry.path()).unwrap();
        let content = md_content.contents_utf8().unwrap();
        println!("\n{}", content);
    }
    for i in 1..count - 1 {
        //nip_vec.push(i * 2); // Add elements (i * 2) to the Vec
        //println!("Found {}", entry.path().display());
        //let README_MD = PROJECT_DIR.get_file("README.md").unwrap();
        //let readme = README_MD.contents_utf8().unwrap();
        //println!("readme={}", readme);
        //nip_vec.push((i * 1).to_string());
    }

    //println!("Vec: {:?}", nip_vec);
    //println!("BOUND={}", BOUND);
}
