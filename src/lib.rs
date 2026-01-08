use {
    colored::Colorize,
    config::{ConfigAddResult, ConfigRemoveResult},
    shell::{Bash, CommandPrompt, PowerShell, Shell},
    std::path::{Path, PathBuf},
};

pub mod config;
pub mod fs;
pub mod shell;

pub fn setup(command: String, path_location: Option<PathBuf>) -> Result<(), String> {
    let config = config::create_config(&command, path_location)?;

    // TODO: Use macro_rules! to avoid repeating myself :)
    // Bash
    match Bash::new() {
        Err(msg) => {
            let msg = format!("Unexpected error looking for Bash: {}", msg);
            eprintln!("{}", msg.red());
        }
        Ok(shell) => {
            shell.map(|x| x.configure(&config));
        }
    }

    // Command prompt
    match CommandPrompt::new() {
        Err(msg) => {
            let msg = format!("Unexpected error looking for Command Prompt: {}", msg);
            eprintln!("{}", msg.red());
        }
        Ok(shell) => {
            shell.map(|x| x.configure(&config));
        }
    };

    // Power shell
    match PowerShell::new() {
        Err(msg) => {
            let msg = format!("Unexpected error looking for Command Prompt: {}", msg);
            eprintln!("{}", msg.red());
        }
        Ok(shell) => {
            shell.map(|x| x.configure(&config));
        }
    }

    Ok(())
}

pub fn list() -> Result<(), String> {
    let mut config = config::get_config()?;
    println!("Command: \"{}\"", config.command);
    if config.shortcuts.len() == 0 {
        println!("No shortcuts. See `shorcuts add --help` for instructions.");
        return Ok(());
    }
    println!("Shortcuts ({}):", config.shortcuts.len());
    config.shortcuts.sort_by(|x, y| {
        let a = x.value.to_lowercase();
        let b = y.value.to_lowercase();
        a.cmp(&b)
    });
    let width = config
        .shortcuts
        .iter()
        .max_by(|x, y| {
            let a = x.key.len();
            let b = y.key.len();
            a.cmp(&b)
        })
        .expect("Already asserted shortcuts list is not empty")
        .key
        .len();
    for shortcut in &config.shortcuts {
        let spaces = String::from_utf8(vec![' ' as u8; 1 + width - shortcut.key.len()])
            .expect("String of 1 or more spaces must be a valid utf-8");
        let text = format!("  {}{}{}", shortcut.key, spaces, shortcut.value);
        println!("{}", text);
    }
    Ok(())
}

pub fn add(key: String, target: PathBuf) -> Result<(), String> {
    let target = fs::to_absolute_path(&target)?;
    let add_result = config::add_shortcut(&key, &target)?;
    match add_result {
        ConfigAddResult::NoChange => println!(
            "Nothing done, shortcut already exists: {} -> {}",
            key,
            target.display()
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
