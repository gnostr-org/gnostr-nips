use rust_embed::Embed;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

#[derive(Embed)]
#[folder = "./template"]
struct Template;

fn make_executable(script_path: &Path) -> io::Result<()> {
    // Get the current permissions of the file.
    let mut permissions = fs::metadata(script_path)?.permissions();

    // Add execute permissions for the owner, group, and others (chmod +x).
    permissions.set_mode(permissions.mode() | 0o111);

    // Set the new permissions for the file.
    fs::set_permissions(script_path, permissions)?;

    let script_name = "install_script.sh";
    let script_path = Path::new(script_name);
    //let script_path = Path::new(".").join(script_name);

    println!("{}", script_path.display());
    //if script_path.exists() {
    //    println!("34:Attempting to make '{}' executable...", script_name);
    //    match make_executable(&script_path) {
    //        Ok(_) => println!("Successfully made '{}' executable.", script_name),
    //        Err(e) => eprintln!("Error making '{}' executable: {}", script_name, e),
    //    }

    //    let output_path = Path::new(".").join(script_name);
    //    println!("Now attempting to execute '{}'...", output_path.display());
    //    execute_script(&script_path)?; // Execute the script after making it executable

    //} else {
    //    eprintln!("Error: Script '{}' does not exist in the current directory.", script_name);
    //}

    Ok(())
}

