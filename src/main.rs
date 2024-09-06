use std::str::FromStr;
use std::{fs::OpenOptions, io, ops::Index, string, task, vec};
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::fs::write;
#[derive(Serialize, Deserialize,Debug)]
struct Task {
    description: String,
    status : Taskstatus,
    priority: TaskPriority, 
    due_date: NaiveDate,  // Add this field (use the chrono crate)

}
#[derive(Serialize, Deserialize, Debug)]
enum Taskstatus {
    Completed,
    Pending,
}
#[derive(Serialize, Deserialize,Debug)]

enum TaskPriority{
    Low,
    Medium,
    High,
}

fn main(){
   
 //let view = view_task(tasks);
 let mut tasks: Vec<Task> = load_task();

 loop {
    println!("=========> MENU <============");
    println!("=======> [1] ADD TASK ");
    println!("=======> [2] View Tasks ");
    println!("=======> [3] Mark Task As Completed");
    println!("=======> [4] Remove Task");
    println!("=======> [5] Exist The Program");

    println!("ENTER YOUR CHOISE ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error input");
    let choise = input.trim().parse().expect("error");
    match choise {
        1 => {
            add_task(&mut tasks);
            save_task(&tasks);
        }
        2 => view_task(&mut tasks),
        3 => { 
            Completed_task(&mut tasks);
             save_task(&tasks);
        }
        4 => {
            remove_task(&mut tasks);
            save_task(&tasks);
        },
        5 => {
            break;
            save_task(&tasks);
        }
        _ => println!("Error Selecting"),
        
    }


 }

}
fn add_task(task: &mut Vec<Task>){
    println!("Enter Task Description");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error input");
    let description = input.trim().to_string();


    println!("Enter Task Priority (Low, Medium, High):");
let mut predioinp = String::new();
io::stdin().read_line(&mut predioinp).expect("error occurs");
let priodity_str = predioinp.trim();

println!("Enter Task Due Date (YYYY-MM-DD):");
let mut due_date_inp = String::new();
io::stdin().read_line(&mut due_date_inp).expect("error");
let due_date= NaiveDate::parse_from_str(due_date_inp.trim(), "%Y-%m-%d").expect("error "); 

let priodity= match priodity_str {
    "Low" => TaskPriority::Low,
    "Higth" => TaskPriority::High,
    "Medium" => TaskPriority::Medium,
    _ => {
        println!("Invalid Input ");
        TaskPriority::Low
    }
};

    let tasks =Task{
        description: description,
        status : Taskstatus::Pending,
        priority: priodity,
        due_date: due_date,
    };

    task.push(tasks);
    println!("New Task Is added");

}
fn load_task()-> Vec<Task> {
    let json_string = std::fs::read_to_string("task.json").unwrap_or_else(|_| String::new());
     serde_json::from_str(&json_string).unwrap_or_else(|_| Vec::new())
}
fn save_task(tasks: &Vec<Task>){
   let json_String = serde_json::to_string(tasks).expect("error");
   std::fs::write("task.json", json_String).expect("error");

}
fn view_task (tasks: &Vec<Task>){
   
    if (tasks.is_empty()) {
        println!("NO AVALAIBLE TASK");
        
    }else {
        for (index , task) in tasks.iter().enumerate()  {
            println!(
                "{}: {} | Status: {:?} | Priority: {:?} | Due Date: {}",
                index, task.description, task.status, task.priority, task.due_date
            );
        }
    }

}
fn Completed_task(task: &mut Vec<Task>){

    println!("Input Task Number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let index:usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Index Number");
            return;
        }
    };
    if (index < task.len()) {
        task[index].status = Taskstatus::Completed;
        println!("Task {} Mark as Completed ",index);
        
    }else {
        println!("Task  Not Found");
    }
}
fn remove_task(task: &mut Vec<Task>){
    println!("Enter Task Number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let index:usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("error occur ");
            return;
        },
        };
    if task.is_empty() {
        println!("Task Not Found");
    }else {
        if (index < task.len()) {
            task.remove(index);
            println!("Task NUmber {} Is removed",index);
            
        }
    }
}