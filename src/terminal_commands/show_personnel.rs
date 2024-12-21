use std::collections::hash_map::Keys;
use std::collections::HashMap;

pub fn show_all_employes(company_dict: &HashMap<String, Vec<String>>) {
	let keys = company_dict.keys();
	let srtd_keys = sort_keys(keys);
	for key in srtd_keys {
		println!("{:?}:", key);
		for person in company_dict.get(&key as &str).unwrap(){
			print!("{:?} ", person);
		}
		println!("")
	}
}

fn sort_keys(keys: Keys<String, Vec<String>>) -> Vec<String> {
	let mut srtd_keys: Vec<String> = Vec::new();
	for key in keys {
		srtd_keys.push(String::from(key))
	}
	srtd_keys.sort();
	srtd_keys
}