# Second Brain

Program to create similar functionality that Obsidian provides for opening and creating daily notes.

This is meant to be used in tandem with Obsidian, but offer an ergonomic entry point to quickly make notes with a terminal editor, and ease some of the friction that comes from using a terminal editor as part of an Obsidian practice.

## 🧠 Features

- run `second-brain` or `sb` to open up a vault (defaults to opening up the daily note)
- Configurable via `.toml`
- Extra commands to pull up other periodic notes (daily, weekly, monthly, yearly)
- Checks for a note on startup and writes a new one if none matches the configured pattern

## 🗓️ Planned Features

- [ ] configure via cli on first run
- [ ] command to reset/remove configuration
- [ ] use templates to write new notes
- [ ] use date time string interpolation in templates

## 📝 Configuration

Second brain uses the `$EDITOR` environment variable. If unset, it will attempt to open `neovim`.

Only the vault configuration is required. If any of the optional configurations are unset, `second-brain` will default to writing and opening everything to the vault's root directory.

| Config     | Description                        | Example            |
| ---------- | ---------------------------------- | ------------------ |
| vault      | map to an obsidian vault directory | `{dir = /vaults}`  |
| periodical | map of a configured time period    | `[periodical.day]` |
| dir        | directory in relation to the vault | `day`              |
| fmt        | date time format for a periodical  | `%Y-%m-%d`         |
| template   | template file relative to vault    | `templates/day.md` |

Example config

```toml
[vault]
# it's recommended to use absolute paths to allow the program
# to properly launch the correct file.
dir = "/vaults"

[periodical.day]
dir = "day"
fmt = "%Y-%m-%d"
```
