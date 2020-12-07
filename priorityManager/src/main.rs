mod models;
use rand::Rng;
use std::env;
use std::io;
use std::cmp::min;
use chrono::{NaiveDate};
use crate::models::{Task, Rules, get_tasks_from_file, write_tasks_to_file,update_priority};

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

    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let words: Vec<&str> = user_input.split_whitespace().collect();
        
        if words.len() < 1 {
            println!("invalid input");
            continue;
        }

        match words[0] {
            "list_all" => list_all(&filename),
            "add_task" => add_task(&filename, words),
            "list" => list(&filename, words),
            "edit" => edit(&filename,words),
            "info" => info(&filename, words),
            "reload" => reload(&filename),
            "help" => {
                println!("Available Commands : ");
                println!("> list_all ");
                println!("\tLists all tasks in the system.");
                println!("> add_task [-rise X] [-when X] [-maxp X]");
                println!("\tAdds a task to the system. ");
                println!("\tOptional parameters -rise and -when should be followed by a number.");
                println!("\t-when indicates what on what interval priority should rise. The default is 0, indicating priority should rise only on the due date.");
                println!("\t-rise indicates how much to raise the priority every (specified by -when) days before the duedate. The default is 5, indicating priority should rise to the max priority of this task.");
                println!("\t-maxp indicates what the maximum priority of this task should be. The default, and cap, is 5.");
                println!("\tThe remaining fields are supplied following the given prompt.");
                println!("> list priority");
                println!("\tLists all tasks having priority priority.");
                println!("> edit id [-name] [-des] [-due] [-rise] [-when] [-maxp] [-prio]");
                println!("\tAllows for the task having id id to be editied. Each additional argument results in a new prompt to supply a new value for that field");
                println!("> info id");
                println!("\tDisplays the task having id id. ");
                println!("> reload ");
                println!("\tReloads current system, will recalculate priorities.");
                println!("> exit ");
                println!("\tCloses the program gracefully.");
                println!("> help ");
                println!("\tDisplays this help message.");
            },
            "exit" => break,
            _ => println!("invalid input"),
        }
    }
}

fn edit(file_name: &String, user_input: Vec<&str>){
	if user_input.len() < 3{
		println!("No args given");
		return;
	}
    let curr_id:usize = user_input[1].parse().unwrap_or(0);
    if curr_id == 0 {
		println!("Invalid Input");
        return;
    } 
	let deserialized: Vec<Task> = get_tasks_from_file(&file_name);
    let mut found_task = false;

    let new_deserialized:Vec<Task> = deserialized.into_iter().map(|mut task| {
        if task.id == curr_id{
			found_task = true;
            for i in 2..user_input.len(){
				match user_input[i]{
					"-name" => {
						let mut input = String::new();
						println!("Give task name");
						io::stdin().read_line(&mut input).expect("Failed to read line");
						task.name = String::from(input.trim()); },
					"-des" => {
						let mut input = String::new();
						println!("Give task description");
						io::stdin().read_line(&mut input).expect("Failed to read line");
						task.desc = String::from(input.trim()); },
					"-due" => {
						let mut input = String::new();
						let parse_from_str = NaiveDate::parse_from_str;
						println!("Give the date the task must be due by (example input: 2015-09-05)");
						io::stdin().read_line(&mut input).expect("Failed to read line");
						let date_only = parse_from_str(&input,"%Y-%m-%d");
						task.date = NaiveDate::from(date_only.unwrap());},
					"-rise" => {
						let mut input = String::new();
                        println!("Give rise");
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        task.rule.rise = input.trim().parse().unwrap() ;},
					"-when" => {
						let mut input = String::new();
						println!("Give when");
						io::stdin().read_line(&mut input).expect("Failed to read line");
						task.rule.when = input.trim().parse().unwrap() ;},
					"-maxp" => {
						let mut input = String::new();
						println!("Give max priority");
						io::stdin().read_line(&mut input).expect("Failed to read line");
						task.rule.maxp = input.trim().parse().unwrap() ;},
					"-prio" => {
						let mut input = String::new();
						println!("Give max priority");
						io::stdin().read_line(&mut input).expect("Failed to read line");
                        task.original_prio = min(input.parse().unwrap(),  task.rule.maxp); },	
						//task.original_prio = input.parse().unwrap(); },
					_ => println!("invalid input"),
				}
			}
		}
        return task;
    }).collect();
    if !found_task {
        println!("Task {} not found",curr_id);
    }
	if write_tasks_to_file(&file_name, &new_deserialized).is_err() {
        println!("File not found");
    } 
	reload(file_name);
}

fn reload(file_name: &String) {
	let deserialized: Vec<Task> = get_tasks_from_file(&file_name).into_iter().map(|t| update_priority(t)).collect();

	if write_tasks_to_file(&file_name, &deserialized).is_err() {
        println!("File not found");
    } 
}

fn info(file_name: &String, user_input: Vec<&str>){
    let curr_id:usize = user_input[1].parse().unwrap();
    let deserialized: Vec<Task> = get_tasks_from_file(&file_name);
    for task in &deserialized {
        if task.id == curr_id{
            println!("{}", task);
            return (); 
        }
    }
    println!("Task could not be found");

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
    let mut inputrise = 5;
    let mut inputwhen = 0;
    let mut inputmaxp = 5;
    if user_input.contains(&"-rise"){
        inputrise = user_input[user_input.iter().position(|&x| x == "-rise").unwrap()+1].parse().unwrap();
    }
    if user_input.contains(&"-when"){
        inputwhen = user_input[user_input.iter().position(|&x| x == "-when").unwrap()+1].parse().unwrap();
    }
    if user_input.contains(&"-maxp"){
        inputmaxp = user_input[user_input.iter().position(|&x| x == "-maxp").unwrap()+1].parse().unwrap();
    }
    let curr_task = Task {
        id            : rand::thread_rng().gen_range(1, 100000),
        name          : String::from(inputname.trim()),
        desc          : String::from(inputdesc.trim()),
        date          : NaiveDate::from(date_only.unwrap()),
        prio          : min(inputprio.parse().unwrap(), inputmaxp),
        original_prio : min(inputprio.parse().unwrap(), inputmaxp),
        rule          : Rules {
            rise : inputrise,
            when : inputwhen,
            maxp : inputmaxp
        }
    };
    deserialized.push(curr_task);
    if write_tasks_to_file(&file_name, &deserialized).is_err() {
        println!("File not found");
    } 

}

fn list_all(file_name: &String) {
    let mut deserialized: Vec<Task> = get_tasks_from_file(&file_name);
    deserialized.sort();
    for task in &deserialized {
        println!("{}", task);
    }
}

fn list(file_name: &String,user_input: Vec<&str>){
    let deserialized: Vec<Task> = get_tasks_from_file(&file_name);
    let mut aflag = false;
    if user_input.contains(&"-a"){
        aflag = true;
    }
    let mut querry_tasks: Vec<&Task> = Vec::new();
    let priority:i64 = user_input[1].parse().unwrap();
    for task in &deserialized {
        if task.prio == priority{ querry_tasks.push(task); }
        else if task.prio > priority && aflag{ querry_tasks.push(task); }
    }
    querry_tasks.sort();
    for task in &querry_tasks {
        println!("{}", task);
    }
}
