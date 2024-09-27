//#![windows_subsystem = "windows"]

extern crate tinyfiledialogs as tfd;
extern crate web_view;

use tfd::MessageBoxIcon;
use web_view::*;

fn main() -> WVResult {
    let webview = web_view::builder()
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

    webview.run()
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
<script type="module">
import http from 'https://unpkg.com/isomorphic-git@beta/http/web/index.js'
// Initialize isomorphic-git with a file system
window.fs = new LightningFS('fs')
// I prefer using the Promisified version honestly
window.pfs = window.fs.promises

window.dir = '/tutorial'
console.log(dir);
await pfs.mkdir(dir);
// Behold - it is empty!
await pfs.readdir(dir);

await git.clone({
  fs,
  http,
  dir,
  corsProxy: 'https://cors.isomorphic-git.org',
  url: 'https://github.com/isomorphic-git/isomorphic-git',
  ref: 'main',
  singleBranch: true,
  depth: 10
});

// Now it should not be empty...
await pfs.readdir(dir);

await git.log({fs, dir})
    await git.status({fs, dir, filepath: 'README.md'})
    await pfs.writeFile(`${dir}/README.md`, 'Very short README', 'utf8')
await git.status({fs, dir, filepath: 'README.md'})
await git.add({fs, dir, filepath: 'README.md'})
await git.status({fs, dir, filepath: 'README.md'})
await pfs.writeFile(`${dir}/newfile.txt`, 'Hello World', 'utf8')
await git.status({fs, dir, filepath: 'newfile.txt'})
await git.add({fs, dir, filepath: 'newfile.txt'})
await git.status({fs, dir, filepath: 'newfile.txt'})
let sha = await git.commit({
  fs,
  dir,
  message: 'Delete package.json and overwrite README.',
  author: {
    name: 'Mr. Test',
    email: 'mrtest@example.com'
  }
})

console.log(sha)
    let commits = await git.log({fs, dir, depth: 1})
console.log(commits[0])


var path = document.location.pathname;
var dir = path.substring(path.indexOf('DIRLISTTEST/', 1),
path.lastIndexOf('/'));
document.write(dir);


</script>


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

git.clone({ fs, http, dir, url: 'https://github.com/isomorphic-git/lightning-fs', corsProxy: 'https://cors.isomorphic-git.org' }).then(console.log)
await git.clone({
  fs,
  http,
  dir: '/tutorial',
  corsProxy: 'https://cors.isomorphic-git.org',
  url: 'https://github.com/isomorphic-git/isomorphic-git',
  singleBranch: true,
  depth: 1
})
console.log('done')

</body>
</html>
"#;
