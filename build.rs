use std::env;
//use std::path::Path;
use std::process::Command;

fn main() {
    let _out_dir = env::var("OUT_DIR").unwrap();

    Command::new("git")
        .args(&[
            "remote",
            "add",
            "gnostr-org",
            "git@github.com:gnostr-org/nips.git",
        ])
        .status()
        .unwrap();
    Command::new("git")
        .args(&[
            "remote",
            "add",
            "upstream",
            "git@github.com:nostr-protocol/nips.git",
        ])
        .status()
        .unwrap();
    Command::new("git")
        .args(&["fetch", "--all"])
        .status()
        .unwrap();
    Command::new("git")
        .args(&["fetch", "--all", "--tags"])
        .status()
        .unwrap();
    Command::new("git")
        .args(&["rebase", "upstream/master"])
        .status()
        .unwrap();

    Command::new("./script.sh");
}
