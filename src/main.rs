use clap::{Parser, Subcommand};

mod commands;
mod parser;
mod storage;
mod hash;
mod config;
mod color;


#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmds>,

    /// run chant without initialization (not recomended)
    #[arg(long)]
    hollow: bool,
}

#[derive(Subcommand, Clone)]
enum Cmds {
    /// creates .chant/ directory with .chant/config.toml and .chant/storage.josn files
    Init {},

    /// force chant to scan directory
    Scan {},
    
    /// display all saved comments (default command)
    List {},

    /// print the current config
    Config {},

    /// remove .chant/ directory and remove it from .gitignore
    Dismiss {},

    /// reset confign to the default and run forced scan
    Reset {},
}

// Examples of
// NOTE: some note
// TODO: this is todo
// FIXME: and here is a fixme

fn main() {
    let cli = Cli::parse();
    //if let Some(_) = cli.in_place {
    if  cli.hollow {
        commands::scan::scan_hollow();
        return;
    }
    match cli.cmd {
        Some(Cmds::Init { }) => commands::init::init(),
        Some(Cmds::Scan { }) => commands::scan::scan_force(),
        Some(Cmds::List { }) => commands::list::list(),
        Some(Cmds::Dismiss { }) => commands::dismiss::dismiss(),
        Some(Cmds::Config { }) => commands::config::print_config(),
        Some(Cmds::Reset { }) => commands::general::reset(),
        None => commands::list::list(),
    }
}
