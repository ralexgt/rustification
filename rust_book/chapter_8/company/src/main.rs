// Using a hash map and vectors, create a text interface to allow 
// a user to add employee names to a department in a company; for 
// example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department 
// or all people in the company by department, sorted alphabetically.

use std::{io, vec};
use std::collections::HashMap;
use std::process;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        display_interface();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue
            }
        };
        match choice {
            0 =>  add_to_department(&mut company),
            1 => show_department(&company),
            2 => show_company(&company),
            3 => quit_interface(),
            _ => {
                println!("Option unavailable!");
                continue
            }
        }
    }
}

fn display_interface() {
    println!("
0. Add a person to a department
1. Show everyone in a department
2. Show everyon in the company
3. Quit the program"
    );
}

fn add_to_department(company: &mut  HashMap<String, Vec<String>>) {
    println!("Name of the employee: ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let name: String = match choice.trim().parse() {
        Ok(name) => name,
    };
    
    println!("Name of the department: ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let department: String = match choice.trim().parse() {
        Ok(dep) => dep,
    };
    println!("Added {} in the {} department.", name, department);
    
    let mut current_employees = company.get(&department.to_uppercase()).cloned().unwrap_or(vec![]);
    current_employees.push(name);
    company.insert(department.to_uppercase(), current_employees);
}

fn show_department(company: &HashMap<String, Vec<String>>) {
    println!("Name of the department: ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let department: String = match choice.to_uppercase().trim().parse() {
        Ok(dep) => dep,
    };
    let mut current_employees = company.get(&department).cloned().unwrap_or(vec![]);
    current_employees.sort();

    if current_employees.is_empty() {
        println!("There are currently no employees in the {} department are: ", department);
        return;
    }

    println!("The employees in the {department} department are: ");
    for (i, e) in current_employees.iter().enumerate() {
        println!("    {}: {}", i+1, e);
    }
}

fn show_company(company: &HashMap<String, Vec<String>>) {
    let departments = company.keys();
    for department in departments {
        println!("Department {department}: ");
        let mut current_employees = company.get(department).cloned().unwrap_or(vec![]);
        current_employees.sort();
        for (i, e) in current_employees.iter().enumerate() {
            println!("    {}: {}", i+1, e);
        }
    }
}

fn quit_interface() {
    println!("Done. Have a nice day!");
    process::exit(0);
}