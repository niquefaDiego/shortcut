use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

/// Ensures the given directory exists, creating as many of the parents directories as needed.
pub fn ensure_dir(directory: &Path) -> Result<(), String> {
    match std::fs::create_dir_all(directory) {
        Ok(_) => Ok(()),
        Err(err) => {
            let msg = format!(
                "Error creating directory \"{}\": {}",
                directory.display(),
                err
            );
            Err(msg.to_string())
        }
    }
}

/// Ensures the given directory exists, creating as many of the parents directories as needed.
pub fn ensure_file_parent_dir(file: &Path) -> Result<(), String> {
    match file.parent() {
        Some(dir) => ensure_dir(dir),
        None => Ok(()),
    }
}

/// Reads the given file content into a vector of strings.
/// Returns Ok(None) when the file does not exists.
pub fn read_lines(file: &Path) -> Result<Option<Vec<String>>, String> {
    match std::fs::read_to_string(file) {
        Ok(content) => Ok(Some(content.lines().map(String::from).collect())),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Ok(None),
            ErrorKind::PermissionDenied => {
                let msg = format!("Permission denied to open file: {}", file.display());
                Err(msg.to_string())
            }
            _ => Err(format!("Error reading file: {}", file.display()).to_string()),
        },
    }
}

/// Writes the given content to the file, overwritting existing file content.
pub fn write_str(file: &Path, content: &str) -> Result<(), String> {
    match std::fs::write(file, content) {
        Ok(()) => {
            println!("Updated file \"{}\"", file.display());
            Ok(())
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                let msg = format!(
                    "Could not write to file {} because it was not found",
                    file.display()
                );
                return Err(msg.to_string());
            }
            ErrorKind::PermissionDenied => {
                let msg = format!("Permission denied to write to file {}", file.display());
                return Err(msg.to_string());
            }
            _ => {
                let msg = format!("Error writing to file {}", file.display());
                return Err(msg.to_string());
            }
        },
    }
}

/// Writes the given vector of lines into the file.
pub fn write_lines(file: &Path, lines: &Vec<String>) -> Result<(), String> {
    let file_content = lines.join("\n");
    write_str(file, &file_content)
}

/// Converts a path to an absolute path, replacing the staring '~' component with the home directory
/// if needed.
pub fn to_absolute_path(path: &Path) -> Result<PathBuf, String> {
    // handle paths starting with ~, replace the ~ component with the home directory
    if let Some(first_component) = path.components().next()
        && first_component.as_os_str() == "~"
    {
        match directories::UserDirs::new() {
            None => {
                println!(
                    "Could not resolve absolute path home directory. \
                    Adding shortcut to relative path \"{}\" instead",
                    path.display()
                );
                return Ok(PathBuf::from(path));
            }
            Some(users_dir) => {
                let home = users_dir.home_dir();
                let path_without_tilde: PathBuf = path.components().skip(1).collect();
                let resolved_path = home.join(path_without_tilde);
                return to_absolute_path_internal(&resolved_path);
            }
        }
    }
    to_absolute_path_internal(path)
}

fn to_absolute_path_internal(path: &Path) -> Result<PathBuf, String> {
    match std::path::absolute(&path) {
        Ok(path) => Ok(path),
        Err(err) => {
            let msg = format!("Error parsing path \"{}\": {}", path.display(), err);
            return Err(msg);
        }
    }
}
