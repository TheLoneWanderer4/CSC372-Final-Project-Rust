use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::fs::File;
use std::fmt;
use std::io::prelude::*;
use chrono::{NaiveDate, Local, DateTime, Utc, Duration};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug,Eq)]
pub struct Rules {
    pub rise: i64, 
    pub when: i64,
    pub maxp: i64
}

impl PartialEq for Rules {
    fn eq(&self, other: &Self) -> bool {
        self.rise == other.rise && self.when == other.when
    }
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rise : {} When : {}", self.rise, self.when)
    }
}

#[derive(Serialize, Deserialize, Debug,Eq)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub desc: String,
    pub date: NaiveDate,
    pub prio: i64,
    pub original_prio: i64,
    pub rule: Rules
}

impl Ord for Task{
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|\n| Id: {} | Name : {} | Due : {} | Priority : {} | Rules: [{}]\n| Description : {}",
                self.id, self.name, self.date, self.prio, self.rule, self.desc)
    }
}

fn update_priority(mut task: Task) -> Task {
    if task.prio > task.rule.maxp {
        task.prio = task.rule.maxp;
        return task;
    }

    let now = Local::now();
    let due_date = DateTime::<Utc>::from_utc(task.date.and_hms(0,0,0), Utc);
    let difference = due_date.signed_duration_since(now).num_days();

    // the case where the task is past due
    if difference <= 0 {
        task.prio = task.rule.maxp;
        return task;
    } 

    // in the case where task.rule.when is 0 the only time priority should update 
    // is when difference == 0
    if task.rule.when != 0 {
        // algorithm will set maxp one day before the due date. 
        // This adds one day to the due date so that the result is correct 
        let mut day_of_first_rise = task.date.checked_add_signed(Duration::days(1)).unwrap();
        let mut hold = task.rule.maxp;
        while hold > task.original_prio {
            day_of_first_rise = day_of_first_rise.checked_sub_signed(Duration::days(task.rule.when)).unwrap();
            hold -= task.rule.rise;
        }
        let day_of_first_rise = DateTime::<Utc>::from_utc(day_of_first_rise.and_hms(0,0,0), Utc);
        let mut difference = now.signed_duration_since(day_of_first_rise).num_days();
        
        // the current day is before when the tasks priority should start rising 
        if difference < 0 {
            return task;
        }

        if difference == 0 {
            difference = 1;
        }

        task.prio = task.original_prio + (task.rule.rise * (task.rule.when / difference));
        if (task.prio > task.rule.maxp) {
            task.prio = task.rule.maxp
        }
        return task;
    }


    // if none of the above conditions are met then the tasks priority doesn't need to 
    // change 
    return task;
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
        return file_result.unwrap().into_iter().map(|t| update_priority(t)).collect();
    }
}

pub fn write_tasks_to_file(file_name: &String, tasks: &Vec<Task>) -> std::io::Result<()> {
    let tasks_string: String = serialize_tasks(&tasks).unwrap();

    let mut file = File::create(&file_name)?;
    file.write_all(&tasks_string.as_bytes())?;
    Ok(())
}