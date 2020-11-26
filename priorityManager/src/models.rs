use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::fs::File;
use std::fmt;
use std::io::prelude::*;
use chrono::{NaiveDate};

#[derive(Serialize, Deserialize, Debug)]
pub struct Rules {
    pub rise: i32, 
    pub when: i32
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\tRise : {}\n\tWhen : {}", self.rise, self.when)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub desc: String,
    pub date: NaiveDate,
    pub prio: i32,
    pub rule: Rules
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name : {}\nDescription : {}\nDue Date : {} \nPriority : {}\nRules: {}\n",
                self.name, self.desc, self.date, self.prio, self.rule)
    }
}

fn serialize_tasks(task: &Vec<Task>) -> Result<String> {
    let ret = serde_json::to_string(&task)?;    
    Ok(ret)
} 

fn deserialize_tasks(task: &String) -> Result<Vec<Task>> {
    let ret: Vec<Task> = serde_json::from_str(&task)?;    
    Ok(ret)
} 

pub fn get_tasks_from_file(file_name: &String) -> Vec<Task> {
    let file_string: String = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");
    let file_result: Result<Vec<Task>> = deserialize_tasks(&file_string);

    if file_result.is_err() {
        return Vec::new();
    } else {
        return file_result.unwrap();
    }
}

pub fn write_tasks_to_file(file_name: &String, tasks: &Vec<Task>) -> std::io::Result<()> {
    let tasks_string: String = serialize_tasks(&tasks).unwrap();

    let mut file = File::create(&file_name)?;
    file.write_all(&tasks_string.as_bytes())?;
    Ok(())
}