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
}

pub fn list() {
    if !is_initialised(){
        init_first();
        return; 
    }

    scan();
    println!("some data");
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

fn add_to_gitignore() {}

fn remove_from_gitignore() {}