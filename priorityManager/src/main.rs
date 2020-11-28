mod models;
use std::env;
use chrono::{NaiveDate};
use crate::models::{Task, Rules, get_tasks_from_file, write_tasks_to_file};

const DEFAULT_FILENAME: &str = "user/savedtasks";

fn get_filename() -> String {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        return args[1].clone();
    } else {
        return String::from(DEFAULT_FILENAME);
    }
}
 
fn main() {
    let filename: String = get_filename();

    let mut test_task = Task {
        id : 0,
        name : String::from("This is a test"),
        desc : String::from("We are using this to test"),
        date : NaiveDate::from_ymd(2015, 3, 14),
        prio : 2,
        rule : Rules {
            rise : 5,
            when : 0
        }
    };

    let mut deserialized: Vec<Task> = get_tasks_from_file(&filename);
    test_task.id = deserialized.len();
    deserialized.push(test_task);

    for task in &deserialized {
        println!("{}", task);
    }

    if write_tasks_to_file(&filename, &deserialized).is_err() {
        println!("File not found");
    }
}
