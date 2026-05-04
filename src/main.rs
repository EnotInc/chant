use clap::{Parser, Subcommand};

mod commands;
mod services;
mod parser;
mod storage;


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

    /// display both comments and tasks (can't be used with --hollow)
    #[arg(long, short)]
    both: bool,
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
        #[arg(long, short)]
        both: bool,

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
    },

    /// Display all 'about' comments
    About {
        /// save all 'about' blocks to the files (per folder)
        #[arg(short, long)]
        save: Option<Option<String>>,
    },

    /// list of projecst. Shows status (is chant initialized or not), name and paht
    Stave {
        /// add current directory as a chant project (also added with 'chant init')
        #[arg(short, long)]
        track: bool,

        /// remove current directory from projects list (also added with 'chant dismiss')
        #[arg(short, long)]
        remove: bool
    }
}

#[derive(Subcommand, Clone)]
enum TaskOpt {
    /// add new task with "message". Command have alias "new"
    #[command(alias="new")]
    Add {
        message: String
    },

    /// mark task as "done" by id
    Done { id: String },

    /// edit task message
    Edit { id: String },

    /// remove task
    Remove {
        /// remove all of the tasks
        #[arg(long)]
        all: bool,

        /// remove all of the tasks
        #[arg(long)]
        done: bool,

        /// remove one task by id
        id: Option<Vec<String>>,
    },
}

// Examples of
// NOTE: some note
// TODO: this is a todo
// FIXME: and here is a fixme

fn main() {
    let cli = Cli::parse();
    if cli.todo || cli.note || cli.todo || cli.hollow || cli.both {
        if cli.hollow && cli.both {
            let error = services::color::paint_str("Error:".to_string(), services::color::Color::Red);
            let hollow = services::color::paint_str("--hollow".to_string(), services::color::Color::Yellow);
            let both = services::color::paint_str("--both".to_string(), services::color::Color::Yellow);
            println!("{error} flag {hollow} can't be used with {both}. Use one or another");
            return;
        }
        if cli.hollow {
            commands::scan::scan_hollow(cli.todo, cli.note, cli.fixme);
            return;
        } else {
            commands::list::list(cli.todo, cli.note, cli.fixme, cli.both);
            if cli.both {
                commands::list::list_both();
                return;
            }
        }
    }
    match cli.cmd {
        Some(Cmds::Stave { track, remove }) => {
            // NOTE: I don't rly like how this turns out, but this is fine for now, I guess
            if track && !remove {
                commands::stave::add_this();
            } else if remove && !track {
                commands::stave::remove_this();
            } else if remove && track {
                println!("You cannot use both flags at the same time");
            } else {
                commands::stave::projects();
            }
        }
        Some(Cmds::About { save }) => {
            match save {
                Some(file) => {
                    match file {
                        Some(v) => commands::about::save(Some(v)),
                        None => commands::about::save(None)
                    }
                }
                None => { commands::about::list(); }
            }
        }
        Some(Cmds::Init { }) => commands::init::init(),
        Some(Cmds::Scan { }) => commands::scan::scan_force(),
        Some(Cmds::List { both, todo, note, fixme}) => {
            commands::list::list(todo, note, fixme, both);
            if both {
                commands::list::list_both();
            }
        },
        Some(Cmds::Dismiss { }) => commands::dismiss::dismiss(),
        Some(Cmds::Config { }) => commands::config::print_config(),
        Some(Cmds::Reset { }) => commands::general::reset(),
        Some(Cmds::Task { option }) => {
            match option {
                Some(TaskOpt::Add { message }) => { commands::task::add_task(message) }
                Some(TaskOpt::Done { id }) => { commands::task::done_task(id); }
                Some(TaskOpt::Edit { id }) => { commands::task::edit_task(id); }
                Some(TaskOpt::Remove { all, done, id }) => {
                    if all && done {
                        let error = services::color::paint_str("Error:".to_string(), services::color::Color::Red);
                        let all = services::color::paint_str("--all".to_string(), services::color::Color::Yellow);
                        let done = services::color::paint_str("--done".to_string(), services::color::Color::Yellow);
                        println!("{error} flag {all} can't be used with {done}. Use one or another");
                        return;
                    }
                    if all {
                        commands::task::remove_all();
                    } else if done {
                        commands::task::remove_done();
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
        None => commands::list::list(true, true, true, false),
    }
}
