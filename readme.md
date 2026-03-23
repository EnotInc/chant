# chant

## About
Chant is a little cli tool to work with TODO, NOTE and FIXME comments.
Comment search is optimized with hash checking

## Usage

### Example

![chant_usage_example](.github/chant_usage.png)

### Commands

- `chant init` to initialize chant. It creates `.chant/` directory and adds it to the `.gitignore` file
- `chant` or `chant list` - display all found comments 
- `chant scan` - forced scan without hash checking
- `chant help` - list all available commands
- `chant config` - display current config
- `chant dismiss` removes `.chant/` dir and removes it from `.gitignore` file

### Config

Now in `.chant/config.toml` chant stores 2 lists:
1. list of supported files (extetions)
2. list of ignored directories

You can modify both of this, but be careful, chant **doesn't** supports languages, where comments are not started with `//`. I'll add this later

## Installation

```bash
git clone https://github.com/EnotInc/chant.git
cd chant
cargo install --path .
```
