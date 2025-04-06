use axum::response::Redirect;

//use tower_http::services::Redirect;

use axum::{
    extract::Request, handler::HandlerWithoutStateExt, http::StatusCode, routing::get, Router,
};
use clap::Parser;
use pulldown_cmark::Options;
use pulldown_cmark::{html, Parser as HTMLParser};
use rust_embed::Embed;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{stdout, Write};
use std::net::SocketAddr;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Stdio;
use termimad::crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode::*, KeyEvent},
    queue,
    style::Color::*,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use termimad::*;
use tower::ServiceExt;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

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
struct Template;

/// A simple tool to view embedded Markdown files.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Enable debug logging.
    #[clap(short, long)]
    debug: bool,

    /// List all embedded files.
    #[clap(short, long)]
    list_embedded: bool,

    /// Show the contents of an embedded file using a pager.
    //#[clap(short, long, value_name = "NIP", default_value = "README.md")]
    #[clap(short, long, value_name = "NIP")]
    show: Option<String>,

    /// Axum Serve.
    //#[clap(long, default_value = "false")]
    #[clap(long, default_value = "false")]
    serve: bool,

    /// Export all embedded files to the current directory.
    #[clap(short, long)]
    export: bool,

    /// Export all embedded files to the current directory.
    #[clap(long)]
    export_html: bool,

    /// Export all embedded files to the specified path.
    #[clap(long, value_name = "PATH")]
    export_path: Option<PathBuf>,
}

fn _make_executable(script_path: &Path) -> io::Result<()> {
    let mut permissions = fs::metadata(script_path)?.permissions();
    permissions.set_mode(permissions.mode() | 0o111);
    fs::set_permissions(script_path, permissions)?;
    tracing::debug!("Made '{}' executable.", script_path.display());
    Ok(())
}

fn _execute_script(script_path: &Path) -> io::Result<()> {
    tracing::debug!("Executing script: {}", script_path.display());
    let log_file = File::create("output.log")?;
    let error_file = File::create("error.log")?;
    let mut command = Command::new(script_path);
    command
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(error_file));
    let status = command.spawn()?.wait()?;
    if status.success() {
        tracing::debug!("Script '{}' executed successfully.", script_path.display());
        Ok(())
    } else {
        eprintln!(
            "Script '{}' failed with exit code: {:?}",
            script_path.display(),
            status.code()
        );
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Script execution failed with exit code: {:?}",
                status.code()
            ),
        ))
    }
}

fn canonicalize_path(path: &Path) -> io::Result<PathBuf> {
    let absolute_path = if path.is_relative() {
        let current_dir = env::current_dir()?;
        current_dir.join(path)
    } else {
        path.to_path_buf()
    };
    fs::canonicalize(absolute_path)
}

