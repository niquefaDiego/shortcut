
use {
    std::{io::ErrorKind, path::Path}
};

/// Ensures the given directory exists, creating as many of the parents directories as needed.
pub fn ensure_dir(directory: &Path) -> Result<(), String> {
    match std::fs::create_dir_all(directory) {
        Ok(_) => Ok(()),
        Err(err) => {
            let msg = format!("Error creating directory \"{}\": {}", directory.display(), err);
            Err(msg.to_string())
        }
    }
}

/// Reads the given file content into a vector of strings.
/// Returns Ok(None) when the file does not exists.
pub fn read_lines(file: &Path) -> Result<Option<Vec<String>>, String> {
    match std::fs::read_to_string(file) {
        Ok(content) => { Ok(Some(content.lines().map(String::from).collect())) },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Ok(None),
            ErrorKind::PermissionDenied => {
                let msg = format!("Permission denied to open file: {}", file.display());
                Err(msg.to_string())
            }
            _ => Err(format!("Error reading file: {}", file.display()).to_string())
        }
    }
}

/// Writes the given vector of lines into the file.
pub fn write_lines(file: &Path, lines: Vec<String>) -> Result<(), String> {
    let file_content = lines.join("\n");
    match std::fs::write(file, file_content) {
        Ok(()) => Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                let msg = format!(
                    "Could not write to file {} because it was not found",
                    file.display()
                );
                return Err(msg.to_string());
            },
            ErrorKind::PermissionDenied => {
                let msg = format!(
                    "Permission denied to write to file {}",
                    file.display()
                );
                return Err(msg.to_string());
            },
            _ => {
                let msg = format!("Error writing to file {}", file.display()
                );
                return Err(msg.to_string());
            }
        }
    }
}
