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
    List {
        /// display all saved comments and tasks
        #[arg(long)]
        all: bool
    },

    /// print the current config
    Config {},

    /// remove .chant/ directory and remove it from .gitignore
    Dismiss {},

    /// reset confign to the default and run forced scan
    Reset {},

    /// Global tasks in the project
    Task {
        #[command(subcommand)]
        option: Option<TaskOpt>,
    }
}

#[derive(Subcommand, Clone)]
enum TaskOpt {
    /// add new task with "message"
    Add {
        message: String
    },
    /// mark task as "done" by id
    Done {
        id: String
    },
    /// remove task
    Remove {
        #[arg(long)]
        /// remove all of the tasks
        all: bool,

        /// remove one task by id
        id: Option<String>,
    },
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
        Some(Cmds::List { all }) => {
            commands::list::list();
            if all {
                commands::list::list_all();
            }
        },
        Some(Cmds::Dismiss { }) => commands::dismiss::dismiss(),
        Some(Cmds::Config { }) => commands::config::print_config(),
        Some(Cmds::Reset { }) => commands::general::reset(),
        Some(Cmds::Task { option }) => {
            match option {
                Some(TaskOpt::Add { message }) => { commands::task::add_task(message) }
                Some(TaskOpt::Done { id }) => { commands::task::done_task(id); }
                Some(TaskOpt::Remove { all, id }) => {
                    if all {
                        commands::task::remove_all();
                    } else {
                        match id {
                            Some(v) => commands::task::remove_task(v),
                            None => commands::general::bad_syntax(),
                        }
                    }
                }
                None => { commands::task::print_tasks(); }
            }
        },
        None => commands::list::list(),
    }
}
