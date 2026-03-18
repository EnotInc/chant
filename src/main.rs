use std::env;
use std::fs;
use hf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let cmd: &str = &args[1];
        match cmd  {
           "init" => init(),
           "scan" => scan(),
           "dismiss" => dismiss(),
           _ => println!("printing...")
       }
    } else if args.len() == 1{
        println!("empty")
    } else {
        is_initialised();
        println!("bad syntax")
    }
}

fn init(){
    println!("initialisation...");
    let _ = fs::create_dir("./.chant");
    let _ = hf::hide("./.chant");
    let _ = fs::File::create("./.chant/foo.json");

    add_to_gitignore();

    println!("done!")
}

fn is_initialised() -> bool {
    return false;
}

fn scan() {
    is_initialised();
    println!("scanning...");
}

fn dismiss() {
    is_initialised();
    remove_from_gitignore();
    println!("dismiss");
}

fn add_to_gitignore(){}

fn remove_from_gitignore(){}