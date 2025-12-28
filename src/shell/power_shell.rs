use {
    super::copy_strs as copy,
    super::{PowerShell, Shell},
    crate::{config::Config, fs},
    std::path::PathBuf,
    which::which,
};

impl Shell for PowerShell {
    fn name(&self) -> &'static str {
        "PowerShell"
    }

    fn available(&self) -> Result<bool, String> {
        match which("PowerShell") {
            Ok(location) => {
                println!("{} found at {}", self.name(), location.display());
                return Ok(true);
            }
            Err(_) => Ok(false),
        }
    }
    fn setup(&self, config: &Config) -> Result<(), String> {
        let function = get_power_shell_function(&config);
        for profile in &config.power_shell_profiles {
            println!("Setting up {} with profile {}", self.name(), profile);
            setup_power_shell_profile(&config, &profile, &function)?;
        }
        Ok(())
    }
}

fn setup_power_shell_profile(
    config: &Config,
    profile: &str,
    function: &Vec<String>,
) -> Result<(), String> {
    let profile = PathBuf::from(profile);
    let existing_profile_content = fs::read_lines(&profile)?;
    dbg!(&config.command);
    dbg!(&existing_profile_content);
    match existing_profile_content {
        None => {
            fs::ensure_file_parent_dir(&profile)?;
            fs::write_lines(&profile, function)?;
        }
        Some(_) => {
            todo!("Need to replace function with new one");
        }
    }
    Ok(())
}

fn get_power_shell_function(config: &Config) -> Vec<String> {
    let mut x = vec![];
    x.push(r#"# ---------- shortcut start ----------"#.to_string());
    x.push(format!("# {}", copy::HEADER_COMMENT_1).to_string());
    x.push(format!("# {}", copy::HEADER_COMMENT_2).to_string());
    x.push(r#"function s {"#.to_string());
    x.push(r#"    param ("#.to_string());
    x.push(r#"        [Parameter(Position = 0, Mandatory = $true)] [string]$p1,"#.to_string());
    x.push(r#"        [Parameter(Position = 1)] [string]$p2,"#.to_string());
    x.push(r#"        [Parameter(Position = 2)] [string]$p3"#.to_string());
    x.push(r#"    )"#.to_string());
    //
    // {command} + <KEY> <TARGET>
    x.push(r#"    if ($p1 -eq "+") {"#.to_string());
    x.push(r#"        shortcut list"#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"        Write-Output "[PS] Running: shortcut add `"$p2`"""#.to_string());
    x.push(r#"        shortcut add $key"#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"        Write-Output "[PS] shortcut add finished""#.to_string());

    // {command} - <KEY>
    x.push(r#"    } elseif ($p1 -eq "-") {"#.to_string());
    x.push(r#"        if ($p2) {"#.to_string());
    #[cfg(debug_assertions)]
    x.push(
        r#"            Write-Output "[PS] Running: shortcut remove `"$p2`" `"$p3`"""#.to_string(),
    );
    x.push(r#"            shortcut remove $key $Value"#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"            Write-Output "[PS] shortcut remove finished""#.to_string());

    // {command} -
    x.push(r#"        } else {"#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"            Write-Output "[PS] Running: Pop-Location""#.to_string());
    x.push(r#"            Pop-Location"#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"            Write-Output "[PS] Pop-Location finished""#.to_string());
    x.push(r#"        }"#.to_string());

    // {command} *
    x.push(r#"    } elseif ($p1 -eq "*") {"#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"        Write-Output "[PS] Running: shortcut list""#.to_string());
    x.push(r#"        shortcut list"#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"        Write-Output "[PS] shortcut list finished""#.to_string());

    // {comand} <KEY>
    for shortcut in &config.shortcuts {
        x.push(format!(r#"    }} elseif ($p1 -eq "{}") {{"#, shortcut.key).to_string());
        #[cfg(debug_assertions)]
        x.push(
            format!(
                r#"        Write-Output "[PS] Using shortcut: `"${}`"""#,
                shortcut.key
            )
            .to_string(),
        );
        x.push(format!(r#"        Push-Location "{}""#, shortcut.value).to_string());
    }
    //
    // If nothing above matches, do a normal CD with the /D flag to automatically change disk
    x.push(r#"    } else {"#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"        Write-Output "[PS] Running: Push-Location `"$p1`"""#.to_string());
    x.push(r#"        Push-Location "$p1""#.to_string());
    #[cfg(debug_assertions)]
    x.push(r#"        Write-Output "[PS] Push-Location finished""#.to_string());
    x.push(r#"    }"#.to_string());
    x.push(r#"}"#.to_string());
    x.push(r#"# ---------- shortcut end ----------"#.to_string());
    x
}
