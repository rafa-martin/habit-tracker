use clap::Parser;
use clap::Subcommand;

use crate::commands::{add_command, done_command, list_command, stats_command, today_command};
use crate::db::HabitDatabase;

#[derive(Parser, Default, Debug)]
pub struct Cli {
    /// Path to the data file
    #[arg(long, short = 'd', default_value = "habits.json")]
    pub data_path: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add an habit or task
    Add { name: String },

    /// Mark an habit or task as done
    Done { id: u32 },

    /// List all habits or tasks
    List,

    /// Today remaining tasks
    Today,

    /// Show statistics of completed tasks
    Stats,
}

pub fn run<T: HabitDatabase>(args: Cli, db: &mut T) -> Result<(), String> {
    match args.command {
        Some(Commands::Add { name }) => add_command(db, &name).map_err(|e| e.to_string()),
        Some(Commands::Done { id }) => done_command(db, id).map_err(|e| e.to_string()),
        Some(Commands::List) => list_command(db).map_err(|e| e.to_string()),
        Some(Commands::Today) => today_command(db).map_err(|e| e.to_string()),
        Some(Commands::Stats) => stats_command(db).map_err(|e| e.to_string()),
        None => Err("No command provided".to_string()),
    }
}
