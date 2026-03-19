use walkdir::WalkDir;
use std::fs;
use hf;

fn init_first() {
    println!("chant wasn't initialised. Run chant init");
}

pub fn init(){
    println!("initialisation...");
    if !is_initialised(){
        let _ = fs::create_dir("./.chant");
        let _ = hf::hide("./.chant");
        let _ = fs::File::create("./.chant/comments.json");
    } else {
        println!("chant is already initialised");
        return;
    }

    add_to_gitignore();

    println!("done!")
}

fn is_initialised() -> bool {
    let ex = fs::exists("./.chant");
    match ex {
        Ok(v)=> return v,
        Err(e)=> {println!("{}", e); return false;}
    }
}

pub fn scan() {
    if !is_initialised(){
        init_first();
        return; 
    }
    println!("scanning...");
    let skip = &["target", ".git", "node_modules"];

    let mut it = WalkDir::new(".").into_iter();
    while let Some(entry) = it.next(){
        match entry {
            Ok(e) => {
                if e.file_type().is_dir() && skip.contains(&e.file_name().to_string_lossy().as_ref()) {
                    it.skip_current_dir();
                    continue;
                }
                if !e.file_type().is_dir() {
                    read_file(&e.path().to_string_lossy());
                }
            },
            Err(e) => println!("{}", e),
        }
    }
}

fn read_file(path: &str){
    let comments = &["TODO", "NOTE", "FIXME"];

    let content = fs::read_to_string(path);
    match content {
        Ok(v) => {
            let lines = v.split("\n");
            for line in lines {
                if comments.iter().any(|&comment| line.contains(comment)) {
                    println!("{}", line)
                }
            }
        },
        Err(e) => {println!("{}", e); return;}
    }
}

pub fn list() {
    if !is_initialised(){
        init_first();
        return; 
    }

    scan();
    // TODO: read .chant/comments.json file next
}

pub fn dismiss() {
    if !is_initialised(){
        init_first();
        return; 
    }
    let _ = fs::remove_dir_all("./.chant");
    remove_from_gitignore();
    println!("dismiss");
}

fn add_to_gitignore() {
    let content = fs::read_to_string(".gitignore");
    match content {
        Ok(v) => {let ignore = format!("{}\n.chant", v); let _ = fs::write(".gitignore", ignore);},
        Err(e) => {println!("{}", e); return}
    }
}

fn remove_from_gitignore() {
    let content = fs::read_to_string(".gitignore");
    match content {
        Ok(v) => {
            let lines = v.split("\n");
            let mut ignore = "".to_owned();
            for line in lines {
                if line != ".chant" {
                    ignore.push_str(line); 
                }
            }
            let _ = fs::write(".gitignore", ignore);
        },
        Err(e) => {println!("{}", e); return}
    }
}