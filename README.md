
# Shortcut

Setup shortcuts for your `cd` in your Windows command lines (only CommandPrompt and PowerShell supported for now).

## Code structure

To to avoid circular references there is a module hierarchy:

- `main`
- `config`, `shell`
- `fs`

A module `X` can only depend on code module `Y` if `X` is appears strictly before `Y` in the hierarchy.

