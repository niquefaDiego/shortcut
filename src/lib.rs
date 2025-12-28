use {
    config::{Config, ConfigAddResult, ConfigRemoveResult},
    shell::{CommandPrompt, PowerShell, Shell},
    std::path::{Path, PathBuf},
};

pub mod config;
pub mod fs;
pub mod shell;

pub fn setup(
    command: String,
    path_location: PathBuf,
    power_shell_profile: Option<PathBuf>,
) -> Result<(), String> {
    let config = config::create_config(&command, &path_location, power_shell_profile)?;
    notify_config_change(&config)?;

    Ok(())
}

pub fn list() -> Result<(), String> {
    let config = config::get_config()?;
    println!("Command: \"{}\"", config.command);
    println!("Shortcuts ({}):", config.shortcuts.len());
    for shortcut in &config.shortcuts {
        let text = format!("  {} -> {}", shortcut.key, shortcut.value);
        println!("{}", text);
    }
    Ok(())
}

pub fn add(key: String, target: PathBuf) -> Result<(), String> {
    let target = fs::to_absolute_path(&target)?;
    let target = target.to_string_lossy();
    let (config, add_result) = config::add_shortcut(&key, &target)?;
    match add_result {
        ConfigAddResult::NoChange => println!(
            "Nothing done, shortcut already exists: {} -> {}",
            key, target
        ),
        ConfigAddResult::Created(sc) => {
            notify_config_change(&config)?;
            println!("Successfully added shortcut: {} -> {}", sc.key, sc.value)
        }
        ConfigAddResult::Updated(existing, added) => {
            notify_config_change(&config)?;
            println!("Successfully updated shortcut.");
            println!(
                "Existing shortcut was: {} -> {}",
                existing.key, existing.value
            );
            println!("New shortcut is: {} -> {}", added.key, added.value);
        }
    };
    Ok(())
}

pub fn remove(key: String) -> Result<(), String> {
    let (config, remove_result) = config::remove_shortcut(&key)?;
    match remove_result {
        ConfigRemoveResult::NotFound => println!("Did not find any shortcut for key \"{}\"", key),
        ConfigRemoveResult::Removed(removed) => {
            notify_config_change(&config)?;
            println!(
                "Successfully removed shortcut {} -> {}",
                removed.key, removed.value
            )
        }
    }
    Ok(())
}

pub fn get(key: String) -> Result<(), String> {
    let config = config::get_config()?;
    if Path::new(&key).is_dir() {
        println!("{}", key);
        return Ok(());
    }
    for shortcut in &config.shortcuts {
        if key == shortcut.key {
            println!("{}", shortcut.value.clone());
            return Ok(());
        }
    }
    println!("{}", key);
    Ok(())
}

// ----- private methods -----

fn notify_config_change(config: &Config) -> Result<(), String> {
    let command_prompt = CommandPrompt {};
    if command_prompt.available()? {
        command_prompt.setup(&config)?;
        let power_shell = PowerShell {};
        power_shell.setup(&config)?;
    }
    Ok(())
}
