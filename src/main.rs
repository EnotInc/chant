use std::env;

mod commands;
mod parser;
mod comments;
mod hash;
mod config;
mod color;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let cmd: &str = &args[1];
        // TODO: add command 'read'/'open' to open in code editor (figure out how)
        match cmd  {
           "init" => commands::init(),
           "scan" => commands::scan_force(),
           "list" => commands::list(),
           "dismiss" => commands::dismiss(),
           "config" => commands::print_config(),
           _ => commands::unknown_command(),
       }
    } else if args.len() == 1 {
        commands::list();
    } else {
        commands::bad_syntax();
    }
}
