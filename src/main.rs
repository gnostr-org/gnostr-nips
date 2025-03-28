use std::process::Command;
use std::env;

fn main() {
    let install_script_path = env::var("INSTALL_SCRIPT").unwrap_or(String::from("./install_script.sh"));

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



    let install_default_conf = Command::new(install_script_path)
        .arg("default_config.conf") // Pass an argument to the script, if needed.
        .status()
        .expect("Failed to execute install script");

    if install_default_conf.success() {
        println!("Installation script executed successfully.");
    } else {
        println!("Installation script failed.");
    }
}

