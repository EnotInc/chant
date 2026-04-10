# **.\src\services**

## **== .\src\services\color.rs ==**

#### *About* [colors.rs](color.rs#L1)
just a small file with [Color](color.rs#L4) enum, and pub fn [paint_str()](color.rs#L29), wich adds ascii escape sequence to change the color of given line

---

#### *About* [|Color|](color.rs#L4)
Simple emun, used to work with colors in terminal

---

#### *About* [|ascii_color()|](color.rs#L15)
used to convert given [Color](color.rs#L4) to the ascii escape sequence of then color

---

#### *About* [|paint_str()|](color.rs#L29)
Gets string, [Color](color.rs#L4), and using [ascii_color()](color.rs#L15) paint the string\
At the end added reset ascii escape sequence

---

## **== .\src\services\config.rs ==**

#### *About* [|Config|](config.rs#L5)
Main struct for config\
Includes [Scanner](config.rs#L15) and [About](..\storage.rs#L21)

---

#### *About* [|Scanner|](config.rs#L15)
`read` - vector of strings, stores a list for supperted file extatoins (without dot '.')\
`ignore` - vector of string with ignored files / directories, such as `target/`, `.chant/`, `.git/` and so on

---

#### *About* [|About|](config.rs#L24)
`output` - name of the file with 'about' blocks. "about.md" by default

---

#### *About* [|create_config()|](config.rs#L31)
used to (re)write a config file in `.chant/config.toml` file\
called [new_config()](config.rs#L52) to get default [Config](config.rs#L5)

---

#### *About* [|new_config()|](config.rs#L52)
creates a new default [Config](config.rs#L5)

---

## **== .\src\services\hash.rs ==**

#### *About* [hash.rs](hash.rs#L3)
contains little func get_hash, witch takes a &String as an arg and returns hash of this line as u64

---

## **== .\src\services\link.rs ==**

#### *About* [|create_obj_link()|](link.rs#L4)
Small helper func, to create markdown link

---
