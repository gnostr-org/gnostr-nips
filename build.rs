use std::process::Command;
use std::{env, fs};

use include_dir::{include_dir, Dir};
//use std::path::Path;
use markdown::to_html;

static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR");

fn main() -> std::io::Result<()> {
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

    //    //pandoc README.md | sed 's/.md/.html/g'  > readme.html
    //
    //    let mut count: u8 = 0;
    //    let glob = "**/*.md";
    //    let mut nip_vec = Vec::<String>::new(); // Create an empty Vec
    //
    //    for entry in PROJECT_DIR.find(glob).unwrap() {
    //        count = count + 1;
    //        //println!("count={}", count);
    //        //println!("{}", entry.path().display());
    //        nip_vec.push((entry.path().display()).to_string().replace(".md",
    // ".html"));        //nip_vec.push("md content".to_string());
    //        let mut md_content = PROJECT_DIR.get_file(entry.path()).unwrap();
    //        let content = md_content.contents_utf8().unwrap();

    // Get the script name (replace "my_script.sh" with the actual script name)
    let script_name = "./script.sh";

    // Build the command
    let mut _command = Command::new(script_name);

    // Add arguments if needed (optional)
    // command.arg("argument1");
    // command.arg("argument2");

    Command::new(script_name)
        .current_dir(".")
        .spawn()
        .expect("script.sh command failed to start");

    //  // Execute the script
    //  match command.spawn() {
    //    Ok(mut child) => {
    //      // Wait for the script to finish
    //      child.wait().expect("script failed to run");
    //      println!("Script finished successfully!");
    //    }
    //    Err(err) => {
    //      println!("Error running script: {}", err);
    //    }
    //  }

    //println!("\n{}", content);
    //println!("{}", markdown::to_html(content));
    //let content_html = markdown::to_html(content);
    //fs::write(format!("{}.html", count), content_html)?;
    //}
    Ok(())
}
