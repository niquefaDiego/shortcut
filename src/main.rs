use {
    clap::{Parser, Subcommand},
    std::process::ExitCode,
    colored::Colorize,
};

mod shell;
mod config;
mod fs;

/// Use shortcuts to quickly cd (change directory) to common directories.
///
/// Run one time set-up: $ shortcut setup --command s
///
/// Add shortcuts:       $ shortcut add dl ~/Downloads
/// 
/// cd with shortcut:    $ s dl
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Do one-time setup of your shells to use shortcuts.
    Setup {
        /// Command used to change directory using the shortcuts.
        #[arg(short, long, default_value_t=String::from("s"))]
        command: String
    },
    /// Adds a shortcut.
    Add {
        /// Shortcut to use go to the target directory.
        key: String,
        /// Absolute or relative path to the target directory.
        target: String
    },
    /// Removes a shortcut
    Remove {
        /// Shortcut key to remove
        key: String,
    },
    /// Lists all the existing shortcuts
    List {
    },
    /// Get the target directory given a key, if there is not shortcut for the given key,
    /// then the key will be returned.
    Get {
        /// Key.
        key: String
    }
}

fn setup(command: String) -> Result<(), String> {
    let config = config::create_config(&command)?;
    let shells = shell::supported_shells();
    for shell in shells {
        if shell.available()? {
            shell.setup(&command)?;
        }
    }
    Ok(())
}

fn list() -> Result<(), String> {
    let config = config::get_config()?;
    println!("Command: \"{}\"", config.command);
    println!("Shortcuts ({}):", config.shortcuts.len());
    for shortcut in &config.shortcuts {
        let text = format!("  {} -> {}", shortcut.key, shortcut.value);
        println!("{}", text);
    }
    Ok(())
}

fn add(key: String, target: String) -> Result<(), String> {
    config::add_shortcut(key, target)
}

fn remove(key: String) -> Result<(), String> {
    config::remove_shortcut(key)
}

fn get(key: String) -> Result<(), String> {
    let config = config::get_config()?;
    // TODO: check if key is a file or directory in the current directory
    for shortcut in &config.shortcuts {
        if key == shortcut.key {
            println!("{}", shortcut.value.clone());
            return Ok(())
        }
    }
    println!("{}", key);
    Ok(())
}

fn main() -> ExitCode {
    let args = Args::parse();
    let result = match args.command {
        Command::Setup{command} => setup(command),
        Command::Remove {key} => remove(key),
        Command::Add{key, target} => add(key, target),
        Command::List{} => list(),
        Command::Get{key} => get(key)
    };

    if let Err(err_msg) = result {
        eprintln!("{}", err_msg.red());
        return ExitCode::FAILURE
    }
    ExitCode::SUCCESS
}
