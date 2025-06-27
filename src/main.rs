mod cli;
mod commands;
mod db;

use clap::Parser;
use cli::{Cli};
use cli::run;

fn main() {
    let mut db = db::FileDatabase::new("data.json");
    if let Err(e) = db.init() {
        eprintln!("Error reading database: {}", e);
    }
    run(Cli::parse(), &mut db);
}
