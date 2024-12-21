use std::io;
use std::collections::HashMap;
use company_terminal::terminal_commands::show_personnel;
use company_terminal::terminal_commands::show_department;
use company_terminal::terminal_commands::add_to_department;

fn main() {
    const COMPANY_NAME: &str = "Weyland-Yutani Corporation";
    println!("{COMPANY_NAME}");
    let mut departments: HashMap<String, Vec<String>> = HashMap::from(
        [(String::from("HR"), vec![String::from("Sally"), String::from("Mike")]),
         (String::from("Marketing"), vec![String::from("John"),  String::from("Amellia")])
         ]);
    loop {
        println!("\nShow personnel company-wise by deparment: 1");
        println!("Show personnel from ... deparment: 2");
        println!("Add person to department: 3");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Command failure!");
        let input: &str = input.trim();
        match input {
            "1" => {
                show_personnel::show_all_employes(&departments)
            },
            "2" => {
                show_department::show_employes_from_department(&departments)
            },
            "3" => {
                departments = add_to_department::add_person_to_department(departments);
            },
            _ => {println!("Unknown command!")}
        }
    }
}
