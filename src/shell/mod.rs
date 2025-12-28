use which::which;

struct PowerShell {}
struct CommandPrompt {}

pub trait Shell {
    fn name(&self) -> &'static str;
    fn available(&self) -> Result<bool, String>;
    fn setup(&self, command: &str) -> Result<(), String>;
}

impl Shell for PowerShell {
    fn name(&self) -> &'static str {
        "PowerShell"
    }
    fn available(&self) -> Result<bool, String> {
        match which("PowerShell") {
            Ok(location) => {
                println!("{} found at {}", self.name(), location.display());
                return Ok(true);
            }
            Err(_) => Ok(false),
        }
    }
    fn setup(&self, command: &str) -> Result<(), String> {
        dbg!("Setting up powershell");
        Ok(())
    }
}

impl Shell for CommandPrompt {
    fn name(&self) -> &'static str {
        "Command Prompt (CMD)"
    }
    fn available(&self) -> Result<bool, String> {
        match which("cmd") {
            Ok(location) => {
                println!("{} found at {}", self.name(), location.display());
                return Ok(true);
            }
            Err(_) => Ok(false),
        }
    }

    fn setup(&self, _: &str) -> Result<(), String> {
        Ok(())
    }
}

pub fn supported_shells() -> Vec<Box<dyn Shell>> {
    let mut ans: Vec<Box<dyn Shell>> = vec![];
    ans.push(Box::new(PowerShell {}));
    ans.push(Box::new(CommandPrompt {}));
    return ans;
}
