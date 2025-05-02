use std::io::{stdin, stdout, Read, Write};

#[derive(Debug)]
struct Task {
    is_done:bool,
    task: String,
}

fn add_task(tasks : &mut Vec<Task>, task: Task) {
    println!("task added {:?}",task);
    tasks.push(task);
}

fn show_all_tasks(tasks : &mut Vec<Task>){
    println!("tasks to do::");
    for (i,task) in tasks.iter().enumerate(){
        if !task.is_done{
            println!("Task Number {} => {}",i+1,task.task);
        } 
    }
}
fn main() {
    let mut tasks:Vec<Task> = Vec::new();
    println!("Welcome to TODO List: ");
    loop {
        println!("1. To  display all tasks");
        println!("2. To add task");
        println!("9. To Exit");
        let mut choice:String = String::new();
        stdin().read_line(&mut choice).expect("failed to read string from line");
        let x:i8 = choice.trim().parse().expect("failed to get input");
        
        match x {
            1 => show_all_tasks(&mut tasks),
            2 => {
                let mut taskinput:String = String::new();
                stdin().read_line(&mut taskinput).expect("enter a valid string");
                let taskc:Task = Task { is_done: false, task: taskinput };
                add_task(&mut tasks, taskc);
            },
            9 => {println!("thanks come again!"); break},
            _ => println!("invalid input"),
        }
        
    }
}
