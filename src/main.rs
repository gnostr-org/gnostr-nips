use std::process::Command;
use std::borrow::Cow;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use rust_embed::Embed;
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, Registry};
use tracing::{info, debug};
use std::fs::File;
use std::io::Write;

#[derive(Embed)]
#[folder = "./template"]
struct Template;

fn canonicalize_path(path: &Path) -> io::Result<PathBuf> {
    // First, make the path absolute if it's not already.
    let absolute_path = if path.is_relative() {
        let current_dir = env::current_dir()?;
        current_dir.join(path)
    } else {
        path.to_path_buf()
    };

    // Then, canonicalize the absolute path. This resolves symbolic links
    // and removes redundant components like ".." and ".".
    fs::canonicalize(absolute_path)
}

fn extract(filename: &str){


   // Create a Path from the current directory.
    let current_dir_path = Path::new(".");

    let relative_dot = Path::new(".");
    let canonical_dot = canonicalize_path(relative_dot).expect("");
    println!("Canonical path of '.': {}", canonical_dot.display());

    // You can now work with this Path object.
    println!("Path to current directory: {}", current_dir_path.display());

    // You can also use it to join with other paths relative to the current directory.
    let sub_dir_path = current_dir_path.join("my_folder");
    println!("Path to a subdirectory: {}", sub_dir_path.display());

    let file_path = current_dir_path.join("my_file.txt");
    println!("Path to a file: {}", file_path.display());

    //let output_path = Path::new("extract_result").join(filename);
    let output_path = Path::new(".").join(filename);
    let _ = std::fs::create_dir_all(output_path.parent().unwrap());
    match Template::get(filename) {
        Some(embedded_file) => {
            let outfile = File::create(&output_path);
            outfile.expect("").write_all(embedded_file.data.as_ref()).expect("");
            println!("Successfully extracted '{}' to '{}'", filename, output_path.display());
        }
        None => {
            eprintln!("Error: Embedded file '{}' not found!", filename);
        }
    }

}

//fn main() -> Result<(), std::io::Error> {
//fn main() -> io::Result<()> {

fn main() {

    let subscriber = Registry::default()
        .with(fmt::layer().with_writer(std::io::stdout)) // Configure the fmt layer
        .with(EnvFilter::from_default_env());        // Add the EnvFilter layer
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");


    // Example 1: Relative path "." (current directory)
    let relative_dot = Path::new(".");
    let canonical_dot = canonicalize_path(relative_dot).expect("");
    println!("Canonical path of '.': {}", canonical_dot.display());

    // Example 2: Relative path to a subdirectory
    let relative_subdir = Path::new("src"); // Assuming you have a 'src' directory
    let canonical_subdir = canonicalize_path(relative_subdir).expect("");
    println!("Canonical path of 'src': {}", canonical_subdir.display());

    // Example 3: Absolute path
    let absolute_path_str = "/bin/ls"; // Example on Unix-like systems
    #[cfg(windows)]
    let absolute_path_str = "C:\\Windows\\System32\\cmd.exe"; // Example on Windows
    //#[cfg(windows)]
    let absolute_path = Path::new(absolute_path_str);
    //#[cfg(windows)]
    let canonical_absolute = canonicalize_path(absolute_path).expect("");
    //#[cfg(windows)]
    println!("Canonical path of '{}': {}", absolute_path_str, canonical_absolute.display());

    // Example 4: Path with ".."
    let path_with_parent = Path::new("template");
    let canonical_with_parent = canonicalize_path(path_with_parent).expect("");
    println!("Canonical path of '{}': {}", path_with_parent.display(), canonical_with_parent.display());



    let filename_to_extract = "Makefile";
	extract(filename_to_extract);
    let filename_to_extract = "install_script.sh";
	extract(filename_to_extract);
    let filename_to_extract = "default_config.conf";
	extract(filename_to_extract);

    let install_script = Template::get("install_script.sh").unwrap();
    let mut content: Cow<str>;// = std::str::from_utf8(install_script.data.as_ref()).unwrap().into();
    for filename in Template::iter() {
        //tracing::debug!("Embedded StaticFile:\n{}", filename.as_ref());
        //println!("Embedded StaticFile:\n{}", filename.as_ref());
        if let Some(file) = Template::get(filename.as_ref()) {

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

