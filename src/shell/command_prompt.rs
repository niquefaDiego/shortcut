use {
    super::common::Shell,
    crate::{config::Config, fs},
    colored::Colorize,
    std::path::Path,
    which::{Error as WhichError, which},
};

pub struct CommandPrompt {}

const NAME: &str = "Command Prompt (CMD)";
const BAT_FILE_CONTENT: &str = include_str!("./script/script.bat");

impl CommandPrompt {
    pub fn new() -> Result<Option<CommandPrompt>, String> {
        match which("cmd") {
            Ok(location) => {
                let instance = CommandPrompt {};
                println!("{} found at {}", NAME, location.display());
                Ok(Some(instance))
            }
            Err(WhichError::CannotFindBinaryPath) => Ok(None),
            Err(err) => Err(format!("Error finding {}: {}", NAME, err).to_string()),
        }
    }
}

impl Shell for CommandPrompt {
    fn name(&self) -> &'static str {
        NAME
    }

    fn try_configure(&self, config: &Config) -> Result<(), String> {
        match &config.path_location {
            None => {
                let message = format!(
                    "Skipping setup for {} as --path-location was not given.\n See shortcut --help.",
                    self.name(),
                );
                println!("{}", message.yellow());
                Ok(())
            }
            Some(path_dir) => {
                let file_name = config.command.to_string() + ".bat";
                let file_path = Path::new(path_dir).join(file_name);
                fs::write_str(&file_path, BAT_FILE_CONTENT)
            }
        }
    }
}
