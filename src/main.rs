use include_dir::{include_dir, Dir};
use std::path::Path;
extern crate comrak;
use comrak::nodes::NodeValue;
use comrak::{format_html, parse_document, Arena, Options};

static GIT_REMOTE: &str = "http://github.com/nostr-protocol/nips.git";

static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR");

fn replace_text(document: &str, orig_string: &str, replacement: &str) -> String {
    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
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

    // let doc = "This is my input.\n\n1. Also [my](#) input.\n2. Certainly *my* input.\n";
    // let orig = "my";
    // let repl = "your";
    // let html = replace_text(&doc, &orig, &repl);
    // println!("{}", html);

    print_entries();
}

fn print_entries() -> () {
    let glob = "**/*.md";
    for entry in PROJECT_DIR.find(glob).unwrap() {

        println!("Found {}", entry.path().display());

        let README_MD = PROJECT_DIR.get_file("README.md").unwrap();
        let readme = README_MD.contents_utf8().unwrap();
        println!("readme={}", readme);
    }
}