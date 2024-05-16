use std::path::Path;
use include_dir::{include_dir, Dir};
static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR");
fn main() {
    print_entries();
}
//#[cfg(feature = "glob")]
fn print_entries() -> () {
    let glob = "**/*.md";
    for entry in PROJECT_DIR.find(glob).unwrap() {
        println!("Found {}", entry.path().display());
    }
}
