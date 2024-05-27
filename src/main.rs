extern crate comrak;
extern crate include_dir;

use comrak::nodes::NodeValue;
use comrak::{format_html, parse_document, Arena, Options};
use include_dir::{include_dir, Dir};
use markdown::to_html;
use nips::*;

//static GIT_REMOTE: &str = "http://github.com/nostr-protocol/nips.git";

static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR");

//#![windows_subsystem = "windows"]

// fn webview() {
//     let html = format!(
//         r#"
// 		<!doctype html>
// 		<html>
// 			<head>
// 				{styles}
// 			</head>
// 			<body>
// 				<!--[if lt IE 9]>
// 				<div class="ie-upgrade-container">
// 					<p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
// 					<a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
// 				</div>
// 				<![endif]-->
// 				<!--[if gte IE 9 | !IE ]> <!-->
// 				{scripts}
// 				<![endif]-->
// 			</body>
// 		</html>
// 		"#,
//         styles = inline_style(include_str!("styles.css")),
//         scripts =
//             inline_script(include_str!("picodom.js")) + &inline_script(include_str!("app.js")),
//     );
// 
//     let mut webview = nips::builder()
//         .title("Rust Todo App")
//         .content(Content::Html(html))
//         .size(320, 480)
//         .resizable(false)
//         .debug(true)
//         .user_data(vec![])
//         .invoke_handler(|webview, arg| {
//             use Cmd::*;
// 
//             let tasks_len = {
//                 let tasks = webview.user_data_mut();
// 
//                 match serde_json::from_str(arg).unwrap() {
//                     Init => (),
//                     Log { text } => println!("{}", text),
//                     AddTask { name } => tasks.push(Task { name, done: false }),
//                     MarkTask { index, done } => tasks[index].done = done,
//                     ClearDoneTasks => tasks.retain(|t| !t.done),
//                 }
// 
//                 tasks.len()
//             };
// 
//             webview.set_title(&format!("Rust Todo App ({} Tasks)", tasks_len))?;
//             render(webview)
//         })
//         .build()
//         .unwrap();
// 
//     webview.set_color((156, 39, 176));
// 
//     let res = webview.run().unwrap();
// 
//     println!("final state: {:?}", res);
// }
// 
// fn render(webview: &mut WebView<Vec<Task>>) -> WVResult {
//     let render_tasks = {
//         let tasks = webview.user_data();
//         println!("{:#?}", tasks);
//         format!("rpc.render({})", serde_json::to_string(tasks).unwrap())
//     };
//     webview.eval(&render_tasks)
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// struct Task {
//     name: String,
//     done: bool,
// }
// 
// #[derive(serde::Deserialize)]
// #[serde(tag = "cmd", rename_all = "camelCase")]
// pub enum Cmd {
//     Init,
//     Log { text: String },
//     AddTask { name: String },
//     MarkTask { index: usize, done: bool },
//     ClearDoneTasks,
// }
// 
// fn inline_style(s: &str) -> String {
//     format!(r#"<style type="text/css">{}</style>"#, s)
// }
// 
// fn inline_script(s: &str) -> String {
//     format!(r#"<script type="text/javascript">{}</script>"#, s)
// }

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

    const BOUND: u8 = 200;
    const NIP: Option<&str> = Some("README.md.html");
    //const NIP: Option<&str> = Some("README.md");

    // webview();
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..BOUND {
        print_entries::<{ BOUND }>(NIP);
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn print_entries<const BOUND: u8>(nip: Option<&str>) -> () {
    let mut count: u8 = 0;
    //let html = "**/*.html";
    //let css = "**/*css.html";
    let txt = "**/*.txt";
    //let glob = "**/*.md";
    let mut nip_vec = Vec::<String>::new(); // Create an empty Vec

    let doc = nip.unwrap().to_string();
    for entry in PROJECT_DIR.find(txt).unwrap() {
        count = count + 1;
        nip_vec.push((entry.path().display()).to_string());
        let mut md_content = PROJECT_DIR.get_file(entry.path()).unwrap();
        let content = md_content.contents_utf8().unwrap();
        match doc == format!("README.md") {
            true => println!("{}", content),
            _ => println!(""),
        }
        match doc == format!("BREAKING.md") {
            //true => println!("{}", markdown::to_html(content)),
            true => println!("{}", content),
            //_ => println!("NOT BREAKING.md.html!")
            _ => println!(""),
        }
        match doc == format!("{}", nip.unwrap().to_string().replace("", "")) {
            //true => println!("{}", markdown::to_html(content)),
            true => println!("{}", content),
            _ => println!("must be a nip!"),
        }
    } //end for entry loop
    std::process::exit(0);

    //README.md
    let (first, remainder) = nip_vec.split_at(1);
    let mut readme = PROJECT_DIR.get_file(first[0].clone()).unwrap();
    let readme_md = readme.contents_utf8().unwrap();
    println!("\n{}", markdown::to_html(readme_md));

    count = count - 1;

    //second
    //BREAKING.md
    let mut breaking = PROJECT_DIR.get_file(remainder[0].clone()).unwrap();
    let breaking_md = breaking.contents_utf8().unwrap();
    //println!("\n{}", breaking_md);
    println!("\n{}", markdown::to_html(breaking_md));

    //REMAINDERS
    //count variable based on new documents added to nips repo

    //traverse documents from last to no more remainders
    count = count - 1;
    //print!("count={}",count);

    //std::process::exit(0);

    count = count - 1;
    //println!("count={}", count);
    let (mut last, remainder) = remainder.split_at(1);
    let mut last = PROJECT_DIR.get_file(remainder[0].clone()).unwrap();
    let last_md = last.contents_utf8().unwrap();
    println!("\n{}", markdown::to_html(last_md));
    //println!("{}", markdown::to_html(remainder));

    count = count - 1;
    let (mut last, remainder) = remainder.split_at(1);
    let mut last = PROJECT_DIR.get_file(remainder[0].clone()).unwrap();
    let last_md = last.contents_utf8().unwrap();
    println!("\n{}", markdown::to_html(last_md));
    //println!("{}", markdown::to_html(remainder));

    count = count - 1;
    let (mut last, remainder) = remainder.split_at(1);
    let mut last = PROJECT_DIR.get_file(remainder[0].clone()).unwrap();
    let last_md = last.contents_utf8().unwrap();
    println!("\n{}", markdown::to_html(last_md));
    //println!("{}", markdown::to_html(remainder));
}
