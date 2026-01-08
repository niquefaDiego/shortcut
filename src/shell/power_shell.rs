use {
    super::common::{Shell, replace_file_content},
    crate::{config::Config, fs},
    std::{path::PathBuf, process::Command},
    which::which,
};

const POWER_SHELL_EXE: &str = "pwsh";
const WINDOWS_POWER_SHELL_EXE: &str = "powershell";
const PS1_FUNCTION_FILE: &str = include_str!("./script/script.ps1");

/// PowerShell, includes both:
/// - Windows PowerShell: powershell.exe
/// - PowerShell: pwsh.exe
pub struct PowerShell {
    profile_locations: Vec<String>,
}

impl PowerShell {
    pub fn new() -> Result<Option<PowerShell>, String> {
        let mut profile_locations: Vec<String> = Vec::new();
        for exec in [POWER_SHELL_EXE, WINDOWS_POWER_SHELL_EXE] {
            match get_power_shell_default_profile(exec) {
                Err(err) => return Err(err),
                Ok(Some(profile_location)) => profile_locations.push(profile_location),
                Ok(None) => (),
            };
        }
        if profile_locations.len() > 0 {
            return Ok(Some(PowerShell { profile_locations }));
        }
        Ok(None)
    }
}

impl Shell for PowerShell {
    fn name(&self) -> &'static str {
        "PowerShell"
    }

    fn try_configure(&self, config: &Config) -> Result<(), String> {
        for profile_location in &self.profile_locations {
            let function = get_power_shell_function(&config);
            setup_power_shell_profile(&config, &profile_location, &function)?;
        }
        Ok(())
    }
}

fn get_power_shell_default_profile(exec: &str) -> Result<Option<String>, String> {
    // See the following docs for more info about PowerPhell profiles.
    // https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_profiles?view=powershell-7.5
    let power_shell_exe = match which(exec) {
        Ok(exe_location) => exe_location,
        Err(_) => return Ok(None),
    };
    println!("{} found at {}", POWER_SHELL_EXE, power_shell_exe.display());
    let mut command = Command::new(exec);
    // https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_powershell_exe?view=powershell-5.1
    let command = command.args(["-Command", "Write-Output $PROFILE"]);
    let output = match command.output() {
        Ok(output) => output,
        Err(err) => {
            let msg = format!("Could not get PowerShell default profile: {}", err);
            return Err(msg.to_string());
        }
    };
    if !output.status.success() {
        let msg = format!(
            r#"`{}.exe -Command "Write-Output" $PROFILE"` \
                  exited with non-success status code"#,
            exec
        );
        return Err(msg.to_string());
    }
    // This will probably fail in some older versions of powershell, need some logic to parse
    // different types of encoding.
    let power_shell_output = match String::from_utf8(output.stdout) {
        Ok(str) => str,
        Err(err) => {
            let msg = format!(
                r#"Error decoding PowerShell output to get profile, \
                      updating PowerShell might help: {}"#,
                err
            );
            return Err(msg.to_string());
        }
    };
    let profile_location = power_shell_output.trim().to_string();
    Ok(Some(profile_location))
}

fn setup_power_shell_profile(
    config: &Config,
    profile: &str,
    function: &Vec<String>,
) -> Result<(), String> {
    let profile = PathBuf::from(profile);
    let existing_profile_content = fs::read_lines(&profile)?;
    match existing_profile_content {
        None => {
            fs::ensure_file_parent_dir(&profile)?;
            fs::write_lines(&profile, function)?;
        }
        Some(existing_content) => {
            let function: Vec<String> = get_power_shell_function(config);
            let new_content = replace_file_content(existing_content, &function);
            fs::write_lines(&profile, &new_content)?;
        }
    }
    Ok(())
}

fn get_power_shell_function(config: &Config) -> Vec<String> {
    let mut new_content: Vec<String> = PS1_FUNCTION_FILE
        .to_owned()
        .split("\n")
        .filter(|x| x.trim().len() > 0)
        .map(|s| s.to_owned())
        .collect();
    new_content[3] = format!("function {} {{", config.command).to_string();
    new_content
}
