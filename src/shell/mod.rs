use crate::config::Config;

pub mod command_prompt;
pub mod power_shell;

pub use command_prompt::CommandPrompt;
pub use power_shell::PowerShell;

pub trait Shell {
    fn name(&self) -> &'static str;
    fn try_configure(&self, config: &Config) -> Result<(), String>;

    fn configure(&self, config: &Config) {
        println!("Setting up {}", self.name());
        match self.try_configure(config) {
            Ok(()) => println!("Successfully set up {}", self.name()),
            Err(msg) => println!("Erring setting up {}: {}", self.name(), msg),
        }
    }
}
