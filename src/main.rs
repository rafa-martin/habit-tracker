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
    let args = Cli::parse();
    let mut db = FileDatabase::new(&args.data_path);
    db.init().map_err(|e| e.to_string())?;
    run(args, &mut db).map_err(|e| e.to_string())?;
    Ok(())
}
