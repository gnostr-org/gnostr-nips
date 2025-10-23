//! run this example with
//!   cargo run --example scrollable
//!
use clap::Parser;
use pulldown_cmark::Options;
use pulldown_cmark::{html, Parser as HTMLParser};
use rust_embed::Embed;

use sha2::Digest;
use sha2::Sha256;
use std::io;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};

use termimad::crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode::*, KeyEvent},
    queue,
    style::Color::*,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use termimad::*;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub mod path;

#[derive(Embed)]
#[folder = "."]
#[include = "*.md"]
#[exclude = "*.DS_Store"]
#[exclude = "target/*"]
#[exclude = "src"]
#[exclude = "src/*"]
#[exclude = ".git"]
#[exclude = ".git/*"]
#[exclude = ".github/*"]
#[exclude = ".gitignore"]
#[exclude = ".justfile"]
#[exclude = ".nojekyll"]
#[exclude = "build.rs"]
#[exclude = "dist-workspace.toml"]
#[exclude = "error.log"]
#[exclude = "output.log"]
#[exclude = "post-commit-history"]
#[exclude = "script.sh"]
#[exclude = "template/Makefile"]
#[exclude = "template/default_config.conf"]
#[exclude = "template/install_script.sh"]
#[exclude = "test_files/tabbed.txtbuild.rs"]
#[exclude = "Cargo.lock"]
#[exclude = "Cargo.toml"]
#[exclude = "LICENSE"]
#[exclude = "Makefile"]
pub struct Template;

/// a simple nostr-protocol/nips server
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Enable debug logging.
    #[clap(short, long)]
    pub debug: bool,

    /// List all embedded files.
    #[clap(short, long)]
    pub list_embedded: bool,

    /// Show the contents of an embedded file using a pager.
    //#[clap(short, long, value_name = "NIP", default_value = "README.md")]
    #[clap(short, long, value_name = "NIP")]
    pub show: Option<String>,

    /// Axum Serve.
    //#[clap(long, default_value = "false")]
    #[clap(long, default_value = "false")]
    pub serve: bool,

    /// Sets the port number to listen on
    #[arg(short, long, value_parser = clap::value_parser!(u16), default_value_t = 8080)]
    pub port: u16,

    /// Export all embedded files to the current directory.
    #[clap(short, long)]
    pub export: bool,

    /// Export all embedded files to the current directory.
    #[clap(long)]
    pub export_html: bool,

    /// Export all embedded files to the specified path.
    #[clap(long, value_name = "PATH")]
    pub export_path: Option<PathBuf>,

    #[arg(short, long, help = "Show USAGE.md")]
    pub usage: bool,
}

pub fn calculate_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub async fn extract(filename: &str, output_dir: &Path) -> io::Result<()> {
    match Template::get(filename) {
        Some(embedded_file) => {
            let output_path = output_dir.join(filename);
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent).await.expect("");
            }
            let mut outfile = File::create(&output_path).await?;
            outfile.write_all(embedded_file.data.as_ref()).await?;
            tracing::trace!(
                "Successfully exported '{}' to '{}'",
                filename,
                output_path.display()
            );
            Ok(())
        }
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Embedded file '{}' not found!", filename),
        )),
    }
}

pub async fn extract_html(filename: &str, output_dir: &Path) -> io::Result<()> {
    match Template::get(filename) {
        Some(embedded_file) => {
            let output_path = output_dir
                .join("docs")
                .join(remove_md_extension(filename).to_owned() + ".html");
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent).await.expect("");
            }
            let mut outfile = File::create(&output_path).await?;
            //            let embedded_file_data: &'static [u8] = embedded_file.data.as_ref();
            let embedded_file_data: Vec<u8> = embedded_file.data.as_ref().to_vec(); // Create an owned Vec

            //std::str::from_utf8(embedded_file_data)
            //outfile.write_all(markdown_to_html(&std::str::from_utf8(embedded_file_data).expect("")).as_bytes())?;
            outfile
                .write_all(
                    markdown_to_html(&std::str::from_utf8(&embedded_file_data).expect(""))
                        .as_bytes(),
                )
                .await?;

            //outfile.write_all(markdown_to_html(embedded_file_data[0..5]));
            tracing::trace!(
                "Successfully exported '{}' to '{}'",
                filename,
                output_path.display()
            );
            Ok(())
        }
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Embedded file '{}' not found!", filename),
        )),
    }
}

pub fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(120); // we don't want a too wide text column
    area
}

