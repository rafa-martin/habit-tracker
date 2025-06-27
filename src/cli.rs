use clap::Parser;
use clap::Subcommand;

use crate::commands::{
    add_command,
    done_command,
    list_command,
    stats_command,
    today_command,
};
use crate::db::HabitDatabase;

#[derive(Parser,Default,Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand,Debug)]
enum Commands {
    /// Add an habit or task
    Add { name: Option<String> },

    /// Mark an habit or task as done
    Done { id: u32 },

    /// List all habits or tasks
    List,

    /// Today remaining tasks
    Today,

    /// Show statistics of completed tasks
    Stats,
}

pub fn run<T: HabitDatabase>(args: Cli, db: &mut T) {
    match args.command {
        Some(Commands::Add { name }) => {
            if let Some(name) = name {
                if let Err(e) = add_command(db, &name) {
                    eprintln!("Error adding item: {}", e);
                }
            } else {
                eprintln!("Please provide a name for the task.");
            }
        },
        Some(Commands::Done { id }) => {
            done_command(db, id);
        },
        Some(Commands::List) => {
            if let Err(e) = list_command(db) {
                eprintln!("Error listing items: {}", e);
            }
        },
        Some(Commands::Stats) => {
            if let Err(e) = stats_command(db) {
                eprintln!("Error showing statistics: {}", e);
            }
        },
        Some(Commands::Today) => {
            if let Err(e) = today_command(db) {
                eprintln!("Error showing today's tasks: {}", e);
            }
        },
        None => println!("No command provided."),
    }
}
