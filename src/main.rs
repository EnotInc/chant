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

    /// display only TODO comments
    #[arg(long, short)]
    todo: bool,

    /// display only NOTE comments
    #[arg(long, short)]
    note: bool,

    /// display only FIXME comments
    #[arg(long, short)]
    fixme: bool,
}

#[derive(Subcommand, Clone)]
enum Cmds {
    /// creates .chant/ directory with .chant/config.toml and .chant/storage.josn files
    Init {},

    /// force chant to scan directory without hash checking
    Scan {},
    
    /// display all saved comments (default command)
    List {
        /// display both comments and tasks
        #[arg(long)]
        all: bool,

        /// display only TODO comments
        #[arg(long, short)]
        todo: bool,

        /// display only NOTE comments
        #[arg(long, short)]
        note: bool,

        /// display only FIXME comments
        #[arg(long, short)]
        fixme: bool,
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
    if cli.todo || cli.note || cli.todo || cli.hollow {
        if cli.hollow {
            commands::scan::scan_hollow(cli.todo, cli.note, cli.fixme);
            return;
        } else {
            commands::list::list(cli.todo, cli.note, cli.fixme);
            return
        }
    }
    match cli.cmd {
        Some(Cmds::Init { }) => commands::init::init(),
        Some(Cmds::Scan { }) => commands::scan::scan_force(),
        Some(Cmds::List { all, todo, note, fixme}) => {
            commands::list::list(todo, note, fixme);
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
        None => commands::list::list(true, true, true),
    }
}
