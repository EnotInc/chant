use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Cmds>,

    /// run chant without initialization (not recomended)
    #[arg(long)]
    pub hollow: bool,

    /// display only TODO comments
    #[arg(long, short)]
    pub todo: bool,

    /// display only NOTE comments
    #[arg(long, short)]
    pub note: bool,

    /// display only FIXME comments
    #[arg(long, short)]
    pub fixme: bool,

    /// display both comments and tasks (can't be used with --hollow)
    #[arg(long, short)]
    pub both: bool,
}

#[derive(Subcommand, Clone)]
pub enum Cmds {
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

    /// remove .chant/ directory and remove it from .gitignore
    Dismiss {},

    /// reset confign to the default and run forced scan
    Reset {},

    /// Global tasks in the project
    Task {
        #[command(subcommand)]
        option: Option<TaskOpt>,
    },

    /// list of projecst. Shows amount of comments, taksk, name (cyan - chant initialized. red - directory was added manually)
    Stave {
        /// manually add current directory as a chant project (also added with 'chant init')
        #[arg(short, long)]
        track: bool,

        /// manually remove current directory from projects list (also added with 'chant dismiss')
        #[arg(short, long)]
        remove: bool
    }
}

#[derive(Subcommand, Clone)]
pub enum TaskOpt {
    /// add new task with "message". Command have alias "new"
    #[command(alias="new")]
    Add {
        message: String,

        #[arg(short, long)]
        in_progress: bool,

        #[arg(short, long)]
        done: bool,
    },

    /// mark task as "done" by id
    Done { id: String },

    /// mark task as "InProgress" by id. Command have alias "working"
    #[command(alias="working")]
    InProgress { id: String },

    /// mark task as "Backlog" by id
    Backlog {id: String},

    /// edit task message
    Edit { id: String },

    /// migrate old tasks to a new form
    Migrate,

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