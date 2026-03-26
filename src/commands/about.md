
# == .\src\commands\about.rs ==

### *About* [list()](about.rs#L5)
This func is displaying all found 'about'\
Not rly useful, you better call `chant about -s` to save output to a file

---

### *About* [save(output: String)](about.rs#L55)
This functions is used to save all found 'About' blocks into files\
By default it creates files in every directory, with name "about.md"\
This can be changed by providing different name with flagh -s\
For example: `chant about -s readme.md` -> now all 'About' blocks will be saved to 'readme.md'

---

### *About* [remove_old_about_files](about.rs#L95)
I need this to rewrite new 'about' blocks\
Maybe i'll add some sort of check, to not rewrite file if nothing was changed in about's. Idk, we'll see\
Now this is fine, ig

---

### *About* [create_about_structures](about.rs#L133)
This func is used to create hash map 'dir' to 'file content'\
both represented as strings

---

### *About* [create_topic_link()](about.rs#L171)
just a small helper func, to create markdown link to each 'about' blocks in the source code

---

### *About* [save_abouts()](about.rs#L179)
used builded `dir_to_about` hash_map to create new files

---

# == .\src\commands\config.rs ==

### *About* [print_config](config.rs#L5)
what's interesting, config.rs doesn't inclue storage::config\
this func just reads the `.chant/config.toml` file and displays it

---

# == .\src\commands\dismiss.rs ==

### *About* [remove_from_gitignore()](dismiss.rs#L16)
It simply read `.gitignore` file line by line and creating a new buffer, ignoring line with `.chant` in it\
after that I use this buffer to rewrite `.gitignore`\
This may cause some trouble, `.gitignore` could not work, and you have to resave it manuanly

---

# == .\src\commands\general.rs ==

### *About* [general.rs](general.rs#L5)
I don't rly know why I keep it here, and not spread all of those functons across the services\
But here is a list some useful functoins:\
[is_gitignore_exists] - checking for `.gitignore` file existance\
[nothing_was_found] - simple notificatoin. Used when scaner function didn't found anything\
[init_first] - also notificaton, showed when chant wasn't initializes, but called feature required it\
[is_initialized] - checking if `.chant/` directory is exists\
[bad_syntax] - used to show used that he fked up\
[reset] - resets the config, and runs forced scan

---

# == .\src\commands\scan.rs ==

### *About* [scan_force](scan.rs#L6)
This functon scans, without hash checking\
It used when you don't need to check hesh (specifically when `chant init` or `chant reset` is called), or when you need to rescan every thing\
Basicaly I just deleting old `.chant/storage` file, and creating a new one. But all tasks and 'about' block are saved

---

### *About* [scan_hollow()](scan.rs#L51)
This function is kinda similar to [scan_force], but it doesn't require chant to be initialized (it didn't use `.chant/storage.json` file or config)\
After each file, it prings out found comments. And it uses a default config

---

### *About* [scan()](scan.rs#L119)
This is the functoin that runs every time you trying to get list of comments, task or 'about' blocks\
It have a hash check, so if file wasn't changed, instead of paring it all again, it just returns an old one

---

# == .\src\commands\task.rs ==

### *About* [tasks.rs](task.rs#L5)
So I've wanned to be able to keep tack of my small chanes, and tasks inside of any project, so I added tasks\
It's basically a small 'todo' app inside of 'chant', but it's quite good, and even useful\
Tasks are stored in `.chant/storage.json` file, and now they are local. Maybe later, if I'll add a global config, I'll also implement a globals storage, so I can save all tasks, there. Idrk yet

---
