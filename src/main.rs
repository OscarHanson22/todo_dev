use ftodo::pattern_parser::*;
use ftodo::*;
use chrono::*;

fn main() {
	let date = NaiveDate::from_ymd_opt(2025, 10, 29).unwrap();
	let date2 = NaiveDate::from_ymd_opt(2026, 11, 30).unwrap();
	let date3 = NaiveDate::from_ymd_opt(2025, 10, 30).unwrap();
	let todo_file = TodoFile::try_open_or_create(date).expect("Could not open file"); 
	let todo_file2 = TodoFile::try_open_or_create(date2).expect("Could not open file"); 
	let todo_file3 = TodoFile::try_open_or_create(date3).expect("Could not open file"); 

	todo_file2.remove();
	todo_file.remove();
	todo_file3.remove();

	// let parser = at_colon_time_pattern_parser();
	// let mut args = UsableArgs::from(&argify(&["at", "something", "at", "6:15", "pm", "end"]));
	// let result = parser.parse(&mut args);
	// println!("13 result: {:?}", result);
	// println!("Hello World!");
}
