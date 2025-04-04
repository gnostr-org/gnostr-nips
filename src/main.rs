use std::process::Command;
use std::borrow::Cow;
use std::env;
use rust_embed::Embed;
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, Registry};
use tracing::{info, debug};

#[derive(Embed)]
#[folder = "./scripts"]
struct Scripts;

fn main() {
    let subscriber = Registry::default()
        .with(fmt::layer().with_writer(std::io::stdout)) // Configure the fmt layer
        .with(EnvFilter::from_default_env());        // Add the EnvFilter layer

    // Set the global default subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    let install_script = Scripts::get("install_script.sh").unwrap();
    let mut content: Cow<str>;// = std::str::from_utf8(install_script.data.as_ref()).unwrap().into();
    for filename in Scripts::iter() {
        tracing::debug!("Embedded StaticFile:\n{}", filename.as_ref());
        //println!("Embedded StaticFile:\n{}", filename.as_ref());
        if let Some(file) = Scripts::get(filename.as_ref()) {
            content = String::from_utf8_lossy(file.data.as_ref());
            tracing::debug!("Content:\n{}", content);
            //println!("Content:\n{}", content);
        }
    }


    match Scripts::get("install_script.sh") {
        Some(file) => {
            content = String::from_utf8_lossy(file.data.as_ref());
            tracing::debug!("Contents of install_script.sh:\n{}", content);
            //println!("Contents of install_script.sh:\n{}", content);
        }
        None => {
            eprintln!("Error: install_script.sh not found in embedded assets!");
        }
    }

    //let install_script = env::var("INSTALL_SCRIPT").unwrap_or(String::from(content));

	//let mut default_config: String;
	//let mut content: String = String::from("");

    //match Scripts::get("default_config.conf") {
    //    Some(file) => {
    //        content = String::from(std::str::from_utf8(file.data.as_ref()).unwrap());
    //        tracing::debug!("Contents of default_config.conf:\n{}", content);
    //        //println!("Contents of default_config.conf:\n{}", content);
    //    }
    //    None => {
    //        eprintln!("Error: default_config.conf not found in embedded assets!");
    //    }
    //}

    //default_config = env::var("DEFAULT_CONFIG").unwrap_or(String::from(content));

    //let touch_makefile = Command::new("touch")
    //    .arg("Makefile")
    //    .status()
    //    .expect("Failed to execute install script");

    //if touch_makefile.success() {
    //    tracing::debug!("touch_makefile");
    //    //println!("touch_makefile");
    //} else {
    //    tracing::debug!("Installation script failed.");
    //    //println!("Installation script failed.");
    //}

    //let clear_makefile = Command::new("echo")
    //    .arg(">")
    //    .arg("Makefile")
    //    .arg(">/dev/null")
    //    .status()
    //    .expect("Failed to execute install script");

    //if clear_makefile.success() {
    //    tracing::debug!("clear_makefile");
    //    //println!("clear_makefile");
    //} else {
    //    tracing::debug!("Installation script failed.");
    //    //println!("Installation script failed.");
    //}
    //let echo_default = Command::new("echo")
    //    .arg("default:")
    //    .arg(">")
    //    .arg("Makefile")
    //    .status()
    //    .expect("Failed to execute install script");

    //if echo_default.success() {
    //    tracing::debug!("echo_default");
    //    //println!("echo_default");
    //} else {
    //    tracing::debug!("Installation script failed.");
    //    //println!("Installation script failed.");
    //}
    //let echo_default = Command::new("echo")
    //    .arg("	")
    //    .arg("@echo")
    //    .arg("default")
    //    .arg("command")
    //    .arg(">>")
    //    .arg("Makefile")
    //    .status()
    //    .expect("Failed to execute install script");

    //if echo_default.success() {
    //    tracing::debug!("Installation script executed successfully.");
    //    //println!("Installation script executed successfully.");
    //} else {
    //    tracing::debug!("Installation script failed.");
    //    //println!("Installation script failed.");
    //}


    //let install_default_conf = Command::new(install_script)
    //    //.arg(default_config)
    //    .status()
    //    .expect("Failed to execute install script");

    //if install_default_conf.success() {
    //    println!("Installation script executed successfully.");
    //} else {
    //    println!("Installation script failed.");
    //}
}

