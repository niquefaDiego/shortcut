use {
    clap::{Parser, Subcommand},
    colored::Colorize,
    std::path::PathBuf,
    std::process::ExitCode,
};

/// Use shortcuts to quickly cd (change directory) to common directories.
///
/// Run one time set-up: $ shortcut setup --command s --path-location C:\Path
///
/// Add shortcuts:       $ shortcut add dl ~/Downloads
///                      or
///                      $ s + dl ~/Downloads
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
        command: String,
        /// Need for Command Prompt setup only. Directory to place a .bat script,
        /// must be part of the PATH environment variable.
        #[arg(short, long)]
        path_location: Option<PathBuf>,
    },
    /// Adds a shortcut.
    /// After one-time setup you can do: $ {command} + <KEY> <TARGET>
    Add {
        /// Shortcut to use go to the target directory.
        key: String,
        /// Absolute or relative path to the target directory.
        target: PathBuf,
    },
    /// Removes a shortcut.
    /// After one-time setup you can do: $ {command} - <KEY>
    Remove {
        /// Shortcut key to remove
        key: String,
    },
    /// Lists all the existing shortcuts.
    /// After one-time setup you can do: $ {command} *
    List {},
    /// Get the target directory given a key, if there is not shortcut for the given key,
    /// then the key will be returned.
    Get {
        /// Key.
        key: String,
    },
}

fn main() -> ExitCode {
    let args = Args::parse();
    let result = match args.command {
        Command::Setup {
            command,
            path_location,
        } => shortcut::setup(command, path_location),
        Command::Remove { key } => shortcut::remove(key),
        Command::Add { key, target } => shortcut::add(key, target),
        Command::List {} => shortcut::list(),
        Command::Get { key } => shortcut::get(key),
    };

    if let Err(err_msg) = result {
        eprintln!("{}", err_msg.red());
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
