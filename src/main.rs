use rust_embed::Embed;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Stdio;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

#[derive(Embed)]
#[folder = "."]
#[exclude = "*.DS_Store"]
#[exclude = "target/*"]
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
    tracing::debug!("{}", script_path.display());
    Ok(())
}

fn execute_script(script_path: &Path) -> io::Result<()> {
    tracing::debug!("Executing script: {}", script_path.display());

    let log_file = File::create("output.log")?;
    let error_file = File::create("error.log")?;

    let mut command = Command::new(script_path);
    command
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(error_file));

    let status = command.spawn()?.wait()?;

    //    let status = Command::new(script_path).status()?;

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
    tracing::debug!("Path to current directory: {}", current_dir_path.display());

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
            tracing::debug!(
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

fn main() {
    let subscriber = Registry::default()
        .with(fmt::layer().with_writer(std::io::stdout)) // Configure the fmt layer
        .with(EnvFilter::from_default_env()); // Add the EnvFilter layer
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    // Example 1: Relative path "." (current directory)
    let relative_dot = Path::new(".");
    let canonical_dot = canonicalize_path(relative_dot).expect("");
    tracing::debug!("Canonical path of '.': {}", canonical_dot.display());

    // Example 2: Relative path to a subdirectory
    let relative_subdir = Path::new("src"); // Assuming you have a 'src' directory
    let canonical_subdir = canonicalize_path(relative_subdir).expect("");
    tracing::debug!("Canonical path of 'src': {}", canonical_subdir.display());

    // Example 3: Absolute path
    #[cfg(windows)]
    let absolute_path_str = "/bin/ls"; // Example on Unix-like systems
    #[cfg(windows)]
    let absolute_path_str = "C:\\Windows\\System32\\cmd.exe"; // Example on Windows
                                                              //#[cfg(windows)]
    #[cfg(windows)]
    let absolute_path = Path::new(absolute_path_str);
    #[cfg(windows)]
    let canonical_absolute = canonicalize_path(absolute_path).expect("");
    #[cfg(windows)]
    tracing::debug!(
        "Canonical path of '{}': {}",
        absolute_path_str,
        canonical_absolute.display()
    );

    // Example 4: Path with ".."
    //let path_with_parent = Path::new("some_folder/../another_folder/file.txt");
    //let canonical_with_parent = canonicalize_path(path_with_parent).expect("");
    //println!("Canonical path of '{}': {}", path_with_parent.display(), canonical_with_parent.display());

    let filename_to_extract = "README.md";
    extract(filename_to_extract);
    //let filename_to_extract = "GNUmakefile";
    //extract(filename_to_extract);
    let filename_to_extract = "install_script.sh";
    extract(filename_to_extract);

    let script_name = "install_script.sh";

    let relative_dot = Path::new(".");
    let canonical_dot = canonicalize_path(relative_dot).expect("");
    tracing::debug!("Canonical path of '.': {}", canonical_dot.display());
    let script_path = Path::new(".").join(script_name);

    if script_path.exists() {
        tracing::debug!("Attempting to make '{}' executable...", script_name);
        match make_executable(&script_path) {
            Ok(_) => tracing::debug!("Successfully made '{}' executable.", script_name),
            Err(e) => eprintln!("Error making '{}' executable: {}", script_name, e),
        }

        tracing::debug!("Now attempting to execute '{}'...", script_name);
        execute_script(&script_path).expect("");
    } else {
        eprintln!(
            "Error: Script '{}' does not exist in the current directory.",
            script_name
        );
    }

    let filename_to_extract = "default_config.conf";
    extract(filename_to_extract);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_embed::RustEmbed;
    use std::thread;
    use std::time::Duration;

    //#[derive(RustEmbed)]
    //#[folder = "test_files"] // Create a 'test_files' directory with 'tabbed.txt'
    //struct EmbeddedAssets;

    #[test]
    fn test_make_test_file() {
        // Create a test file with tabs
        let test_file_content = "Line with\ttab.\nAnother\t\ttabbed line.\n\tLeading tab.\n\\tThis line starts with a literal backslash-t.\nThis line has a tab in the middle:	like this.";
        let script_path = Path::new("tabbed.txt");
        let path = Path::new(".");
        let test_files = Path::new(path).join("test_files");
        let script_path = Path::new(&test_files).join(script_path);
        tracing::debug!("{}", script_path.display());
        std::fs::create_dir_all(test_files).unwrap();

        let duration = Duration::from_secs(1);
        thread::sleep(duration);
        std::fs::write(script_path, test_file_content).unwrap();
    }
    #[test]
    fn test_preserve_tabs() {
        test_make_test_file();
        // Create a test file with tabs
        let test_file_content = "Line with\ttab.\nAnother\t\ttabbed line.\n\tLeading tab.\n\\tThis line starts with a literal backslash-t.\nThis line has a tab in the middle:	like this.";
        println!("{}", test_file_content);
        let script_path = Path::new("tabbed.txt");
        println!("{}", script_path.display());
        let path = Path::new(".");
        println!("{}", path.display());
        let test_files = Path::new(path).join("test_files");
        println!("{}", test_files.display());
        let script_path = Path::new(&test_files).join(script_path);
        println!("{}", script_path.display());

        if test_files.exists() {
            println!("Directory '{}' already exists.", test_files.display());
        } else {
            println!("Creating directory '{}'...", test_files.display());
            fs::create_dir_all(&test_files).expect("");
            println!("Directory '{}' created.", test_files.display());
        }

        //std::fs::create_dir_all(test_files).unwrap();
        std::fs::write(script_path, test_file_content).unwrap();

        #[derive(RustEmbed)]
        #[folder = "./test_files"]
        #[exclude = "*.DS_Store"]
        struct EmbeddedAssets;

        for file in EmbeddedAssets::iter() {
            println!("Found asset: {}", file.as_ref());

            match EmbeddedAssets::get(file.as_ref()) {
                Some(file) => {
                    let content = String::from_utf8_lossy(file.data.as_ref());
                    //tracing::debug!("Contents of install_script.sh:\n{}", content);
                    println!("Contents of file.data.as_ref():\n{}", content);
                    let install_default_conf = Command::new("echo")
                        .arg(content.as_ref())
                        //.arg("|")
                        //.arg("bash")
                        .status()
                        .expect("Failed to execute install script");
                    if install_default_conf.success() {
                        println!("Installation script executed successfully.");
                    } else {
                        eprintln!("Installation script failed.");
                    }
                }
                None => {
                    eprintln!("Error: tabbed.txt not found in embedded assets!");
                    //println!("Error: tabbed.txt not found in embedded assets!");
                }
            }
        }

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
            assert!(embedded_content
                .lines()
                .nth(0)
                .expect("REASON")
                .contains("h\t"));
            assert!(embedded_content
                .lines()
                .nth(1)
                .expect("REASON")
                .contains("r\t"));
            assert!(embedded_content
                .lines()
                .nth(2)
                .expect("REASON")
                .contains("\tL"));
            //assert!(embedded_content.lines().nth(3).expect("REASON").contains("\tT"));
            //assert!(embedded_content.lines().nth(4).expect("REASON").contains(":\tl"));
        } else {
            //panic!("Failed to embed 'tabbed.txt'");
            println!("Failed to embed 'tabbed.txt'");
        }
        let script_path = Path::new("tabbed.txt");
        let path = Path::new(".");
        let test_files = Path::new(path).join("test_files");
        let script_path = Path::new(&test_files).join(script_path);
        println!("{}", script_path.display());
        //let _ = remove_dir_all_custom();
    }
    #[test]
    fn remove_dir_all_custom() -> io::Result<()> {
        let dot_path = Path::new(".");
        let path = Path::new(dot_path).join("test_files");
        let _ = if path.is_dir() {
            for entry_result in fs::read_dir(&path)? {
                let entry = entry_result?;
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    remove_dir_all_custom()?; // Recursively remove subdirectories
                } else {
                    fs::remove_file(&entry_path)?; // Remove files
                }
            }
            fs::remove_dir(path)?; // Finally, remove the now-empty directory
            Ok(())
        } else if path.is_file() {
            fs::remove_file(path)?; // Remove a file if the path points to one
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Path not found: {}", path.display()),
            ))
        };
        test_make_test_file();
        Ok(())
    }
}
