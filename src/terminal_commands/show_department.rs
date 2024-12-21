use std::io;
use std::collections::HashMap;

pub fn show_employes_from_department(company_dict: &HashMap<String, Vec<String>>) {
	let mut input: String = String::new();
	println!("Enter department name");
    io::stdin().read_line(&mut input).expect("Command failure!");
    let department: &str = input.trim();
	println!("{:?}:", department);
	let department = company_dict.get(department as &str).expect("Err: invalid department");
	for person in department{
		print!("{:?} ", person);
	}
}