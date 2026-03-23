use std::env;

mod commands;
mod parser;
mod storage;
mod hash;
mod config;
mod color;

// Examples of
// NOTE: some note
// TODO: this is todo
// FIXME: and here is a fixme

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let cmd: &str = &args[1];
        // TODO: add command 'read'/'open' to open in code editor (figure out how)
        match cmd  {
            "init" => commands::init::init(),
            "scan" => commands::scan::scan_force(),
            "list" => commands::list::list(),
            "dismiss" => commands::dismiss::dismiss(),
            "config" => commands::config::print_config(),
            "help" => commands::general::help(),
            "reset" => commands::general::reset(),
            _ => commands::general::unknown_command(),
        }
    } else if args.len() == 1 {
        commands::list::list();
    } else {
        commands::general::bad_syntax();
    }
}
