pub mod number_word_parser;
pub mod ordinal_number_parser;
pub mod ordinal_number_word_parser;
pub mod pattern_parser;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[derive(Debug)]
pub struct UsableArgs {
	args: Vec<String>,
	used: Vec<bool>,
}

impl UsableArgs {
	pub fn from(args: &[String]) -> Self {
		let length = args.len();
		Self {
			args: args.to_vec(),
			used: (0..length).into_iter().map(|_| false).collect(),
		}
	}

	pub fn use_args(&mut self, position: usize, amount: usize) {
		if amount == 0 {
			panic!("Amount of args to use must be more than 0.");
		}
		let end = position + amount;
		if end >= self.args.len() {
			// for trouble shooting during parser testing
			return;
			// panic!("Should not use args out of range.");
		}
		for i in position..end {
			self.used[i] = true;
		}
	}

	pub fn positions_and_windows(&self, size: usize) -> Vec<(usize, &[String])> {
		let last_used_position = |start: usize| -> Option<usize> {
			let mut last_used = None;
			for i in 0..size {
				if self.used[start + i] {
					last_used = Some(i + 1);
				}
			}
			last_used
		};

		let mut windows = Vec::new();
		let mut start = 0;
		loop {
			let end = start + size;
			if end > self.args.len() {
				break;
			}
			match last_used_position(start) {
				Some(pos) => start += pos,
				None => {
					windows.push((start, &self.args[start..end]));
					start += 1;
				}
			}
		}

		windows
	}

	pub fn parse(&self, size: usize) {
		println!("{:?}", self.args);
		println!("{:?}", self.used);
		for window in self.positions_and_windows(size) {
			println!("{:?}", window);
		}
	}
}

use std::ops::Index;

impl Index<usize> for UsableArgs {
	type Output = String;

	fn index(&self, index: usize) -> &Self::Output {
		&self.args[index]
	}
}

#[derive(Debug)]
pub struct Todo {
	completed: bool,
	name: String,
	time: Option<NaiveTime>,
	warning: usize,
}

impl Todo {
	pub fn new(name: &str, time: Option<NaiveTime>, warning: usize) -> Self {
		Self {
			completed: false,
			name: name.to_string(),
			time,
			// description: if description == "" {
			//     None
			// } else {
			//     Some(description.to_string())
			// },
			warning,
		}
	}

	pub fn is_completed(&self) -> bool {
		self.completed
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn time(&self) -> Option<&NaiveTime> {
		self.time.as_ref()
	}

	// pub fn description(&self) -> Option<&str> {
	//     self.description.as_ref().map(|s| s.as_ref())
	// }

	pub fn mark_completed(&mut self) {
		self.completed = true;
	}

	pub fn warning(&self) -> usize {
		self.warning
	}

	// pub fn warning_string(&self, ) -> String {
	// 	format!("{} is due in {} days.", self.name)
	// }
}



// pub struct CsvLine<'a> {
//     data: String, 
//     headers: &'a [String], 
// } 

// pub struct CsvReader {
//     file: File, 
//     headers: Vec<String>, 
// }

// impl CsvReader {
//     pub fn new(file: File, headers: &[&str]) -> Self {
//         Self {
//             file, 
//             headers: headers.into_iter().map(|s| s.to_string()).collect(),
//         }
//     }

//     pub fn 

// pub fn date_and_todo_from_csv(csv_line: String, date: NaiveDate) -> Result<(NaiveDate, Todo), String> {
//     let mut items = csv_line.split(",").map(|s| s.trim());
//     let year = items.next().ok_or("Expected year but found nothing.".to_string())?.parse().or("Could not parse year to positive integer.")?;
//     let month = items.next().ok_or("Expected month but found nothing.".to_string())?.parse().or("Could not parse year to positive integer.")?;
//     let day = items.next().ok_or("Expected day but found nothing.".to_string())?.parse().or("Could not parse year to positive integer.")?;
//     let warning_name = items.next().ok_or("Expected warning_name but found nothing.".to_string())?;
//     let warning_string = items.next().ok_or("Expected warning_string but found nothing.".to_string())?;
//     let todo_name = items.next().ok_or("Expected todo_name but found nothing.".to_string())?;
//     let completed = items.next().ok_or("Expected completed but found nothing.".to_string())?;
//     let todo_time = items.next().ok_or("Expected todo_time but found nothing.".to_string())?;
//     let warning_amount = items.next().ok_or("Expected warning_amount but found nothing.".to_string())?.parse().or("Could not parse year to positive integer.")?;



//     let date = NaiveDate::from_ymd_opt(year, month, day
// }

// pub fn todo_to_csv(todo: Todo, date: NaiveDate) -> Result<String, String> {
//     todo!()
// } 

// impl std::fmt::Display for Todo {
// 	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
// 		let completed_character = if self.is_completed() { 'X' } else { '_' };
// 		let name = self.name();
// 		let time_string = match &self.time() {
// 			Some(time) => format!(" @ {}", time.format("%-I:%M %p").to_string()),
// 			None => String::new(),
// 		};
// 		// let description = match &self.description() {
// 		//     Some(description) => format!("-- {}", description),
// 		//     None => String::new(),
// 		// };

// 		write!(f, "{completed_character} {name}{time_string}")
// 	}
// }

// #[derive(Debug)]
// pub struct DailyTodoList {
//     date: NaiveDate,
//     todos: Vec<Todo>,
// }

// impl DailyTodoList {
//     pub fn new(date: NaiveDate, todos: Vec<Todo>) -> Self {
//         Self { date, todos }
//     }

//     pub fn date(&self) -> &NaiveDate {
//         &self.date
//     }

//     pub fn todos(&self) -> &Vec<Todo> {
//         &self.todos
//     }

//     pub fn add(&mut self, todo: Todo) {
//         self.todos.push(todo);
//     }
// }

use std::fs::{OpenOptions, File, create_dir_all, remove_file, remove_dir};
use std::io::Read;
use std::path::{PathBuf, Path};
use chrono::Datelike;

// warnings.csv headers
// warning_name, warning_message

// todos.csv headers
// name, completed, time, description (if still going to be used)

// todo add cat-box warn 1 day at 6:15 pm
// repeat will be a *later* functionality lol

#[derive(Debug)]
pub struct TodoFile {
    warnings_csv: File,
    todos_csv: File, 
    date: NaiveDate,
}

impl TodoFile {
    const ROOT_DIRECTORY: &'static str = "todo";
    const WARNINGS_FILENAME: &'static str = "warnings.csv";
    const TODO_INFORMATION_FILENAME: &'static str = "todos.csv";

