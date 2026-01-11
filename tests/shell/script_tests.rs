use {
    shortcut::config::override_config_file,
    std::{env, path::PathBuf, process::Command},
};

#[test]
fn posix_script() {
    setup("posix");
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
}

#[test]
fn bash_script() {}

#[test]
fn power_shell_script() {}

#[test]
fn command_prompt_script() {}

fn setup(name: &str) {
    let mut config_file = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    config_file.push(name);
    override_config_file(config_file);
}
