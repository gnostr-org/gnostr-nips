use axum::response::Redirect;
use nips::extract;
use nips::extract_html;
use nips::markdown_to_html;
use nips::path::canonicalize_path;
use nips::run_app;
use nips::*;
use nips::{Args, Template};
//use tower_http::services::Redirect;
use axum::{
    extract::Request, /*handler::HandlerWithoutStateExt, http::StatusCode, */ routing::get,
    Router,
};
use clap::Parser;
use pulldown_cmark::Options;
use pulldown_cmark::{html, Parser as HTMLParser};
use std::env;
//use rust_embed::RustEmbed;
use sha2::{Digest, Sha256};
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
use tokio::fs;
use tower::ServiceExt;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};
//fn _make_executable(script_path: &Path) -> io::Result<()> {
//    let mut permissions = fs::metadata(script_path)?.permissions();
//    permissions.set_mode(permissions.mode() | 0o111);
//    fs::set_permissions(script_path, permissions)?;
//    tracing::debug!("Made '{}' executable.", script_path.display());
//    Ok(())
//}
//
//fn _execute_script(script_path: &Path) -> io::Result<()> {
//    tracing::debug!("Executing script: {}", script_path.display());
//    let log_file = File::create("output.log")?;
//    let error_file = File::create("error.log")?;
//    let mut command = Command::new(script_path);
//    command
//        .stdout(Stdio::from(log_file))
//        .stderr(Stdio::from(error_file));
//    let status = command.spawn()?.wait()?;
//    if status.success() {
//        tracing::debug!("Script '{}' executed successfully.", script_path.display());
//        Ok(())
//    } else {
//        eprintln!(
//            "Script '{}' failed with exit code: {:?}",
//            script_path.display(),
//            status.code()
//        );
//        Err(io::Error::new(
//            io::ErrorKind::Other,
//            format!(
//                "Script execution failed with exit code: {:?}",
//                status.code()
//            ),
//        ))
//    }
//}

//async fn print_list() -> Result<(), Box<dyn std::error::Error>> {
async fn print_list() -> i32 {
    let args = Args::parse();
    tracing::trace!("Embedded files:");
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
    0 as i32
}
async fn usage() -> i32 {
    match Template::get(&"USAGE.md") {
        Some(embedded_file) => {
            let content = String::from_utf8_lossy(embedded_file.data.as_ref());
            let res = markdown_to_html(&content);
            tracing::debug!("{}", res);
            //print!("{}", res);
            //std::process::exit(0);
            //#[allow(unreachable_code)]
            let skin = make_skin();
            let _res = run_app(skin, (&content).to_string()).await;
        }
        None => {
            tracing::trace!("Error: {}' not found!", &"USAGE.md");
            print_list().await;
        }
    }
    0 as i32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args::parse();

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

    tracing::trace!("Parsed arguments: {:?}", args);

    if args.usage {
        std::process::exit(usage().await)
    } else {
        0
    };
    //both cases we assume the server may already
    //be running
    //nips -s --serve
    if args.show.is_none() && args.serve.clone() {
        args.serve = false;
        //std::process::exit(print_list().await);
        std::process::exit(usage().await);
    } else if args.show.is_none() {
        args.serve = false;
    }

    if args.list_embedded && !args.serve {
        let _ = print_list().await;
    }

    if args.export && !args.serve {
        tracing::trace!("Exporting all embedded files to the current directory...");
        let current_dir = env::current_dir()?;
        let mut export_count = 0;
        for file in Template::iter() {
            match extract(file.as_ref(), &current_dir).await {
                Ok(_) => {
                    export_count += 1;
                }
                Err(e) => {
                    eprintln!("Error exporting '{}': {}", file.as_ref(), e);
                }
            }
        }
        tracing::trace!("Successfully exported {} embedded files.", export_count);
        if !args.serve && args.export {
            return Ok(());
        }
    }
    //if args.export_html && !args.serve {
    tracing::trace!("Exporting all embedded files to the current directory...");
    let current_dir = env::current_dir()?;
    let mut export_count = 0;
    for file in Template::iter() {
        match extract_html(file.as_ref(), &current_dir).await {
            Ok(_) => {
                export_count += 1;
            }
            Err(e) => {
                eprintln!("Error exporting '{}': {}", file.as_ref(), e);
            }
        }
    }
    tracing::trace!("Successfully exported {} embedded files.", export_count);
    if !args.serve && args.export_html {
        return Ok(());
    }
    //}
    if let Some(export_path) = &args.export_path {
        tracing::trace!(
            "Exporting all embedded files to '{}'...",
            export_path.display()
        );
        let mut export_count = 0;
        for file in Template::iter() {
            match extract(file.as_ref(), export_path).await {
                Ok(_) => {
                    export_count += 1;
                }
                Err(e) => {
                    eprintln!("Error exporting '{}': {}", file.as_ref(), e);
                }
            }
        }
        tracing::trace!(
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
                //print!("{}", res);
                //std::process::exit(0);
                //#[allow(unreachable_code)]
                let skin = make_skin();
                let _res = run_app(skin, (&content).to_string()).await;
                if !args.serve && args.show.is_some() {
                    return Ok(());
                }
            }
            None => {
                tracing::trace!("Error: Embedded NIP file '{}' not found!", filename);
                print_list().await;
            }
        }
    }

    tracing::trace!(
        "Canonical path of '.': {}",
        canonicalize_path(Path::new(".")).await?.display()
    );
    tracing::trace!(
        "Canonical path of 'docs': {}",
        canonicalize_path(Path::new("docs")).await?.display()
    );

    #[cfg(windows)]
    let absolute_path_str = "C:\\Windows\\System32\\cmd.exe";
    #[cfg(not(windows))]
    let absolute_path_str = "/bin/ls";

    tracing::trace!(
        "Canonical path of '{}': {}",
        absolute_path_str,
        canonicalize_path(Path::new(absolute_path_str))
            .await?
            .display()
    );

    if args.serve {
        //MUST be true
        tokio::join!(run_app(make_skin(), String::from("01")));
        tokio::join!(serve(using_serve_dir_with_assets_fallback(), args.port),);
    }

    Ok(())
}

