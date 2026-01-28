use clap::{Parser, Subcommand};

mod commands;
mod config;
mod models;

#[derive(Parser)]
#[command(name = "pmcli")]
#[command(about = "Project Manager CLI for Termux")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create new project
    Create { name: String },

    /// Add task to project
    AddTask {
        project: String,
        task: String,

        /// Task priority (low | medium | high)
        #[arg(short, long)]
        priority: Option<String>,

        /// Deadline (uses config date_format)
        #[arg(short, long)]
        deadline: Option<String>,
    },

    /// Show tasks
    Tasks {
        project: String,

        /// Filter: mine | todo | done | blocked
        #[arg(short, long)]
        filter: Option<String>,
    },

    /// Mark task as done
    DoneTask { project: String, id: u32 },

    /// List all projects
    List,

    /// Add project note
    Note { project: String, note: String },

    /// Export project data
    Export { project: String },

    /// Terminal UI
    Tui { project: String },

    // ===== GIT SYNC =====
    /// Initialize git repository
    GitInit { project: String },

    /// Commit project changes
    GitCommit { project: String, message: String },

    /// Push to remote repository
    GitPush { project: String },

    /// Pull from remote repository
    GitPull { project: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name } => commands::create::run(&name),

        Commands::AddTask {
            project,
            task,
            priority,
            deadline,
        } => commands::add_task::run(&project, &task, priority.as_deref(), deadline.as_deref()),

        Commands::Tasks { project, filter } => commands::tasks::run(&project, filter.as_deref()),

        Commands::DoneTask { project, id } => commands::done_task::run(&project, id),

        Commands::List => commands::list::run(),

        Commands::Note { project, note } => commands::note::run(&project, &note),

        Commands::Export { project } => commands::export::run(&project),

        Commands::Tui { project } => commands::tui::run(&project),

        Commands::GitInit { project } => commands::git::init(&project),

        Commands::GitCommit { project, message } => commands::git::commit(&project, &message),

        Commands::GitPush { project } => commands::git::push(&project),

        Commands::GitPull { project } => commands::git::pull(&project),
    }
}
