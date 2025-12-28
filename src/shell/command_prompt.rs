use {
    super::{Shell, copy_strs as copy},
    crate::{config::Config, fs},
    std::path::Path,
    which::which,
};

pub struct CommandPrompt {}
const CMD: &str = "Command Prompt (CMD)";

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

    fn setup(&self, config: &Config) -> Result<(), String> {
        println!("Setting up {}", self.name());
        let file_name = config.command.to_string() + ".bat";
        let file_path = Path::new(&config.path_location).join(file_name);
        let file_content = get_bat_file_content(config);
        fs::write_lines(&file_path, &file_content)
    }
}

// private methods

fn get_bat_file_content(config: &Config) -> Vec<String> {
    let mut x: Vec<String> = vec![];
    x.reserve(30 + config.shortcuts.len() * 2);
    x.push(format!(":: {}", copy::HEADER_COMMENT_1).to_string());
    x.push(format!(":: {}", copy::HEADER_COMMENT_2).to_string());
    x.push(r#"@ECHO OFF"#.to_string());

    // {command} + <KEY> <TARGET>
    #[cfg(debug_assertions)]
    x.push(r#"ECHO [CMD] Batch execution started"#.to_string());
    x.push(r#"IF "%1"=="+" ("#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"    ECHO [CMD] Running: $ shortcut add "%2" "%3""#.to_string());
    x.push(r#"    shortcut add "%2" "%3""#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"    ECHO [CMD] shortcut add finished"#.to_string());
    x.push(r#"    EXIT /B 0"#.to_string());

    // {command} - <KEY>
    x.push(r#") ELSE IF "%1"=="-" ("#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"    ECHO [CMD] Running: $ shortcut remove "%2""#.to_string());
    x.push(r#"    shortcut remove "%2""#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"    ECHO [CMD] shortcut remove finished"#.to_string());
    x.push(r#"    EXIT /B 0"#.to_string());

    // {command} *
    x.push(r#") ELSE IF "%1"=="*" ("#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"    ECHO [CMD] Running: $ shortcut list"#.to_string());
    x.push(r#"    shortcut list"#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"    ECHO [CMD] shortcut list finished"#.to_string());
    x.push(r#"    EXIT /B 0"#.to_string());

    // {comand} <KEY>
    for shortcut in &config.shortcuts {
        x.push(format!(r#") ELSE IF "%1"=="{}" ("#, shortcut.key).to_string());
        #[cfg(debug_assertions)]
        x.push(r#"    ECHO [CMD] Using shortcut "%1""#.to_string());
        x.push(format!(r#"    CD /D "{}""#, shortcut.value).to_string());
    }

    // If nothing above matches, do a normal CD with the /D flag to automatically change disk
    x.push(r#") ELSE ("#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"    ECHO [CMD] Chaging directory to "%1""#.to_string());
    x.push(r#"    CD /D "%1""#.to_string());
    x.push(r#")"#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"ECHO [CMD] Batch execution finished"#.to_string());

    x
}