// //multi server impl
fn using_serve_dir_with_assets_fallback() -> Router {
    let serve_dir = ServeDir::new("docs").not_found_service(ServeFile::new("docs/readme.html"));
    let mut router = Router::new();

    for file in Template::iter() {
        let filename = file.as_ref();
        if filename.ends_with(".md") {
            let route_path = format!("/{}.md", remove_md_extension(filename));
            let redirect_path = format!("/{}.html", remove_md_extension(filename));
            tracing::debug!(
                "Route added for {} (redirecting to {})",
                route_path,
                &redirect_path.clone()
            );
            router = router.route(
                &route_path,
                get(move || async move { Redirect::permanent(&redirect_path) }),
            );
            //we handle if localhost:port/01 requested for example
            //curl http://localhost/01
            let route_path = format!("/{}", remove_md_extension(filename));
            let redirect_path = format!("/{}.html", remove_md_extension(filename)); // Construct the redirect path
            tracing::debug!(
                "Route added for {} (redirecting to {})",
                route_path,
                &redirect_path.clone()
            );
            router = router.route(
                &route_path,
                get(move || async move { Redirect::permanent(&redirect_path) }),
            );
        } //
    }

    router
        .nest_service("/docs", serve_dir.clone())
        .fallback_service(serve_dir)
}

#[allow(clippy::let_and_return)]
fn _calling_serve_dir_from_a_handler() -> Router {
    tracing::debug!("/foo:port");
    tracing::debug!("/docs:port");
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

fn _using_serve_file_from_a_route() -> Router {
    tracing::debug!("/index:port");
    Router::new().route_service("/index", ServeFile::new("docs/index.html"))
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("{}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    //use rust_embed::RustEmbed;
    use rust_embed::Embed;
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
        use rust_embed::Embed;
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
        use rust_embed::Embed;
        test_make_test_file()?;
        let test_file_content = "Line with\ttab.\nAnother\t\ttabbed line.\n\tLeading tab.\n\\tThis line starts with a literal backslash-t.\nThis line has a tab in the middle:	like this.";

        #[derive(Embed)]
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

    //#[test]
    //fn remove_dir_all_custom() -> io::Result<()> {
    //    use rust_embed::Embed;
    //    let path = Path::new("test_files");
    //    if path.is_dir() {
    //        fs::remove_dir_all(path)?;
    //        fs::create_dir_all(path)?;
    //    } else if path.is_file() {
    //        fs::remove_file(path)?;
    //        fs::create_dir_all(path)?;
    //    } else {
    //        fs::create_dir_all(path)?;
    //    }
    //    test_make_test_file()?;
    //    Ok(())
    //}
}
