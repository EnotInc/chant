use walkdir::WalkDir;
use std::fs;
use hf;

use crate::{comments::{self, new_storage}, parser};

// TODO: rewrite notificatoins

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

    let mut storage = comments::new_storage();

    let mut it = WalkDir::new(".").into_iter();
    while let Some(entry) = it.next(){
        match entry {
            Ok(e) => {
                let file_name = e.file_name().to_string_lossy();
                if e.file_type().is_dir() && skip.contains(&file_name.as_ref()) {
                    it.skip_current_dir();
                    continue;
                }
                if !e.file_type().is_dir() {
                    let comments=  parser::parse_file(&e.path().to_string_lossy());
                    for c in comments {
                        let not_hash = format!("{}:{}",c.line, c.index);
                        storage.comments.insert(not_hash, c);
                    }
                }
            },
            Err(e) => println!("{}", e),
        }
    }

    save_storage(&storage);
}

pub fn list() {
    if !is_initialised(){
        init_first();
        return; 
    }

    scan();
    // TODO: read .chant/comments.json file next
    let s = load_storage();
    for comment in s.comments {
        println!("{} {}:{} - {}", comment.1.kind, comment.1.file, comment.1.index, comment.1.line);
    }
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

fn save_storage(storage: &comments::Storage) {
    let json = serde_json::to_string(storage);
    match json {
        Ok(v)=>{let _ = fs::write("./.chant/comments.json", v);},
        Err(e)=>{println!("{}", e)}
    }
}

fn load_storage() -> comments::Storage{
    let content = fs::read_to_string("./.chant/comments.json");
    match content {
        Ok(v) =>{
            let res: Result<comments::Storage, serde_json::Error >= serde_json::from_str(&v);
            match res {
                Ok(sorage) => {
                    return sorage;
                },
                Err(err)=>{println!("{}", err); return new_storage();}
            }
        },
        Err(e)=>{println!("{}", e); return new_storage()}
    }
}