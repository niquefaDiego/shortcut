use {
    config::{ConfigAddResult, ConfigRemoveResult},
    shell::{CommandPrompt, PowerShell, Shell},
    std::path::{Path, PathBuf},
};

pub mod config;
pub mod fs;
pub mod shell;

pub fn setup(command: String, path_location: PathBuf) -> Result<(), String> {
    let config = config::create_config(&command, &path_location)?;

    // Find and configure command prompt
    match CommandPrompt::new() {
        Err(msg) => println!("Unexpected error looking for Command Prompt: {}", msg),
        Ok(shell) => {
            shell.map(|x| x.configure(&config));
        }
    };

    // Find and configure power shell
    match PowerShell::new() {
        Err(msg) => println!("Unexpected error looking for PowerShell: {}", msg),
        Ok(shell) => {
            shell.map(|x| x.configure(&config));
        }
    }

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
    let add_result = config::add_shortcut(&key, &target)?;
    match add_result {
        ConfigAddResult::NoChange => println!(
            "Nothing done, shortcut already exists: {} -> {}",
            key, target
        ),
        ConfigAddResult::Created(sc) => {
            println!("Successfully added shortcut: {} -> {}", sc.key, sc.value)
        }
        ConfigAddResult::Updated(existing, added) => {
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
    let remove_result = config::remove_shortcut(&key)?;
    match remove_result {
        ConfigRemoveResult::NotFound => println!("Did not find any shortcut for key \"{}\"", key),
        ConfigRemoveResult::Removed(removed) => {
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
