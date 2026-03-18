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