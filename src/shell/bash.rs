use {
    super::common::{Shell, replace_file_content},
    crate::config::Config,
    crate::fs,
    std::path::Path,
    which::{Error as WhichError, which},
};

pub struct Bash {}

const NAME: &str = "Bash";
const BASH_FUNCTION_FILE: &str = include_str!("./script/bash.sh");

impl Bash {
    pub fn new() -> Result<Option<Bash>, String> {
        match which("bash") {
            Ok(location) => {
                let instance = Bash {};
                println!("{} found at {}", instance.name(), location.display());
                Ok(Some(instance))
            }
            Err(WhichError::CannotFindBinaryPath) => Ok(None),
            Err(msg) => Err(format!("Error finding {}: {}", NAME, msg).to_string()),
        }
    }
}

impl Shell for Bash {
    fn name(&self) -> &'static str {
        NAME
    }

    fn try_configure(&self, config: &Config) -> Result<(), String> {
        let bashrc_dir = std::path::PathBuf::from("~/.bashrc");
        let bashrc_dir = fs::to_absolute_path(&bashrc_dir)?;
        let function = get_bash_function(&config);
        setup_bash(&config, &bashrc_dir, &function)?;
        Ok(())
    }
}

fn setup_bash(config: &Config, profile: &Path, function: &Vec<String>) -> Result<(), String> {
    let existing_bashrc_content = fs::read_lines(&profile)?;
    match existing_bashrc_content {
        None => {
            fs::ensure_file_parent_dir(&profile)?;
            fs::write_lines(&profile, function)?;
        }
        Some(existing_content) => {
            let function: Vec<String> = get_bash_function(config);
            let new_content = replace_file_content(existing_content, &function);
            fs::write_lines(&profile, &new_content)?;
        }
    }
    Ok(())
}

fn get_bash_function(config: &Config) -> Vec<String> {
    let mut new_content: Vec<String> = BASH_FUNCTION_FILE
        .to_owned()
        .split("\n")
        .filter(|x| x.trim().len() > 0)
        .map(|s| s.to_owned())
        .collect();
    new_content[3] = format!("function {} {{", config.command).to_string();
    new_content
}
