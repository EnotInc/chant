use std::env;

mod commands;
mod parser;
mod comments;
mod hash;
mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let cmd: &str = &args[1];
        match cmd  {
           "init" => commands::init(),
           "scan" => commands::scan(),
           "dismiss" => commands::dismiss(),
           _ => println!("unknown command")
       }
    } else if args.len() == 1 {
        commands::list();
    } else {
        println!("bad syntax")
    }
}