pub async fn run_app(skin: MadSkin, nip: String) -> Result<(), Error> {
    let mut w = stdout(); // we could also have used stderr
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?; // hiding the cursor
                      // get nip here
    let header = "# Nostr NIPs

";
    let content_with_header = format!("{}{}", header, nip);
    let mut view = MadView::from(content_with_header, view_area(), skin);
    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                Up => view.try_scroll_lines(-1),
                Down => view.try_scroll_lines(1),
                PageUp => view.try_scroll_pages(-1),
                PageDown => view.try_scroll_pages(1),
                _ => break,
            },
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }
    terminal::disable_raw_mode()?;
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}

pub fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(AnsiValue(178));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin.set_fg(AnsiValue(178)); // Set default foreground color for the header
    skin.bold.set_fg(Yellow); // Make header bold and yellow
    skin
}

pub fn markdown_to_html(markdown_input: &str) -> String {
    let mut options = Options::empty();
    //options.insert(Options::all());
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    //options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    options.insert(Options::ENABLE_OLD_FOOTNOTES);
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_DEFINITION_LIST);
    options.insert(Options::ENABLE_SUPERSCRIPT);
    options.insert(Options::ENABLE_SUBSCRIPT);
    options.insert(Options::ENABLE_WIKILINKS);

    let parser = HTMLParser::new_ext(markdown_input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

//pub fn scrollable() -> Result<(), Error> {
//    let skin = make_skin();
//    run_app(skin, /* std::string::String */)
//}

pub fn remove_md_extension(filename: &str) -> &str {
    filename.strip_suffix(".md").unwrap_or(filename)
}

static MD: &str = r#"# Scrollable Markdown in Termimad

Use the **↓** and **↑** arrow keys to scroll this page.
Use any other key to quit the application.

*Now I'll describe this example with more words than necessary, in order to be sure to demonstrate scrolling (and **wrapping**, too, thanks to long sentences).*

## What's shown

* an **area** fitting the screen (with a max width of 120, to be prettier)
* a markdown text
 * **parsed**,
 * **skinned**,
 * and **wrapped** to fit the width
* a **scrollable** view in *raw terminal mode*

## Area

The area specifies the part of the screen where we'll display our markdown.

    let mut area = Area::full_screen();
    area.pad_for_max_width(120); // we don't want a too wide text column

*(yes the code block centering in this example is a little too much, it's just here to show what's possible)*

## Parsed Markdown

The text is parsed from a string. In this example we directly wrap it for the width of the area:

    let text = skin.area_wrapped_text(markdown, &area);

If we wanted to modify the parsed representation, or modify the area width, we could also have kept the parsed text (*but parsing is cheap*).

## The TextView

It's just a text put in an area, tracking your **scroll** position (and whether you want the scrollbar to be displayed).

    let mut text_view = TextView::from(&area, &text);

## Really Scrolling

Not two applications handle events in the same way. **Termimad** doesn't try to handle this but lets you write it yourself, which is fairly easily done with **Crossterm** for example:

```
let mut events = TerminalInput::new().read_sync();
loop {
    text_view.write()?;
    if let Some(Keyboard(key)) = events.next() {
        match key {
            Up => text_view.try_scroll_lines(-1),
            Down => text_view.try_scroll_lines(1),
            PageUp => text_view.try_scroll_pages(-1),
            PageDown => text_view.try_scroll_pages(1),
            _ => break,
        }
    }
}
```

## Skin

We want *shiny **colors*** (and unreasonnable centering):

    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fgbg(Magenta, rgb(30, 30, 40));
    skin.scrollbar.track.set_fg(Rgb{r:30, g:30, b:40});
    skin.scrollbar.thumb.set_fg(Rgb{r:67, g:51, b:0});
    skin.code_block.align = Alignment::Center;

The scrollbar's colors were also adjusted to be consistent.

## Usage

* **↓** and **↑** arrow keys : scroll this page
* any other key : quit

## And let's just finish by a table

It's a little out of context but it shows how a wide table can be wrapped in a thin terminal.

|feature|supported|details|
|-|:-:|-
| tables | yes | pipe based only, alignement not yet supported
| italic, bold | yes | star based only|
| inline code | yes |
| code bloc | yes |with tabs. Fences not supported
| crossed text |  ~~not yet~~ | wait... now it works!
| phpbb like links | no | (because it's preferable to show an URL in a terminal)

(resize your terminal if it's too wide for wrapping to occur)

"#;
