use std::process::Command;
use std::env;
use rust_embed::Embed;
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, Registry};
use tracing::info;

#[derive(Embed)]
#[folder = "./scripts"]
struct Scripts;

fn main() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("")
        .try_init();

    let mut install_script: String;
    let mut content: String = String::from("");
    for filename in Scripts::iter() {
        tracing::info!("Embedded StaticFile: {}", filename.as_ref());
        println!("Embedded StaticFile: {}", filename.as_ref());

        // You can access the file content using StaticFiles::get(filename.as_ref())
        if let Some(file) = Scripts::get(filename.as_ref()) {
            let content = String::from_utf8_lossy(file.data.as_ref());
            // Do something with the content
            tracing::info!("Content: {}", content);
            println!("Content: {}", content);
        }
    }


    match Scripts::get("install_script.sh") {
        Some(file) => {
            content = String::from(std::str::from_utf8(file.data.as_ref()).unwrap());
            println!("Contents of install_script.sh:\n{}", content);
        }
        None => {
            eprintln!("Error: install_script.sh not found in embedded assets!");
        }
    }

    install_script = env::var("INSTALL_SCRIPT").unwrap_or(String::from(content));

	let mut default_config: String;
	let mut content: String = String::from("");

    match Scripts::get("default_config.conf") {
        Some(file) => {
            content = String::from(std::str::from_utf8(file.data.as_ref()).unwrap());
            println!("Contents of default_config.conf:\n{}", content);
        }
        None => {
            eprintln!("Error: default_config.conf not found in embedded assets!");
        }
    }

    default_config = env::var("DEFAULT_CONFIG").unwrap_or(String::from(content));

    let touch_makefile = Command::new("touch")
        .arg("Makefile")
        .status()
        .expect("Failed to execute install script");

    if touch_makefile.success() {
        println!("touch_makefile");
    } else {
        println!("Installation script failed.");
    }

    let clear_makefile = Command::new("echo")
        .arg(">")
        .arg("Makefile")
        .status()
        .expect("Failed to execute install script");

    if clear_makefile.success() {
        println!("clear_makefile");
    } else {
        println!("Installation script failed.");
    }
    let echo_default = Command::new("echo")
        .arg("default:")
        .arg(">")
        .arg("Makefile")
        .status()
        .expect("Failed to execute install script");

    if echo_default.success() {
        println!("echo_default");
    } else {
        println!("Installation script failed.");
    }
    let echo_default = Command::new("echo")
        .arg("	")
        .arg("@echo")
        .arg("default")
        .arg("command")
        .arg(">>")
        .arg("Makefile")
        .status()
        .expect("Failed to execute install script");

    if echo_default.success() {
        println!("Installation script executed successfully.");
    } else {
        println!("Installation script failed.");
    }


    //let install_default_conf = Command::new(install_script)
    //    .arg(default_config)
    //    .status()
    //    .expect("Failed to execute install script");

    //if install_default_conf.success() {
    //    println!("Installation script executed successfully.");
    //} else {
    //    println!("Installation script failed.");
    //}
}