fn execute_script(script_path: &Path) -> io::Result<()> {
    println!("Executing script: {}", script_path.display());
    let status = Command::new(script_path).status()?; // Execute the command and wait for it to finish

    if status.success() {
        println!("Script '{}' executed successfully.", script_path.display());
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

fn extract(filename: &str) {
    // Create a Path from the current directory.
    let current_dir_path = Path::new(".");

    // You can now work with this Path object.
    println!("Path to current directory: {}", current_dir_path.display());

    //// You can also use it to join with other paths relative to the current directory.
    //let sub_dir_path = current_dir_path.join("my_folder");
    //println!("Path to a subdirectory: {}", sub_dir_path.display());

    //let file_path = current_dir_path.join("my_file.txt");
    //println!("Path to a file: {}", file_path.display());

    //let output_path = Path::new("extract_result").join(filename);
    let output_path = Path::new(".").join(filename);
    //let _ = std::fs::create_dir_all(output_path.parent().unwrap());
    match Template::get(filename) {
        Some(embedded_file) => {
            let outfile = File::create(&output_path);
            outfile
                .expect("")
                .write_all(embedded_file.data.as_ref())
                .expect("");
            println!(
                "Successfully extracted '{}' to '{}'",
                filename,
                output_path.display()
            );
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
        .with(EnvFilter::from_default_env()); // Add the EnvFilter layer
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
    println!(
        "Canonical path of '{}': {}",
        absolute_path_str,
        canonical_absolute.display()
    );

    // Example 4: Path with ".."
    //let path_with_parent = Path::new("some_folder/../another_folder/file.txt");
    //let canonical_with_parent = canonicalize_path(path_with_parent).expect("");
    //println!("Canonical path of '{}': {}", path_with_parent.display(), canonical_with_parent.display());

    let filename_to_extract = "Makefile";
    extract(filename_to_extract);
    //let filename_to_extract = "GNUmakefile";
    //extract(filename_to_extract);
    let filename_to_extract = "install_script.sh";
    extract(filename_to_extract);

    let script_name = "install_script.sh";

    let relative_dot = Path::new(".");
    let canonical_dot = canonicalize_path(relative_dot).expect("");
    println!("Canonical path of '.': {}", canonical_dot.display());
    let script_path = Path::new(".").join(script_name);

    if script_path.exists() {
        println!("170:Attempting to make '{}' executable...", script_name);
        match make_executable(&script_path) {
            Ok(_) => println!("Successfully made '{}' executable.", script_name),
            Err(e) => eprintln!("Error making '{}' executable: {}", script_name, e),
        }

        println!("Now attempting to execute '{}'...", script_name);
        execute_script(&script_path).expect("");
    } else {
        eprintln!(
            "Error: Script '{}' does not exist in the current directory.",
            script_name
        );
    }

    let filename_to_extract = "default_config.conf";
    extract(filename_to_extract);

    //let install_script = Template::get("install_script.sh").unwrap();
    //let mut content: Cow<str>;// = std::str::from_utf8(install_script.data.as_ref()).unwrap().into();
    //for filename in Template::iter() {
    //    //tracing::debug!("Embedded StaticFile:\n{}", filename.as_ref());
    //    //println!("Embedded StaticFile:\n{}", filename.as_ref());
    //    if let Some(file) = Template::get(filename.as_ref()) {

    //        let mut outfile = File::create(&filename.as_ref());
    //        content = String::from_utf8_lossy(file.data.as_ref());
    ////        outfile?.write_all(content).expect("");

    //        content = String::from_utf8_lossy(file.data.as_ref());
    //        //tracing::debug!("Content:\n{}", content);
    //        //println!("Content:\n{}", content);
    //    }
    //}

    //let touch_makefile = Command::new("touch")
    //    .arg("Makefile")
    //    .status()
    //    .expect("Failed to execute install script");

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

#[cfg(test)]
mod tests {
    use super::*;
    use rust_embed::RustEmbed;

    #[derive(RustEmbed)]
    #[folder = "./test_files"] // Create a 'test_files' directory with 'tabbed.txt'
    struct EmbeddedAssets;

    #[test]
    fn test_preserve_tabs() {
        // Create a test file with tabs
        let test_file_content = "Line with\ttab.\nAnother\t\ttabbed line.\n\tLeading tab.\n\\tThis line starts with a literal backslash-t.\nThis line has a tab in the middle:	like this.";
        std::fs::create_dir_all("test_files").unwrap();
        std::fs::write("test_files/tabbed.txt", test_file_content).unwrap();

        if let Some(file) = EmbeddedAssets::get("tabbed.txt") {
            let embedded_content = String::from_utf8_lossy(file.data.as_ref()).to_string();
            assert_eq!(embedded_content, test_file_content);
            assert!(embedded_content.contains("\t"));
            assert!(embedded_content.contains("	"));
            assert!(embedded_content.lines().nth(0).unwrap().contains('\t')); //Line with	tab.
            assert!(embedded_content.lines().nth(0).unwrap().contains("	"));
			//
            assert!(embedded_content.lines().nth(1).unwrap().contains('\t')); //Another		tabbed line.
            assert!(embedded_content.lines().nth(1).unwrap().contains("	"));
            assert!(embedded_content.lines().nth(1).unwrap().contains("\t\t")); //Another		tabbed line.
            assert!(embedded_content.lines().nth(1).unwrap().contains("		"));
			//
            assert!(embedded_content.lines().nth(2).unwrap().contains('\t')); //	Leading tab.
            assert!(embedded_content.lines().nth(2).unwrap().contains("	"));
			//
            //assert!(embedded_content.lines().nth(3).unwrap().contains('\t')); //\tThis line starts with a literal backslash-t
            //assert!(embedded_content.lines().nth(3).unwrap().contains("\t"));
            //assert!(embedded_content.lines().nth(3).unwrap().contains("	"));
			//
            assert!(embedded_content.lines().nth(4).unwrap().contains('\t'));
            assert!(embedded_content.lines().nth(4).unwrap().contains("	"));
			//
            assert!(embedded_content.lines().nth(2).unwrap().starts_with('\t'));
            assert!(embedded_content.lines().nth(2).unwrap().starts_with("	"));
            assert!(!embedded_content.lines().nth(3).unwrap().starts_with('\t'));
            assert!(!embedded_content.lines().nth(3).unwrap().starts_with("\t"));
            assert!(!embedded_content.lines().nth(3).unwrap().starts_with("	"));
            assert!(embedded_content.lines().nth(0).expect("REASON").contains("h\t"));
            assert!(embedded_content.lines().nth(1).expect("REASON").contains("r\t"));
            assert!(embedded_content.lines().nth(2).expect("REASON").contains("\tL"));
            //assert!(embedded_content.lines().nth(3).expect("REASON").contains("\tT"));
            //assert!(embedded_content.lines().nth(4).expect("REASON").contains(":\tl"));
        } else {
            panic!("Failed to embed 'tabbed.txt'");
        }

        // Clean up the test file and directory
        //std::fs::remove_file("test_files/tabbed.txt").unwrap();
        //std::fs::remove_dir("test_files").unwrap();
    }
}
