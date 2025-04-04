use std::process::Command;
use std::borrow::Cow;
use std::env;
use rust_embed::Embed;
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, Registry};
use tracing::{info, debug};
use std::fs::File;
use std::io::Write;
use std::path::Path;



#[derive(Embed)]
#[folder = "./scripts"]
struct Scripts;

//fn main() -> Result<(), std::io::Error> {
fn main() {

    let subscriber = Registry::default()
        .with(fmt::layer().with_writer(std::io::stdout)) // Configure the fmt layer
        .with(EnvFilter::from_default_env());        // Add the EnvFilter layer

    // Set the global default subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");




    let filename_to_extract = "Makefile"; // Replace with the name of your embedded file
    let output_path = Path::new("extracted_files").join(filename_to_extract);

    // Ensure the output directory exists
    std::fs::create_dir_all(output_path.parent().unwrap());

    match Scripts::get(filename_to_extract) {
        Some(embedded_file) => {
            let mut outfile = File::create(&output_path);
            outfile.expect("").write_all(embedded_file.data.as_ref()).expect("");
            println!("Successfully extracted '{}' to '{}'", filename_to_extract, output_path.display());
        }
        None => {
            eprintln!("Error: Embedded file '{}' not found!", filename_to_extract);
        }
    }



    let install_script = Scripts::get("install_script.sh").unwrap();
    let mut content: Cow<str>;// = std::str::from_utf8(install_script.data.as_ref()).unwrap().into();
    for filename in Scripts::iter() {
        //tracing::debug!("Embedded StaticFile:\n{}", filename.as_ref());
        //println!("Embedded StaticFile:\n{}", filename.as_ref());
        if let Some(file) = Scripts::get(filename.as_ref()) {

            let mut outfile = File::create(&filename.as_ref());
            content = String::from_utf8_lossy(file.data.as_ref());
    //        outfile?.write_all(content).expect("");


            content = String::from_utf8_lossy(file.data.as_ref());
            //tracing::debug!("Content:\n{}", content);
            //println!("Content:\n{}", content);
        }
    }


    let touch_makefile = Command::new("touch")
        .arg("Makefile")
        .status()
        .expect("Failed to execute install script");



    //match Scripts::get("Makefile") {
    //    Some(file) => {
    //        content = String::from_utf8_lossy(file.data.as_ref());
    //        tracing::debug!("Contents of Makefile:\n{}", content);
    //        //println!("Contents of Makefile:\n{}", content);
    //        let install_default_conf = Command::new("echo")
    //            //.arg(format!("\"{}\"", content.as_ref()))
    //            .arg(format!("{}", content.as_ref()))
    //            .arg("|")
    //            .arg("Makefile")
    //            .status()
    //            .expect("Failed to make Makefile");
    //        if install_default_conf.success() {
    //            println!("Installation script executed successfully.");
    //        } else {
    //            println!("Installation script failed.");
    //        }
    //    }
    //    None => {
    //       eprintln!("Error: Makefile not found in embedded assets!");
    //    }
    //}




    //match Scripts::get("install_script.sh") {
    //    Some(file) => {
    //        content = String::from_utf8_lossy(file.data.as_ref());
    //        //tracing::debug!("Contents of install_script.sh:\n{}", content);
    //        //println!("Contents of install_script.sh:\n{}", content);
    //        let install_default_conf = Command::new("echo")
    //            .arg(content.as_ref())
    //            .arg("|")
    //            .arg("bash")
    //            .status()
    //            .expect("Failed to execute install script");
    //        if install_default_conf.success() {
    //            println!("Installation script executed successfully.");
    //        } else {
    //            println!("Installation script failed.");
    //        }
    //    }
    //    None => {
    //       eprintln!("Error: install_script.sh not found in embedded assets!");
    //    }
    //}


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