    fn directory_path(date: NaiveDate) -> PathBuf {
        PathBuf::from(&format!("{}/{}/{}/{}", Self::ROOT_DIRECTORY, date.year(), date.month(), date.day()))
    }

    fn warnings_csv_filename() -> PathBuf {
    	Self::directory_path().push(Self::WARNINGS_FILENAME)
    }

    fn todos_csv_filename() -> PathBuf {
    	Self::directory_path().push(Self::TODO_INFORMATION_FILENAME)
    }

    /// Will try to open or create the TodoFile (and any directories) for the specified date.
    pub fn try_open_or_create(date: NaiveDate) -> Result<Self, String> {
        let directory_path = Self::directory_path(date);
        create_dir_all(&directory_path).or(Err(format!("Could not create directories for date: {date}.")))?;

        let warnings_csv = OpenOptions::new().write(true).create(true).open(&Self::warnings_csv_filename()).or(Err("Could not create/open warnings file.".to_string()))?;
        let todos_csv = OpenOptions::new().write(true).create(true).open(&Self::warnings_csv_filename()).or(Err("Could not create/open todo information file.".to_string()))?;
        
        Ok(Self {
        	warnings_csv, 
        	todos_csv, 
        	date, 
        })
    }

    /// Removes the file and/or directories housing the file if they will become empty
    pub fn remove(self) -> Result<(), String> {
        let filename_path = Self::filename_path(self.date);
        remove_file(&filename_path).or(Err(format!("Could not remove file: {filename_path}.")))?;
        let directory_path = Self::directory_path(self.date);
        remove_dir(&directory_path).or(Err(format!("Could not remove directory: {directory_path}.")))?;
        let year_directory_path = format!("{}/{}", Self::ROOT_DIRECTORY, self.date.year());
        remove_dir(&year_directory_path).or(Err(format!("Could not remove directory: {year_directory_path}.")))?;
        Ok(())
    }

    pub fn add_warning(&self, warning_todo_name: &str, warning: &str) -> bool {
        todo!()
    }

    pub fn add_todo(&self, todo: Todo) -> bool {
        todo!()
    }

    pub fn remove_todo(&self, todo_name: &str) -> bool {
        todo!()
    }

    pub fn remove_warning(&self, warning_todo_name: &str) {
        todo!()
    }
}

use std::collections::HashMap;

/// must be stored and read from a file/folder structure
pub struct TodoManager {
	todo_lists: HashMap<NaiveDate, TodoFile>,
}

impl TodoManager {
	pub fn new() -> Self {
		Self {
			todo_lists: HashMap::new(),
		}
	}

	pub fn add(&mut self, todo: Todo, date: NaiveDate) {
		todo!()
		if 
		// self.todo_lists
		//     .entry(date)
		//     .and_modify(|todo_list| todo_list.push(todo))
		//     .or_insert(vec![todo]);
	}
}

pub struct Command(fn(UsableArgs, NaiveDateTime, &mut TodoManager) -> Result<(), String>);

impl Command {
	pub fn execute(
		&self,
		args: UsableArgs,
		context: NaiveDateTime,
		todo_manager: &mut TodoManager,
	) -> Result<(), String> {
		(self.0)(args, context, todo_manager)
	}
}

// fn parse_time(args:

fn parse_todo(args: &mut UsableArgs) -> Result<Todo, String> {
	todo!()
}

fn parse_date(args: &mut UsableArgs) -> Result<NaiveDate, String> {
	todo!()
}

fn test_add_command() {
	let add_command = Command(
		|mut args: UsableArgs,
		 context: NaiveDateTime,
		 todo_manager: &mut TodoManager|
		 -> Result<(), String> {
			let date = parse_date(&mut args)?;
			let todo = parse_todo(&mut args)?;
			todo_manager.add(todo, date);
			Ok(())
		},
	);
}

// pub struct Sequential

// pub struct Command {
//     options: Vec<CommandOptions>,
// }

// pub struct CommandOption {
//
