# **.\src\commands**

## **== .\src\commands\about.rs ==**

#### *About* [|list()|](about.rs#L8)
This func is displaying all found 'about'\
Not rly useful, you better call `chant about -s` to save output to a file

---

#### *About* [|save()|](about.rs#L59)
This functions is used to save all found 'About' blocks into files\
By default it creates files in every directory, with name "about.md"\
This can be changed by providing different name with flagh -s\
For example: `chant about -s readme.md` -> now all 'About' blocks will be saved to 'readme.md'

---

#### *About* [|remove_old_about_files()|](about.rs#L108)
I need this to rewrite new 'about' blocks\
Maybe i'll add some sort of check, to not rewrite file if nothing was changed in about's. Idk, we'll see\
Now this is fine, ig

---

#### *About* [|create_about_structures()|](about.rs#L145)
This func is used to create hash map dir to 'file content'\
both represented as strings

---

#### *About* [|save_abouts()|](about.rs#L194)
used builded `dir_to_about` hash_map to create new files

---

#### *About* [|connect_links()|](about.rs#L219)
used to replace found objects, with markdown links

---

## **== .\src\commands\config.rs ==**

#### *About* [print_config](config.rs#L5)
what's interesting, config.rs doesn't inclue storage::config\
this func just reads the `.chant/config.toml` file and displays it

---

## **== .\src\commands\dismiss.rs ==**

#### *About* [remove_from_gitignore()](dismiss.rs#L16)
It simply read `.gitignore` file line by line and creating a new buffer, ignoring line with `.chant` in it\
after that I use this buffer to rewrite `.gitignore`\
This may cause some trouble, `.gitignore` could not work, and you have to resave it manuanly

---

## **== .\src\commands\general.rs ==**

#### *About* [|is_gitignore_exists()|](general.rs#L4)
used to check, if `.gitignore` file is in project\
If captures an error - returns `false` by default

---

#### *About* [|nothing_was_found()|](general.rs#L15)
Used to print notification, when scan result or tasks list is empty

---

#### *About* [|init_first()|](general.rs#L22)
Asking to run `chant init`, before using chant\
Hollow chant can be used only for displaying all of the comments (TODO, NOTE and FIXME), but everything else is required an initialization

---

#### *About* [|is_initialized()|](general.rs#L32)
checks if Chant was initialized in the directory\
by default returns false

---

#### *About* [|bad_syntax()|](general.rs#L43)
displays and notificatoin when chant can't parce an args

---

#### *About* [|reset()|](general.rs#L51)
checks if [is_initialized()](general.rs#L32) is true\
used to reset config by calling [create_config()](..\services\config.rs#L31) and [scan_force()](scan.rs#L6)\
Can be useful after some updates, where config structure is changed

---

## **== .\src\commands\scan.rs ==**

#### *About* [|scan_force()|](scan.rs#L6)
This functon scans, without hash checking\
It used when you don't need to check hesh (specifically when `chant init` or `chant reset` is called), or when you need to rescan everything\
Basicaly I just deleting old `.chant/storage` file, and creating a new one. But all tasks and 'about' block are saved

---

#### *About* [scan_hollow()](scan.rs#L51)
This function is kinda similar to [scan_force()](scan.rs#L6), but it doesn't require chant to be initialized (it didn't use `.chant/storage.json` file or config)\
After each file, it prings out found comments. And it uses a default config

---

#### *About* [|scan()|](scan.rs#L124)
This is the functoin that runs every time you trying to get list of comments, task or 'about' blocks\
It have a hash check, so if file wasn't changed, instead of paring it all again, it just returns an old one

---

## **== .\src\commands\task.rs ==**

#### *About* [tasks.rs](task.rs#L5)
So I've wanned to be able to keep tack of my small chanes, and tasks inside of any project, so I added tasks\
It's basically a small 'todo' app inside of 'chant', but it's quite good, and even useful\
Tasks are stored in `.chant/storage.json` file, and now they are local. Maybe later, if I'll add a global config, I'll also implement a globals storage, so I can save all tasks, there. Idrk yet

---
