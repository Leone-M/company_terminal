use std::io;
use std::collections::HashMap;

pub fn add_person_to_department(mut dep_dict: HashMap<String, Vec<String>>)
-> HashMap<String, Vec<String>> {
	let mut input: String = String::new();
	println!("Command format: Add ... to ...");
    io::stdin().read_line(&mut input).expect("Command failure!");
    let command: &str = input.trim();
    let splited_command: Vec<&str> = command.split(" ").collect();
    let member_department = dep_dict.get_mut(&String::from(splited_command[3])).expect("Err: invalid department");
  	member_department.push(splited_command[1].to_string());
  	let new_member_department = member_department.clone();
    dep_dict.insert(String::from(splited_command[3]), new_member_department.to_vec());
	return dep_dict
	
}