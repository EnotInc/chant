# chant

## About
Chant is a little cli tool to work with comments and generate documentation

## Usage

### Example

![chant_usage_example](.github/chant_usage.png)

### Commands

Main commands:
- `chant init` to initialize chant. It creates `.chant/` directory and adds it to the `.gitignore` file
- `chant` or `chant list` - display all found comments
- `chant scan` - forced scan without hash checking
- `chant dismiss` - removes `.chant/` dir and removes it from `.gitignore` file
- `chant stave` - tack your project. Use `-t` flag to add a new one, or `-r` to remove. `chant init` automatically adds project to the stave

You can also run Chant without initializatoin with `chant --hollow` command. This will not create `.chant/` directory, so not all functions will be available

You can run `--help` with any command for more informaton

### Task

Tasks were appeared in v0.2.0. Essentially, it's a simple 'todo' app inside Chant
- `chant task` - list of tasks
- `chant task add/new <message>` - add new task with message
- `chant task edit <id>` - change task message
- `chant task done <id>` - mark task as complete
- `chant task remove <id>` remove one task from storage

### Config

Now in `~/.chant/config.toml` chant store:
1. [scanner]:
    1. list of supported files (extentions)
    2. list of ignored directories

You can modify both of this, but be careful, chant **doesn't** supports languages, where comments are not started with `//`. I'll add this later

### Useful flags
- `chant` with `-t --todo`, `-n --note`, `-f --fixme` - show specific comments (works with `--hollow` flag) 
- `chant [list]` with `-b --both` - show both comments and tasks
- `chant remove --done` - remove every complete task
- `chant remove --all` - remove all tasks

## Installation

```bash
git clone https://github.com/EnotInc/chant.git
cd chant
cargo install --path . # --force
```
