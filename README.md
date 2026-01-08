
# Shortcut

Setup shortcuts for your `cd` in you terminal:

Supported shells:
- Bash: `bash`
- Command Prompt: `1cmd.exe`
- PowerShell: `pwsh.exe`
- Windows PowerShell: `powershell.exe`

Support for Bourne Shell (`sh`) comming soon :)

## Getting started


Install:
```
$ cargo install --path .
```

Do one-time setup, you need to specify the name of the command you will use to change directory and a directory that is part of the `PATH` environment variable:

```
$ shortcut setup --command s --path-location C:\Path
```

You can add some shortcuts to frequent directories (Note `s` is the command you specified above, you can choose a different name):
```
$ s + dl ~/Downloads
$ s + repo "C:\Code\GitHub repositories"
```

Now instead of `$ cd "C:\Code\GitHub repositories"`, you can do:

```
$ s repo
```

You can go back to the previous directory by doing:
```
$ s -
```

If you no longer need the shortcut to `~/Downloads` you can remove it like this:
```
$ s - dl
```

To see a list of all your shortcuts you can do:
```
$ s *
```

## Code structure

To to avoid circular references there is a module hierarchy:

```
main -> lib -> shell -> config -> fs
```

A module `X` can only depend on code module `Y` if `X` is appears strictly before `Y` in the hierarchy.
