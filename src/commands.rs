use crate::db::HabitDatabase;
use crate::config::{TASK_STR_LEN, DEFAULT_DAYS};

pub fn add_command<T: HabitDatabase>(db: &mut T, name: &String) -> Result<(), std::io::Error> {
    let data = db.get_mut_data()?;
    let id = data.add_task(name);
    println!("Added task as '{}' with ID {}.", name, id);
    Ok(())
}

pub fn done_command<T: HabitDatabase>(db: &mut T, id: u32) -> Result<(), String> {
    let data = db.get_mut_data().map_err(|e| e.to_string())?;
    data.mark_task_done(id, chrono::Local::now())
        .map_err(|e| format!("Error marking task as done: {}", e))?;
    println!("Marked task with ID {} as done.", id);
    Ok(())
}

pub fn list_command<T: HabitDatabase>(db: &mut T) -> Result<(), std::io::Error> {
    let data = db.get_data()?;
    println!("Listing items:");
    for task in data.tasks.iter() {
        println!("- {} (ID: {})", task.name, task.id);
    }
    Ok(())
}

pub fn stats_command<T: HabitDatabase>(db: &mut T) -> Result<(), std::io::Error> {
    let data = db.get_data()?;

    // Print the header for the statistics
    println!("Showing statistics for the last {} days.", DEFAULT_DAYS);
    print!("ID | {:<width$} | ", "Task", width = TASK_STR_LEN + 2);
    for day in 1..DEFAULT_DAYS {
        print!("-{}d | ", DEFAULT_DAYS - day);
    }
    println!("Today");
    println!("{:-<width$}", "", width = TASK_STR_LEN + 9 + (DEFAULT_DAYS * 6));

    // Print the tasks and their completion status
    let today = chrono::Local::now();

    for task in data.tasks.iter() {
        print!(
            "{:>2} | {:<width$} ",
            task.id,
            task.name,
            width = TASK_STR_LEN + 2
        );
        for day in 0..DEFAULT_DAYS {
            let diff = chrono::Duration::days((DEFAULT_DAYS - day - 1) as i64);
            print!(
                "|  {}  ",
                match data.is_task_done(task.id, today - diff) {
                    true => 'X',
                    false => ' ',
                }
            );
        }
        println!();
    }
    Ok(())
}

pub fn today_command<T: HabitDatabase>(db: &mut T) -> Result<(), std::io::Error> {
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
