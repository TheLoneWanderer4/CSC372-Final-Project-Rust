mod models;
use std::env;
use std::io;
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
    } */

    let mut exitLoop = false;
    while !exitLoop{
        let mut userInput = String::new();

        io::stdin()
            .read_line(&mut userInput)
            .expect("Failed to read line");
        let words: Vec<&str> = userInput.split_whitespace().collect();
        println!("You guessed: {:?}", words);
        match words[0] {
            "list_all" => list_all(&filename),
            "add_task" => add_task(&filename, words),
            "list" => println!("to be implimented "),
            "edit" => println!("to be implimented "),
            "info" => println!("to be implimented "),
            "reload" => println!("to be implimented "),
            _ => println!("invalid input"),
        }
    }
}

fn add_task(file_name: &String, user_input: Vec<&str>){
    let parse_from_str = NaiveDate::parse_from_str;
    let mut deserialized: Vec<Task> = get_tasks_from_file(&file_name);
    let mut inputname = String::new();
    let mut inputdesc = String::new();
    let mut inputprio = String::new();
    let mut inputdate = String::new();
    println!("Give task name");
    io::stdin().read_line(&mut inputname).expect("Failed to read line");
    println!("Give task description");
    io::stdin().read_line(&mut inputdesc).expect("Failed to read line");
    println!("Give task priority");
    io::stdin().read_line(&mut inputprio).expect("Failed to read line");
    inputprio = (&inputprio.trim_end()).to_string();
    println!("Give the date the task must be due by (example input: 2015-09-05)");
    io::stdin().read_line(&mut inputdate).expect("Failed to read line");
    inputdate = (&inputdate.trim_end()).to_string();
    let date_only = parse_from_str(&inputdate,"%Y-%m-%d");
    let mut inputrise = 1;
    let mut inputwhen = 0;
    if user_input.contains(&"-rise"){
        inputrise = user_input[user_input.iter().position(|&x| x == "-rise").unwrap()+1].parse().unwrap();
    }
    if user_input.contains(&"-when"){
        inputwhen = user_input[user_input.iter().position(|&x| x == "-when").unwrap()+1].parse().unwrap();
    }
    let curr_task = Task {
        name : String::from(inputname),
        desc : String::from(inputdesc),
        date : NaiveDate::from(date_only.unwrap()),
        prio : inputprio.parse().unwrap(),
        rule : Rules {
            rise : inputrise,
            when : inputwhen
        }
    };
    deserialized.push(curr_task);
    if write_tasks_to_file(&file_name, &deserialized).is_err() {
        println!("File not found");
    } 

}

fn list_all(file_name: &String) {
    let deserialized: Vec<Task> = get_tasks_from_file(&file_name);
    for task in &deserialized {
        println!("{}", task);
    }
}
