use {
    super::Shell,
    crate::{config::Config, fs},
    std::path::Path,
    which::which,
};

pub struct CommandPrompt {}
const CMD: &str = "Command Prompt (CMD)";

const BAT_FILE_CONTENT: &str = include_str!("./script/script.bat");

impl CommandPrompt {
    pub fn new() -> Result<Option<CommandPrompt>, String> {
        match which("cmd") {
            Ok(location) => {
                let instance = CommandPrompt {};
                println!("{} found at {}", instance.name(), location.display());
                Ok(Some(instance))
            }
            Err(msg) => Err(format!("Error finding {}: {}", CMD, msg).to_string()),
        }
    }
}

impl Shell for CommandPrompt {
    fn name(&self) -> &'static str {
        CMD
    }

    fn try_configure(&self, config: &Config) -> Result<(), String> {
        let file_name = config.command.to_string() + ".bat";
        let file_path = Path::new(&config.path_location).join(file_name);
        fs::write_str(&file_path, BAT_FILE_CONTENT)
    }
}
