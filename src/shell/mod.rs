use crate::config::Config;

pub mod command_prompt;
pub mod power_shell;
mod copy_strs;

pub struct PowerShell {}
pub struct CommandPrompt {}

pub trait Shell {
    fn name(&self) -> &'static str;
    fn available(&self) -> Result<bool, String>;
    fn setup(&self, config: &Config) -> Result<(), String>;
}
