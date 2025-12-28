use {
    crate::fs,
    directories::ProjectDirs,
    std::{
        fmt,
        path::{Path, PathBuf},
        str::FromStr,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigVersion {
    V0,
}

/// Running `$ {command} {key}` will be equivalent to doing `$ cd {value}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShortcutKV {
    /// Key for the shortcut.
    pub key: String,
    /// Absoute path of the target directory.
    pub value: String,
}

/// Data persisted in the config file.
#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    /// Version of config, needed for correct deserialization.
    /// For serialization, the latest version will always be used.
    /// Currently 0.1.0 it the only valid version.
    pub version: ConfigVersion,
    /// Directory in PATH in which the command executable is located.
    pub path_location: String,
    /// Name of the exectuable to do the cd command using the shortcuts.
    pub command: String,
    /// List of all shortcuts.
    pub shortcuts: Vec<ShortcutKV>,

    /// PowerShell profile locations
    pub power_shell_profiles: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigAddResult {
    Created(ShortcutKV),
    Updated(ShortcutKV, ShortcutKV),
    NoChange,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigRemoveResult {
    Removed(ShortcutKV),
    NotFound,
}

const ORGANIZATION: &str = "niquefaDiego";
const APPLICATION: &str = "Shortcuts";
const CONFIG_FILE_NAME: &str = "shortcuts.config";

impl Config {
    pub fn latest() -> ConfigVersion {
        ConfigVersion::V0
    }
}

impl fmt::Display for ConfigVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0.1.0")
    }
}

impl FromStr for ConfigVersion {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0.1.0" => Ok(ConfigVersion::V0),
            _ => {
                let msg = format!("Invalid config version \"{}\".", s);
                Err(msg.to_string())
            }
        }
    }
}

impl Config {
    pub fn serialize(&self) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        ans.reserve_exact(self.shortcuts.len() * 2 + 3);
        ans.push(self.version.to_string());
        ans.push(self.command.clone());
        ans.push(self.path_location.clone());
        ans.push(self.power_shell_profiles.join(";"));
        for shortcut in &self.shortcuts {
            ans.push(shortcut.key.clone());
            ans.push(shortcut.value.clone());
        }
        ans
    }

    pub fn add_power_shell_profile(&mut self, profile: String) {
        if !self.power_shell_profiles.contains(&profile) {
            self.power_shell_profiles.push(profile);
        }
    }

    pub fn deserialize(lines: Vec<String>) -> Result<Self, String> {
        const HEADER_LINES: usize = 4;
        if lines.len() < HEADER_LINES {
            let msg = format!("Config must contain at least {} lines", HEADER_LINES);
            return Err(msg);
        }
        if (lines.len() - HEADER_LINES) % 2 != 0 {
            return Err("Config file has an invalid number of lines".to_string());
        }
        let version = ConfigVersion::from_str(&lines[0])?;
        let command = lines[1].clone();
        let path_location = lines[2].clone();
        let power_shell_profiles: Vec<String> =
            lines[3].clone().split(';').map(|x| x.to_string()).collect();
        let mut shortcuts: Vec<ShortcutKV> = vec![];
        shortcuts.reserve((lines.len() - HEADER_LINES) / 2);
        for i in (HEADER_LINES..lines.len()).step_by(2) {
            shortcuts.push(ShortcutKV {
                key: lines[i].clone(),
                value: lines[i + 1].clone(),
            });
        }
        Ok(Self {
            version,
            path_location,
            command,
            shortcuts,
            power_shell_profiles,
        })
    }

    pub fn add(&mut self, key: String, value: String) -> Result<ConfigAddResult, String> {
        match self.shortcuts.iter().position(|x| x.key == key) {
            Some(position) => {
                if value == self.shortcuts[position].value {
                    return Ok(ConfigAddResult::NoChange);
                }
                let existing = self.shortcuts[position].clone();
                self.shortcuts[position].value = value;
                let updated = self.shortcuts[position].clone();
                return Ok(ConfigAddResult::Updated(existing, updated));
            }
            None => {
                let new_shortcut = ShortcutKV { key, value };
                self.shortcuts.push(new_shortcut.clone());
                self.shortcuts.sort_by(|a, b| a.key.cmp(&b.key));
                Ok(ConfigAddResult::Created(new_shortcut))
            }
        }
    }

    pub fn remove(&mut self, key: String) -> Result<ConfigRemoveResult, String> {
        match self.shortcuts.iter().position(|x| x.key == key) {
            None => Ok(ConfigRemoveResult::NotFound),
            Some(position) => {
                let removed_value = self.shortcuts[position].clone();
                self.shortcuts.remove(position);
                Ok(ConfigRemoveResult::Removed(removed_value))
            }
        }
    }
}

