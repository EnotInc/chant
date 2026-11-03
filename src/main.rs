use clap::{Parser};

mod commands;
mod services;
mod parser;
mod storage;
mod cli_struct;

use crate::{cli_struct::{Cli, Cmds, TaskOpt}, commands::general::dont_run_at_home};

fn main() {
    dont_run_at_home();

    let cli = Cli::parse();
    if cli.todo || cli.note || cli.fixme || cli.hollow || cli.both {
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
        } 

        commands::list::list(cli.todo, cli.note, cli.fixme, cli.both);
        if cli.both {
            commands::list::list_both();
            return;
        }
    }
    match cli.cmd {
        None => commands::list::list(true, true, true, false),
        Some(Cmds::Dismiss { }) => commands::dismiss::dismiss(),
        Some(Cmds::Init { }) => commands::init::init(),
        Some(Cmds::Scan { }) => commands::scan::scan_force(),
        Some(Cmds::Reset { }) => commands::general::reset(),
        Some(Cmds::Stave { track, remove , short}) => {
            match (track, remove) {
                (true,  true) => { println!("You cannot use both flags at the same time"); }
                (true,  false) => { commands::stave::add_this(); }
                (false, true) => { commands::stave::remove_this(); }
                (false, false) => { commands::stave::projects(short); }
            }
        }
        Some(Cmds::List { both, todo, note, fixme}) => {
            commands::list::list(todo, note, fixme, both);
            if both { commands::list::list_both(); }
        },
        Some(Cmds::Task { option }) => {
            match option {
                None => { commands::tasks::print_tasks(); }
                Some(TaskOpt::Add { message, in_progress, done }) => { commands::tasks::add_task(message, in_progress, done) }
                Some(TaskOpt::Done { id }) => { commands::tasks::done_task(id); }
                Some(TaskOpt::InProgress { id }) => { commands::tasks::progress_task(id); }
                Some(TaskOpt::Backlog { id }) => { commands::tasks::backlog_task(id); }
                Some(TaskOpt::Migrate) => { commands::tasks::migrate(); }
                Some(TaskOpt::Edit { id }) => { commands::tasks::edit_task(id); }
                Some(TaskOpt::Remove { all, done, id }) => {
                    match (all, done ) {
                        (true,  false) => { commands::tasks::remove_all(); }
                        (false, true) => { commands::tasks::remove_done(); }
                        (false, false) => {
                            match id {
                                Some(v) => commands::tasks::remove_task(v),
                                None => commands::general::bad_syntax(),
                            }
                        }
                        (true,  true) => {
                            let error = services::color::paint_str("Error:".to_string(), services::color::Color::Red);
                            let all = services::color::paint_str("--all".to_string(), services::color::Color::Yellow);
                            let done = services::color::paint_str("--done".to_string(), services::color::Color::Yellow);
                            println!("{error} flag {all} can't be used with {done}. Use one or another");
                        }
                    }
                }
            }
        },
    }
}
