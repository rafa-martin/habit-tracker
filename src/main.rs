mod cli;
mod commands;
mod db;

use clap::Parser;
use cli::{Cli};
use cli::run;

use crate::db::HabitDatabase;

fn main() {
    let mut db = db::FileDatabase::new("habits.json");
    if let Err(e) = db.init() {
        eprintln!("Error reading database: {}", e);
    }
    run(Cli::parse(), &mut db);
}
