use {
    super::Shell,
    crate::{config::Config, fs},
    std::{path::PathBuf, process::Command},
    which::which,
};

const POWER_SHELL_EXE: &str = "pwsh";
const WINDOWS_POWER_SHELL_EXE: &str = "powershell";
const PROFILE_CONTENT: &str = include_str!("./script/script.ps1");

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
        POWER_SHELL_EXE
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

fn replace_file_content(existing_content: Vec<String>, new_content: &Vec<String>) -> Vec<String> {
    assert!(new_content.len() > 0);
    let fr = existing_content
        .iter()
        .position(|x| x.trim() == new_content[0].trim());
    if let Some(fr) = fr {
        let last_line = new_content
            .last()
            .expect("already asserted new_content.len() > 0")
            .trim();
        let to = existing_content
            .iter()
            .skip(fr)
            .position(|x| x.trim() == last_line);
        if let Some(to) = to {
            let to = to + fr;
            let mut updated_content = existing_content;
            let suffix = updated_content[to + 1..].to_owned();
            updated_content.truncate(fr);
            updated_content.extend(new_content.clone());
            updated_content.extend_from_slice(&suffix);
            return updated_content;
        }
    }
    let mut existing_content = existing_content;
    existing_content.extend_from_slice(new_content);
    existing_content
}

fn get_power_shell_function(config: &Config) -> Vec<String> {
    let mut new_content: Vec<String> = PROFILE_CONTENT
        .to_owned()
        .split("\n")
        .filter(|x| x.trim().len() > 0)
        .map(|s| s.to_owned())
        .collect();
    new_content[3] = format!("function {} {{", config.command).to_string();
    new_content
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_replace_file_content() {
        let existing_content: Vec<String> = vec!["0", "1", "2", "X", "A", "B", "C", "D", "Y"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let new_content: Vec<String> = vec!["X", "a", "b", "c", "Y"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let expected: Vec<String> = vec!["0", "1", "2", "X", "a", "b", "c", "Y"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let updated_content = replace_file_content(existing_content, &new_content);
        assert_eq!(expected, updated_content);

        let existing_content: Vec<String> = vec!["X", "1", "2", "3", "Y"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let new_content = existing_content.clone();
        let expected = existing_content.clone();
        let updated_content = replace_file_content(existing_content, &new_content);
        assert_eq!(expected, updated_content);
    }
}
