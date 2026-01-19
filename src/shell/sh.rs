use {
    super::common::{Shell, replace_file_content},
    crate::config::Config,
    crate::fs,
    std::path::Path,
    which::{Error as WhichError, which},
};

pub struct BourneShell {}

impl BourneShell {
    pub fn new() -> Result<Option<BourneShell>, String> {
        match which("sh") {
            Ok(location) => {
                let instance = BourneShell {};
                println!("{} found at {}", instance.name(), location.display());
                Ok(Some(instance))
            }
            Err(WhichError::CannotFindBinaryPath) => Ok(None),
            Err(msg) => Err(format!("Error finding {}: {}", NAME, msg).to_string()),
        }
    }
}

const NAME: &str = "Bourne Shell";
const SH_FUNCTION_FILE: &str = include_str!("./script/script-posix.sh");

impl Shell for BourneShell {
    fn name(&self) -> &'static str {
        NAME
    }

    fn try_configure(&self, config: &Config) -> Result<(), String> {
        let sh_profile_path = std::path::PathBuf::from("~/.profile");
        let sh_profile_path = fs::to_absolute_path(&sh_profile_path)?;
        let function = get_sh_function(&config);
        setup_sh(&config, &sh_profile_path, &function)?;
        Ok(())
    }
}

fn setup_sh(config: &Config, profile: &Path, function: &Vec<String>) -> Result<(), String> {
    let existing_content = fs::read_lines(&profile)?;
    match existing_content {
        None => {
            fs::ensure_file_parent_dir(&profile)?;
            fs::write_lines(&profile, function)?;
        }
        Some(existing_content) => {
            let function: Vec<String> = get_sh_function(config);
            let new_content = replace_file_content(existing_content, &function);
            fs::write_lines(&profile, &new_content)?;
        }
    }
    Ok(())
}

fn get_sh_function(config: &Config) -> Vec<String> {
    let mut new_content: Vec<String> = SH_FUNCTION_FILE
        .to_owned()
        .split("\n")
        .filter(|x| x.trim().len() > 0)
        .map(|s| s.to_owned())
        .collect();
    new_content[4] = format!("{} {{", config.command).to_string();
    new_content
}