fn get_project_dirs() -> Result<ProjectDirs, String> {
    match ProjectDirs::from("", ORGANIZATION, APPLICATION) {
        Some(proj_dirs) => Ok(proj_dirs),
        None => Err(
            "No valid home directory path could be retrieved from the operating system."
                .to_string(),
        ),
    }
}

fn get_config_file() -> Result<PathBuf, String> {
    let proj_dirs = get_project_dirs()?;
    let dir = proj_dirs.config_local_dir();
    fs::ensure_dir(dir)?;
    Ok(PathBuf::from(dir).join(CONFIG_FILE_NAME))
}

fn read_config(config_file: &Path) -> Result<Option<Config>, String> {
    match fs::read_lines(config_file)? {
        Some(content) => {
            let config = match Config::deserialize(content) {
                Ok(config) => config,
                Err(err) => {
                    let msg = format!(
                        "Corrupted config file: \"{}\".\n{}",
                        config_file.display(),
                        err
                    );
                    return Err(msg.to_string());
                }
            };
            Ok(Some(config))
        }
        None => Ok(None),
    }
}

fn get_config_from_file(config_file: &Path) -> Result<Config, String> {
    match read_config(config_file) {
        Ok(Some(config)) => Ok(config),
        Ok(None) => Err("Config file not found. Run one-time setup (see --help)".to_string()),
        Err(err) => Err(err),
    }
}

pub fn get_config() -> Result<Config, String> {
    let config_file = get_config_file()?;
    get_config_from_file(&config_file)
}

pub fn add_shortcut(key: &str, path: &str) -> Result<(Config, ConfigAddResult), String> {
    let config_file = get_config_file()?;
    let mut config = get_config_from_file(&config_file)?;
    let add_result = config.add(key.to_string(), path.to_string())?;
    if add_result != ConfigAddResult::NoChange {
        let serialized_config = config.serialize();
        fs::write_lines(&config_file, &serialized_config)?;
    }
    Ok((config, add_result))
}

pub fn remove_shortcut(key: &str) -> Result<(Config, ConfigRemoveResult), String> {
    let config_file = get_config_file()?;
    let mut config = get_config_from_file(&config_file)?;
    let remove_result = config.remove(key.to_string())?;
    if remove_result != ConfigRemoveResult::NotFound {
        let serialized_config = config.serialize();
        fs::write_lines(&config_file, &serialized_config)?;
    }
    Ok((config, remove_result))
}

pub fn create_config(
    command: &str,
    path_location: &Path,
    power_shell_profile: Option<PathBuf>,
) -> Result<Config, String> {
    let path_location = fs::to_absolute_path(path_location)?;
    let path_location = path_location.to_string_lossy();
    let power_shell_profile: Option<String> = match power_shell_profile {
        Some(profile) => {
            let profile: PathBuf = fs::to_absolute_path(&profile)?;
            Some(profile.to_string_lossy().to_string())
        }
        None => None,
    };
    let config_file = get_config_file()?;
    let config = match read_config(&config_file)? {
        Some(config) => {
            let mut config = config;
            config.command = command.to_string();
            power_shell_profile.map(|p| config.add_power_shell_profile(p));
            config
        }
        None => Config {
            version: Config::latest(),
            path_location: path_location.to_string(),
            power_shell_profiles: power_shell_profile.into_iter().collect(),
            command: command.to_string(),
            shortcuts: vec![],
        },
    };
    let serialized_config = config.serialize();
    fs::write_lines(&config_file, &serialized_config)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let c0 = Config {
            version: ConfigVersion::V0,
            path_location: "C:\\Path".to_string(),
            power_shell_profiles: vec![
                "C:\\Users\\foo\\OneDrive\\Documents\\WindowsPowerShell\\Microsoft.PowerShell_profile.ps1".to_string(),
                "C:\\Users\\foo\\OneDrive\\Documents\\WindowsPowerShell\\Microsoft.PowerShellISE_profile.ps1".to_string()
            ],
            command: "cd2".to_string(),
            shortcuts: vec![],
        };
        let c0_serialized = c0.serialize();
        let c0_deserialized =
            Config::deserialize(c0_serialized).expect("Deserialization should work");
        assert_eq!(c0, c0_deserialized);

        let c1 = Config {
            version: ConfigVersion::V0,
            path_location: "C:\\Path".to_string(),
            power_shell_profiles: vec![],
            command: "changedir".to_string(),
            shortcuts: vec![
                ShortcutKV {
                    key: "dls".to_string(),
                    value: "C:\\Users\\user\\Downloads".to_string(),
                },
                ShortcutKV {
                    key: "x84".to_string(),
                    value: "C:\\Program Files (x84)".to_string(),
                },
            ],
        };
        let c1_serialized = c1.serialize();
        let c1_deserialized =
            Config::deserialize(c1_serialized).expect("Deserialization should work");
        assert_eq!(c1, c1_deserialized);
    }
}
