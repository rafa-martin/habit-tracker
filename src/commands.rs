use crate::db::{FileDatabase};
use chrono::Local;

pub fn add_command(db: &mut FileDatabase, name: &String) -> Result<(), std::io::Error> {
    let data = db.get_mut_data()?;
    let id = data.add_task(name);
    println!("Added task as '{}' with ID {}.", name, id);
    Ok(())
}

pub fn done_command(db: &mut FileDatabase, id: u32) {
    let data = db.get_mut_data();
    if data.is_err() {
        eprintln!("Error accessing database: {}", data.err().unwrap());
        return;
    }
    let data = data.unwrap();

    // Mark the task as done for today
    match data.mark_task_done(id, Local::now()) {
        Ok(_) => println!("Task with ID {} marked as done.", id),
        Err(e) => {
            eprintln!("Error marking task as done: {}", e);
        }
    }
}

pub fn list_command(db: &mut FileDatabase) -> Result<(), std::io::Error> {
    let data = db.get_data()?;
    println!("Listing items:");
    for task in data.tasks.iter() {
        println!("- {} (ID: {})", task.name, task.id);
    }
    Ok(())
}

pub fn stats_command(db: &mut FileDatabase) -> Result<(), std::io::Error> {
    let task_str_len = 20; // Length of the task name column
    let days = 7; // Number of days to show in the statistics
    let data = db.get_data()?;

    // Print the header for the statistics
    println!("Showing statistics for the last {} days.", days);
    print!("ID | {:<width$} | ", "Task", width = task_str_len + 2);
    for day in 1..days {
        print!("-{}d | ", days - day);
    }
    println!("Today");
    println!("{:-<width$}", "", width = task_str_len + 9 + (days * 6));

    // Print the tasks and their completion status
    let today = chrono::Local::now();

    for task in data.tasks.iter() {
        print!("{:>2} | {:<width$} ", task.id, task.name, width = task_str_len + 2);
        for day in 0..days {
            let diff = chrono::Duration::days((days - day - 1) as i64);
            print!("|  {}  ", match data.is_task_done(task.id, today - diff) {
                true => 'X',
                false => ' ',
            });
        }
        println!();
    }
    Ok(())
}

pub fn today_command(db: &mut FileDatabase) -> Result<(), std::io::Error> {
    let data = db.get_data()?;
    let today = chrono::Local::now();

    // Calculate remaining tasks for today
    let mut remaining_tasks = Vec::new();
    for task in data.tasks.iter() {
        if !data.is_task_done(task.id, today) {
            remaining_tasks.push(task);
        }
    }
    if remaining_tasks.is_empty() {
        println!("Good job! You have no tasks for today.");
        return Ok(());
    }

    println!("Today's tasks:");
    for task in data.tasks.iter() {
        if data.is_task_done(task.id, today) {
            continue; // Skip tasks that are already done today
        }
        println!("- {} (ID: {})", task.name, task.id);
    }
    Ok(())
}
