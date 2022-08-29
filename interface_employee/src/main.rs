// Using a hash map and vectors, create a text interface to allow a user to add
// employee names to a department in a company. For example, “Add Sally to
// Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of
// all people in a department or all people in the company by department,
// sorted alphabetically.

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

type Employee = String;
type Departement = String;
type Employees = HashSet<Employee>;
type DataBase = HashMap<Departement, Employees>;

enum Command {
    Add(Employee, Departement),
    Remove(Employee, Departement),
    ListDepartement(Departement),
    ListAll,
}

fn parse_cmd(cmd: &str) -> Option<Command> {
    // Add -> add <name> to <dpt>
    // Remove -> remove <name> from <dpt>
    // ListDepartement -> list <dpt>
    // ListAll -> list all
    let tokens: Vec<&str> = cmd.split(' ').collect();
    match tokens.len() {
        2 => match tokens[0].to_lowercase().as_str() {
            "list" => {
                if tokens[1] == "all" {
                    Some(Command::ListAll)
                } else {
                    Some(Command::ListDepartement(Departement::from(tokens[2])))
                }
            }
            _ => unreachable!(),
        },
        4 => match tokens[0].to_lowercase().as_str() {
            "add" => Some(Command::Add(
                Employee::from(tokens[1]),
                Departement::from(tokens[3]),
            )),
            "remove" => Some(Command::Remove(
                Employee::from(tokens[1]),
                Departement::from(tokens[3]),
            )),
            _ => None,
        },
        _ => None,
    }
}

fn do_cmd(database: &mut DataBase, cmd: &Command) {
    match cmd {
        Command::Add(employee, departement) => {
            insert_employee(database, employee.to_string(), departement.to_string())
        }
        Command::Remove(employee, departement) => remove_employee(database, employee, departement),
        Command::ListDepartement(departement) => list_departement(database, departement),
        Command::ListAll => list_all(database),
    }
}

fn insert_employee(database: &mut DataBase, name: Employee, departement: Departement) {
    match database.get_mut(departement.as_str()) {
        Some(dpt) => {
            dpt.insert(name);
            ()
        }
        None => {
            let mut employees = Employees::new();
            employees.insert(name);
            database.insert(departement, employees);
            ()
        }
    }
}

fn remove_employee(database: &mut DataBase, name: &str, departement: &str) {
    match database.get_mut(departement) {
        Some(dpt) => {
            if !dpt.remove(name) {
                println!("No employee {} in departement {}", name, departement);
            }
        }
        None => println!("No departement {} found", departement),
    }
}

fn list_departement(database: &DataBase, department: &str) {
    match database.get(department) {
        None => println!("No departement {} found", department),
        Some(employees) => {
            println!("Employees' list in departement {}:", department);
            for employee in employees.iter() {
                println!(" - {}", employee);
            }
        }
    }
}

fn list_all(database: &DataBase) {
    for key in database.keys().sorted() {
        list_departement(database, key);
    }
}

fn parse_loop(database: &mut DataBase) {
    let mut is_reading = true;
    while is_reading {
        let mut line = String::new();
        let _input = std::io::stdin().read_line(&mut line).unwrap();
        let command = parse_cmd(&line);
        match command {
            Some(cmd) => do_cmd(database, &cmd),
            None => is_reading = false,
        }
    }
}

fn main() {
    let mut database = HashMap::new();

    let cmd = parse_cmd("Add Sally to Engineering").unwrap();
    {
        match cmd {
            Command::Add(name, departement) => {
                insert_employee(&mut database, name, departement.to_string());
                list_departement(&database, &departement);
            }
            _ => unreachable!(),
        }
    }

    let cmd_2 = parse_cmd("Add Samir to Sales").unwrap();
    {
        match cmd_2 {
            Command::Add(name, departement) => {
                insert_employee(&mut database, name.to_string(), departement.to_string());
                list_all(&database);
            }
            _ => unreachable!(),
        }
    }

    parse_loop(&mut database);

    println!("\nDebug: {:#?}", database);
}
