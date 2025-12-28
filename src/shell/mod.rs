use crate::config::Config;

pub mod command_prompt;
mod copy_strs;
pub mod power_shell;

pub use command_prompt::CommandPrompt;
pub use power_shell::PowerShell;

pub trait Shell {
    fn name(&self) -> &'static str;
    fn setup(&self, config: &Config) -> Result<(), String>;
}
