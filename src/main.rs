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
        //println!("{}", entry.path().display());
        nip_vec.push((entry.path().display()).to_string());
        //nip_vec.push("md content".to_string());
        let mut md_content = PROJECT_DIR.get_file(entry.path()).unwrap();
        let content = md_content.contents_utf8().unwrap();
        println!("\n{}", content);
    }

    println!("count={}", count); //original document count

    //README.md
    let (first, remainder) = nip_vec.split_at(1);
    //println!("a={:}", format!("{:?}",a[0]));
    let mut readme = PROJECT_DIR.get_file(first[0].clone()).unwrap();
    let readme_md = readme.contents_utf8().unwrap();
    println!("\n{}", readme_md);

    count = count - 1;
    println!("count={}", count);

    //second
    //BREAKING.md
    let mut breaking = PROJECT_DIR.get_file(remainder[0].clone()).unwrap();
    let breaking_md = breaking.contents_utf8().unwrap();
    println!("\n{}", breaking_md);

    //REMAINDERS
    //count variable based on new documents added to nips repo

    //traverse documents from last to no more remainders
    count = count - 1;

    //loop {
    //    if count > 1 {
    //        print!("count={}", count);
    //    }
    //        count = count - 1;
    //}
    println!("count={}", count);
    let (last, remainder) = remainder.split_at(1);
    let mut last = PROJECT_DIR.get_file(remainder[0].clone()).unwrap();
    let last_md = last.contents_utf8().unwrap();
    //println!("\nlast_md={}", last_md);
}
