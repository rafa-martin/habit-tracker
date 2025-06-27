// tests/cli.rs
use assert_cmd::Command;
use predicates::str::contains;
use assert_fs::{prelude::PathChild, TempDir};

#[test]
fn add_and_list_habit() {
    // ❶ create an isolated workspace so tests don’t stomp on ~/.config
    let tmp = TempDir::new().unwrap();
    let data_file = tmp.child("habits.json");

    // ❷ run: `habit-tracker add "Drink water"`
    Command::cargo_bin("habit-tracker")
        .unwrap()
        .args(["--data-path", data_file.path().to_str().unwrap(), "add", "Drink water"])
        .assert()
        .success()
        .stdout(contains("Added"));

    // ❸ run: `habit-tracker list`
    Command::cargo_bin("habit-tracker")
        .unwrap()
        .args(["--data-path", data_file.path().to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(contains("Drink water"));
}

#[test]
fn mark_habit_done() {
    // ❶ create an isolated workspace so tests don’t stomp on ~/.config
    let tmp = TempDir::new().unwrap();
    let data_file = tmp.child("habits.json");
    // ❷ run: `habit-tracker add "Drink water"`
    Command::cargo_bin("habit-tracker")
        .unwrap()
        .args(["--data-path", data_file.path().to_str().unwrap(), "add", "Drink water"])
        .assert()
        .success()
        .stdout(contains("Added"));
    // ❸ run: `habit-tracker done 1`
    Command::cargo_bin("habit-tracker")
        .unwrap()
        .args(["--data-path", data_file.path().to_str().unwrap(), "done", "1"])
        .assert()
        .success()
        .stdout(contains("Marked task with ID 1 as done."));
}
