use crate::fs;
use std::path::{Path, PathBuf};
use directories::ProjectDirs;

#[derive(Debug, PartialEq, Eq)]
pub struct ShortcutKV {
    pub key: String,
    pub value: String
}

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub command: String,
    pub shortcuts: Vec<ShortcutKV>
}

impl Config {
    pub fn serialize(&self) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        ans.reserve_exact(self.shortcuts.len()*2+1);
        ans.push(self.command.clone());
        for shortcut in &self.shortcuts {
            ans.push(shortcut.key.clone());
            ans.push(shortcut.value.clone());
        }
        ans
    }

    pub fn deserialize(lines: Vec<String>) -> Result<Self, String> {
        if lines.len() == 0 { return Err("Empty config file".to_string()); }
        if lines.len()%2 == 0 {
            return Err("Config file must have an odd number of lines".to_string());
        }
        let command = lines[0].clone();
        let mut shortcuts: Vec<ShortcutKV> = vec![];
        shortcuts.reserve((lines.len()-1)/2);
        for i in (1..lines.len()).step_by(2) {
            shortcuts.push(ShortcutKV{
                key: lines[i].clone(),
                value: lines[i+1].clone()
            });
        }
        Ok(Self{
            command,
            shortcuts
        })
    }

    pub fn add(&mut self, key: String, value: String) -> Result<(), String> {
        if self.shortcuts.iter().any(|x| x.key == key) {
            return Err(format!("Key \"{}\" is already present in shortcuts", key).to_string());
        }
        self.shortcuts.push(ShortcutKV{key, value});
        self.shortcuts.sort_by(|a, b| a.key.cmp(&b.key));
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<String, String> {
        match self.shortcuts.iter().position(|x| x.key == key) {
            None => Err(format!("Key \"{}\" is not present in shortcuts", key).to_string()),
            Some(position) => {
                let value = self.shortcuts[position].value.clone();
                self.shortcuts.remove(position);
                Ok(value)
            }
        }
    }
}

fn get_project_dirs() -> Result<ProjectDirs, String> {
    match ProjectDirs::from("", "niquefaDiego", "Shortcuts") {
        Some(proj_dirs) => Ok(proj_dirs),
        None => Err(
            "No valid home directory path could be retrieved from the operating system."
            .to_string())
    }
}

fn get_config_file() -> Result<PathBuf, String> {
    let proj_dirs = get_project_dirs()?;
    let dir = proj_dirs.config_local_dir();
    fs::ensure_dir(dir)?;
    Ok(PathBuf::from(dir).join("shortcuts.txt"))
}

fn read_config(config_file: &Path) -> Result<Option<Config>, String> {
    match fs::read_lines(config_file)? {
        Some(content) => {
            let config = Config::deserialize(content)?;
            Ok(Some(config))
        }
        None => Ok(None) 
    }
}

fn get_config_from_file(config_file: &Path) -> Result<Config, String>{ 
    match read_config(config_file) {
        Ok(Some(config)) => Ok(config),
        Ok(None) => Err("Config file not found. Run one-time setup (see --help)".to_string()),
        Err(err) => Err(err)
    }
}

pub fn get_config() -> Result<Config, String> {
    let config_file = get_config_file()?;
    get_config_from_file(&config_file)
}

pub fn add_shortcut(key: String, value: String) -> Result<(), String> {
    let config_file = get_config_file()?;
    let mut config = get_config_from_file(&config_file)?;
    config.add(key.clone(), value.clone())?;
    let serialized_config = config.serialize();
    fs::write_lines(&config_file, serialized_config)?;
    println!("Successfully added shortcut {} -> {}", key, value);
    Ok(())
}

pub fn remove_shortcut(key: String) -> Result<(), String> {
    let config_file = get_config_file()?;
    let mut config = get_config_from_file(&config_file)?;
    let value = config.remove(key.clone())?;
    let serialized_config = config.serialize();
    fs::write_lines(&config_file, serialized_config)?;
    println!("Successfully removed shortcut {} -> {}", key, value);
    Ok(())
}

pub fn create_config(command: &str) -> Result<(), String> {
    let config_file = get_config_file()?;
    let config = match read_config(&config_file)? {
        Some(config) => Config { command: command.to_string(), ..config },
        None => Config { command: command.to_string(), shortcuts: vec![] }
    };
    let serialized_config = config.serialize();
    fs::write_lines(&config_file, serialized_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let c0 = Config {
            command: "cd2".to_string(),
            shortcuts: vec![]
        };
        let c0_serialized = c0.serialize();
        let c0_deserialized = Config::deserialize(c0_serialized)
            .expect("Deserialization should work");
        assert_eq!(c0, c0_deserialized);

        let c1 = Config {
            command: "changedir".to_string(),
            shortcuts: vec![
                ShortcutKV {
                    key: "dls".to_string(),
                    value: "C:\\Users\\user\\Downloads".to_string()
                },
                ShortcutKV {
                    key: "x84".to_string(),
                    value: "C:\\Program Files (x84)".to_string()
                }
            ]
        };
        let c1_serialized = c1.serialize();
        let c1_deserialized = Config::deserialize(c1_serialized)
            .expect("Deserialization should work");
        assert_eq!(c1, c1_deserialized);
    }
}
