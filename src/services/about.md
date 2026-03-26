# **.\src\services**

## **== .\src\services\color.rs ==**

#### *About* [colors.rs](color.rs#L1)
just a small file with [Color] enum, and pub fn [paint_str], wich adds ascii escape sequence to change the color of given line

---

## **== .\src\services\config.rs ==**

#### *About* [Config](config.rs#L5)
Main struct for config\
Includes [Scanner] and [About]

---

#### *About* [Scanner](config.rs#L15)
`read` - vector of strings, stores a list for supperted file extatoins (without dot '.')\
`ignore` - vector of string with ignored files / directories, such as `target/`, `.chant/`, `.git/` and so on

---

#### *About* [About](config.rs#L24)
`output` - name of the file with 'about' blocks. "about.md" by default

---

## **== .\src\services\hash.rs ==**

#### *About* [hash.rs](hash.rs#L3)
contains little func [get_hash], witch takes a &String as an arg and returns hash of this line as u64

---
