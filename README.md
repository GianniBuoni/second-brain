# Second Brain

Program to create similar functionality that Obsidian provides for opening and creating daily notes. Meant to be used in tandem with Obsidian, but offer an ergonomic entry point to quickly make notes with a terminal editor.

## 🧠 Features

- Defaults to opening up the daily note
- Simple to use and fully configurable

## Goals

- [ ] configure vault locations via file or cli
- [ ] command `second-brain` or `sb` to open up a vault
- [ ] command to reset/remove configuration
- [ ] check for a daily note on startup and write one if none matches the pattern

## Environment Variables

- For the configuration file, `second-brain` looks for a `$SECOND_BRAIN_CONFIG` environment varaible. Otherwise, it uses yours system's default configuration directory.
- `second-brain` uses the `$EDITOR` environment variable. If unset, it will attempt to run `neovim`.

## Configuration

| Config     | Description                                  | Default        |
| ---------- | -------------------------------------------- | -------------- |
| vault-path | Path to the vault second brain is targenting | `~/vaults`     |
| daily-dir  | Directory of the daily notes                 | The vault root |
| daily-fmt  | The date-time format for the daily note name | `YYYY-MM-DD`   |