fn extract(filename: &str, output_dir: &Path) -> io::Result<()> {
    match Template::get(filename) {
        Some(embedded_file) => {
            let output_path = output_dir.join(filename);
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&output_path)?;
            outfile.write_all(embedded_file.data.as_ref())?;
            tracing::debug!(
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

fn remove_md_extension(filename: &str) -> &str {
    filename.strip_suffix(".md").unwrap_or(filename)
}

fn extract_html(filename: &str, output_dir: &Path) -> io::Result<()> {
    match Template::get(filename) {
        Some(embedded_file) => {
            let output_path = output_dir
                .join("docs")
                .join(remove_md_extension(filename).to_owned() + ".html");
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&output_path)?;
            //            let embedded_file_data: &'static [u8] = embedded_file.data.as_ref();
            let embedded_file_data: Vec<u8> = embedded_file.data.as_ref().to_vec(); // Create an owned Vec

            //std::str::from_utf8(embedded_file_data)
            //outfile.write_all(markdown_to_html(&std::str::from_utf8(embedded_file_data).expect("")).as_bytes())?;
            outfile.write_all(
                markdown_to_html(&std::str::from_utf8(&embedded_file_data).expect("")).as_bytes(),
            )?;

            //outfile.write_all(markdown_to_html(embedded_file_data[0..5]));
            tracing::debug!(
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

fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(120);
    area
}

#[allow(unused_variables)]
fn run_app(skin: MadSkin, nip: String) -> Result<(), Error> {
    let res = markdown_to_html(&nip);
    tracing::debug!("{}", res);
    print!("{}", res);
    //std::process::exit(0);
    //#[allow(unreachable_code)]
    let mut w = stdout();
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?;
    let mut view = MadView::from(nip.to_owned(), view_area(), skin);
    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                Up => view.try_scroll_lines(-1),
                Down => view.try_scroll_lines(1),
                PageUp => view.try_scroll_pages(-1),
                PageDown => view.try_scroll_pages(1),
                Char('q') | Esc => break,
                _ => {}
            },
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }
    terminal::disable_raw_mode()?;
    queue!(w, Show)?;
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(AnsiValue(178));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin
}

fn markdown_to_html(markdown_input: &str) -> String {
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

fn calculate_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let level_filter = if args.debug {
        EnvFilter::new("debug")
    } else {
        EnvFilter::from_default_env()
    };

    let subscriber = Registry::default()
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(level_filter);
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    tracing::debug!("Parsed arguments: {:?}", args);

    if args.list_embedded && !args.serve {
        tracing::debug!("Embedded files:");
        for file in Template::iter() {
            let _level_filter = if args.debug {
                let this_file = Template::get(&file).unwrap();
                //match output of shasum -a 256
                //$ shasum -a 256 README.md
                //b238c63f0e937a2b3a3982ecd8328ee03be26584d10723575802e9c6f098f361  README.md
                println!(
                    "{}  {}",
                    calculate_sha256(this_file.data.as_ref()),
                    file.as_ref()
                );
            } else {
                println!("{}", file.as_ref());
            };
        }
        if !args.serve && args.list_embedded {
            return Ok(());
        }
    }

    if args.export && !args.serve {
        tracing::info!("Exporting all embedded files to the current directory...");
        let current_dir = env::current_dir()?;
        let mut export_count = 0;
        for file in Template::iter() {
            match extract(file.as_ref(), &current_dir) {
                Ok(_) => {
                    export_count += 1;
                }
                Err(e) => {
                    eprintln!("Error exporting '{}': {}", file.as_ref(), e);
                }
            }
        }
        tracing::info!("Successfully exported {} embedded files.", export_count);
        if !args.serve && args.export {
            return Ok(());
        }
    }
    //if args.export_html && !args.serve {
    tracing::info!("Exporting all embedded files to the current directory...");
    let current_dir = env::current_dir()?;
    let mut export_count = 0;
    for file in Template::iter() {
        match extract_html(file.as_ref(), &current_dir) {
            Ok(_) => {
                export_count += 1;
            }
            Err(e) => {
                eprintln!("Error exporting '{}': {}", file.as_ref(), e);
            }
        }
    }
    tracing::info!("Successfully exported {} embedded files.", export_count);
    if !args.serve && args.export_html {
        return Ok(());
    }
    //}
    if let Some(export_path) = &args.export_path {
        tracing::info!(
            "Exporting all embedded files to '{}'...",
            export_path.display()
        );
        let mut export_count = 0;
        for file in Template::iter() {
            match extract(file.as_ref(), export_path) {
                Ok(_) => {
                    export_count += 1;
                }
                Err(e) => {
                    eprintln!("Error exporting '{}': {}", file.as_ref(), e);
                }
            }
        }
        tracing::info!(
            "Successfully exported {} embedded files to '{}'.",
            export_count,
            export_path.display()
        );
        if !args.serve && args.export_path.is_some() {
            return Ok(());
        }
    }

    if let Some(nip_arg) = &args.show {
        let filename = if nip_arg.ends_with(".md") {
            nip_arg.clone()
        } else {
            format!("{:02}.md", nip_arg)
        };

        match Template::get(&filename) {
            Some(embedded_file) => {
                let content = String::from_utf8_lossy(embedded_file.data.as_ref());
                let res = markdown_to_html(&content);
                tracing::debug!("{}", res);
                print!("{}", res);
                std::process::exit(0);
                #[allow(unreachable_code)]
                let skin = make_skin();
                let _res = run_app(skin, (&content).to_string());
                if !args.serve && args.show.is_some() {
                    return Ok(());
                }
            }
            None => {
                eprintln!("Error: Embedded NIP file '{}' not found!", filename);
                std::process::exit(1);
            }
        }
    }

    tracing::debug!(
        "Canonical path of '.': {}",
        canonicalize_path(Path::new("."))?.display()
    );
    tracing::debug!(
        "Canonical path of 'docs': {}",
        canonicalize_path(Path::new("docs"))?.display()
    );

    #[cfg(windows)]
    let absolute_path_str = "C:\\Windows\\System32\\cmd.exe";
    #[cfg(not(windows))]
    let absolute_path_str = "/bin/ls";

    tracing::debug!(
        "Canonical path of '{}': {}",
        absolute_path_str,
        canonicalize_path(Path::new(absolute_path_str))?.display()
    );

    if args.serve {
        //MUST be true
        tokio::join!(
            // serve(using_serve_dir(), 3000),
            // serve(using_serve_dir(), 3001),
            serve(using_serve_dir_with_assets_fallback(), 3000),
            serve(using_serve_dir_only_from_root_via_fallback(), 3003),
            serve(using_serve_dir_with_handler_as_service(), 3004),
            serve(two_serve_dirs(), 3005),
            serve(calling_serve_dir_from_a_handler(), 3006),
            serve(using_serve_file_from_a_route(), 3307),
        );
    }

    Ok(())
}

// //multi server impl
// fn using_serve_dir() -> Router {
//     tracing::debug!("/docs:3000");
//     // serve the file in the "docs" directory under `/docs`
//     Router::new().nest_service("/docs", ServeDir::new("docs"))
// }

fn using_serve_dir_with_assets_fallback() -> Router {
    let serve_dir = ServeDir::new("docs").not_found_service(ServeFile::new("docs/readme.html"));
    let mut router = Router::new();

    for file in Template::iter() {
        let filename = file.as_ref();
        if filename.ends_with(".md") {
            let route_path = format!("/{}.md", remove_md_extension(filename));
            let redirect_path = format!("/{}.html", remove_md_extension(filename)); // Construct the redirect path
            tracing::debug!(
                "Route added for {} (redirecting to {})",
                route_path,
                &redirect_path.clone()
            );
            router = router.route(
                &route_path,
                get(move || async move {
                    //Redirect::permanent(&redirect_path.clone()) // Perform the redirect
                    Redirect::permanent(&redirect_path) // Perform the redirect
                }),
            );
        }
    }

    router
        .nest_service("/docs", serve_dir.clone())
        .fallback_service(serve_dir)
}

fn using_serve_dir_only_from_root_via_fallback() -> Router {
    // you can also serve the assets directly from the root (not nested under `/docs`)
    // by only setting a `ServeDir` as the fallback
    tracing::debug!("/docs/index.html:3003");
    let serve_dir = ServeDir::new("docs").not_found_service(ServeFile::new("docs/index.html"));

    Router::new()
        .route("/foo", get(|| async { "Hi from /foo" }))
        .fallback_service(serve_dir)
}

fn using_serve_dir_with_handler_as_service() -> Router {
    tracing::debug!("/docs/index.html:3004");
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found")
    }

    // you can convert handler function to service
    let service = handle_404.into_service();

    let serve_dir = ServeDir::new("assets").not_found_service(service);

    Router::new()
        .route("/docs", get(|| async { "Hi from /docs" }))
        .fallback_service(serve_dir)
}

fn two_serve_dirs() -> Router {
    tracing::debug!("/assets/index.html:3005");
    // you can also have two `ServeDir`s nested at different paths
    let serve_dir_from_docs = ServeDir::new("docs");
    let serve_dir_from_dist = ServeDir::new("dist");

    Router::new()
        .nest_service("/docs", serve_dir_from_docs)
        .nest_service("/dist", serve_dir_from_dist)
}

#[allow(clippy::let_and_return)]
fn calling_serve_dir_from_a_handler() -> Router {
    tracing::debug!("/foo:3006");
    tracing::debug!("/docs:3006");
    // via `tower::Service::call`, or more conveniently `tower::ServiceExt::oneshot` you can
    // call `ServeDir` yourself from a handler
    Router::new().nest_service(
        "/foo",
        get(|request: Request| async {
            let service = ServeDir::new("docs");
            let result = service.oneshot(request).await;
            result
        }),
    )
}

fn using_serve_file_from_a_route() -> Router {
    tracing::debug!("/index:3307");
    Router::new().route_service("/index", ServeFile::new("assets/index.html"))
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on\n{}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_embed::RustEmbed;
    use std::fs;
    use std::path::PathBuf;
    use std::thread;
    use std::time::Duration;

    fn create_test_file(path: &PathBuf, content: &str) -> io::Result<()> {
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, content)?;
        Ok(())
    }

    #[test]
    fn test_make_test_file() -> io::Result<()> {
        let test_file_content = "Line with\ttab.\nAnother\t\ttabbed line.\n\tLeading tab.\n\\tThis line starts with a literal backslash-t.\nThis line has a tab in the middle:	like this.";
        let script_path = Path::new("tabbed.txt");
        let test_files = Path::new("test_files");
        let full_path = test_files.join(script_path);
        tracing::debug!("{}", full_path.display());
        create_test_file(&full_path, test_file_content)?;
        thread::sleep(Duration::from_secs(1));
        Ok(())
    }

    #[test]
    fn test_preserve_tabs() -> io::Result<()> {
        test_make_test_file()?;
        let test_file_content = "Line with\ttab.\nAnother\t\ttabbed line.\n\tLeading tab.\n\\tThis line starts with a literal backslash-t.\nThis line has a tab in the middle:	like this.";

        #[derive(RustEmbed)]
        #[folder = "./test_files"]
        #[exclude = "*.DS_Store"]
        struct EmbeddedAssets;

        for file in EmbeddedAssets::iter() {
            println!("Found asset: {}", file.as_ref());

            match EmbeddedAssets::get(file.as_ref()) {
                Some(embedded_file) => {
                    let content = String::from_utf8_lossy(embedded_file.data.as_ref());
                    println!("Contents of {}:\n{}", file.as_ref(), content);
                    let install_default_conf = Command::new("echo")
                        .arg(content.as_ref())
                        .status()
                        .expect("Failed to execute install script");
                    if install_default_conf.success() {
                        println!("Installation script executed successfully.");
                    } else {
                        eprintln!("Installation script failed.");
                    }
                }
                None => {
                    eprintln!("Error: {} not found in embedded assets!", file.as_ref());
                }
            }
        }

        if let Some(file) = EmbeddedAssets::get("tabbed.txt") {
            let embedded_content = String::from_utf8_lossy(file.data.as_ref()).to_string();
            assert_eq!(embedded_content, test_file_content);
            assert!(embedded_content.contains("\t"));
            assert!(embedded_content.contains("	"));
            assert!(embedded_content.lines().nth(0).unwrap().contains('\t'));
            assert!(embedded_content.lines().nth(0).unwrap().contains("	"));
            assert!(embedded_content.lines().nth(1).unwrap().contains('\t'));
            assert!(embedded_content.lines().nth(1).unwrap().contains("	"));
            assert!(embedded_content.lines().nth(1).unwrap().contains("\t\t"));
            assert!(embedded_content.lines().nth(1).unwrap().contains("		"));
            assert!(embedded_content.lines().nth(2).unwrap().contains('\t'));
            assert!(embedded_content.lines().nth(2).unwrap().contains("	"));
            assert!(!embedded_content.lines().nth(3).unwrap().starts_with('\t'));
            assert!(!embedded_content.lines().nth(3).unwrap().starts_with("\t"));
            assert!(!embedded_content.lines().nth(3).unwrap().starts_with("	"));
            assert!(embedded_content.lines().nth(0).unwrap().contains("h\t"));
            assert!(embedded_content.lines().nth(1).unwrap().contains("r\t"));
            assert!(embedded_content.lines().nth(2).unwrap().contains("\tL"));
        } else {
            println!("Failed to embed 'tabbed.txt'");
        }
        Ok(())
    }

    #[test]
    fn remove_dir_all_custom() -> io::Result<()> {
        let path = Path::new("test_files");
        if path.is_dir() {
            fs::remove_dir_all(path)?;
            fs::create_dir_all(path)?;
        } else if path.is_file() {
            fs::remove_file(path)?;
            fs::create_dir_all(path)?;
        } else {
            fs::create_dir_all(path)?;
        }
        test_make_test_file()?;
        Ok(())
    }
}
