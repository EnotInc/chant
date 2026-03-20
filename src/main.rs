use std::env;

mod commands;
mod parser;
mod comments;
mod hash;
mod config;
mod color;

fn main() {
    // TODO: add flags, for example --TODO -t and other
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let cmd: &str = &args[1];
        // TODO: add commands 'restart' (dismiss + init) and 'read' to open in code editor (figure out how)
        match cmd  {
           "init" => commands::init(),
           "scan" => commands::scan_force(),
           "dismiss" => commands::dismiss(),
           _ => commands::unknown_command(),
       }
    } else if args.len() == 1 {
        commands::list();
    } else {
        commands::bad_syntax();
    }
}
