use {
    shortcut::config::override_config_file,
    std::{env, path::PathBuf, process::Command},
};

#[test]
fn posix_script() {
    set_up_test("posix");
    let cmd = Command::new("sh")
        .arg("tests/shell/script/posix-test.sh")
        .output()
        .expect("posix scripot should run");
    if cmd.status.success() {
        let stdout = str::from_utf8(&cmd.stdout).expect("Invalid UTF-8");
        println!("Status: success");
        println!("Stdout: {}", stdout);
    } else {
        let stderr = str::from_utf8(&cmd.stderr).expect("Invalid UTF-8");
        eprintln!("Status: failed");
        eprintln!("Stderr: {}", stderr);
    }
    // TODO: Validate script worked correctly with assertions
}

#[test]
fn bash_script() {
    set_up_test("bash");
    // TODO
}

#[test]
fn power_shell_script() {
    set_up_test("pwsh");
    // TODO
}

#[test]
fn command_prompt_script() {
    set_up_test("cmd");
    // TODO
}

fn set_up_test(name: &str) {
    let mut config_file = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    config_file.push(format!("{}.cfg", name));
    println!("Configuring with settings file: {}", config_file.display());
    override_config_file(config_file);
    shortcut::set_up("s".to_string(), None).expect("setup should not fail in integration tests");
}
