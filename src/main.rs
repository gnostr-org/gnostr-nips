//#![windows_subsystem = "windows"]

extern crate tinyfiledialogs as tfd;
extern crate nips;

use tfd::MessageBoxIcon;
use nips::*;

fn main() -> WVResult {
    let nips = nips::builder()
        .title("dialog.rs")
        .content(Content::Html(HTML))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                "open" => match tfd::open_file_dialog("Please choose a file...", "", None) {
                    Some(path) => tfd::message_box_ok("File chosen", &path, MessageBoxIcon::Info),
                    None => tfd::message_box_ok(
                        "Warning",
                        "You didn't choose a file.",
                        MessageBoxIcon::Warning,
                    ),
                },
                "save" => match tfd::save_file_dialog("Save file...", "") {
                    Some(path) => tfd::message_box_ok("File chosen", &path, MessageBoxIcon::Info),
                    None => tfd::message_box_ok(
                        "Warning",
                        "You didn't choose a file.",
                        MessageBoxIcon::Warning,
                    ),
                },
                "info" => {
                    tfd::message_box_ok("Info", "This is a info dialog", MessageBoxIcon::Info)
                }
                "warning" => tfd::message_box_ok(
                    "Warning",
                    "This is a warning dialog",
                    MessageBoxIcon::Warning,
                ),
                "error" => {
                    tfd::message_box_ok("Error", "This is a error dialog", MessageBoxIcon::Error)
                }
                "exit" => webview.exit(),
                _ => unimplemented!(),
            };
            Ok(())
        })
        .build()?;

    nips.run()
}

const HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>dialog.rs</title>
    <style>
li {
  float: left;
}
li a {
  display: block;
  padding: 8px;
  background-color: \#dddddd;
}
body {margin:0;}
ul {
  list-style-type: none;
  margin: 0;
  padding: 0;
  overflow: hidden;
  background-color: #333;
  position: fixed;
  top: 0;
  width: 100%;
}
li {
  float: left;
}
li a {
  display: block;
  color: white;
  text-align: center;
  padding: 14px 16px;
  text-decoration: none;
}
li a:hover:not(.active) {
  background-color: #111;
}
.active {
  background-color: #04AA6D;
}
button {
    background: none;
    color: white;
    border: none;
    padding: 20px;
    font: inherit;
    cursor: pointer;
    outline: inherit;
}
button:focus {
  outline: revert;
}
</style>
</head>
<body>
<script src="https://unpkg.com/@isomorphic-git/lightning-fs"></script>
<script src="https://unpkg.com/isomorphic-git@beta"></script>
<script type="module"></script>
<ul>
<li>
    <button onclick="external.invoke('open')">Open</button>
</li>
<li>
    <button onclick="external.invoke('save')">Save</button>
</li>
<li>
    <button onclick="external.invoke('info')">Info</button>
</li>
<li>
    <button onclick="external.invoke('warning')">Warning</button>
</li>
<li>
    <button onclick="external.invoke('warning')">Warning</button>
</li>
<li>
    <button onclick="external.invoke('exit')">Exit</button>
</li>
</ul>
</body>
</html>
"#;
