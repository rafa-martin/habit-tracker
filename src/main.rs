mod cli;
mod commands;
mod config;
mod db;

use clap::Parser;
use cli::{
    Cli,
    run,
};

use crate::db::{
    FileDatabase,
    HabitDatabase,
};

fn main() -> Result<(), String> {
    let mut db = FileDatabase::new("habits.json");
    db.init().map_err(|e| e.to_string())?;

    run(Cli::parse(), &mut db).map_err(|e| e.to_string())?;
    Ok(())
}
