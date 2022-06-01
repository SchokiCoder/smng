/*
	SchokiManager
	Copyright (C) 2021	Andy Frank Schoknecht

	This program is free software: you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation, either version 3 of the License, or
	(at your option) any later version.

	This program is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

pub mod app;
pub mod commands;
use std::env;

pub const MSG_ERR_LOW_ARGUMENTS: &str = "ERROR: Not enough arguments given.";
pub const MSG_WARN_HIGH_ARGUMENTS: &str = "WARNING: Too many arguments were given.\n\
										   Additional arguments will be ignored.";
fn argcount_check(argcount: usize, args_min: usize, args_max: usize) -> bool {
	if argcount < args_min {
		println!("{}", MSG_ERR_LOW_ARGUMENTS);
		return true;
	}

	else if argcount > args_max {
		println!("{}", MSG_WARN_HIGH_ARGUMENTS);
		return false;
	}

	else {
		return false;
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();

	// if not args given, print usage help and end
	if args.len() < 2 {
		println!("Usage {} COMMAND [ARGUMENTS]:", app::NAME);
		println!("Try '{} {}' for more information.", app::NAME, commands::HELP_NAME);
		return;
	}
	
	match args[1].as_str() {
		commands::HELP_NAME | commands::HELP_ABBR => commands::help(),

		commands::ADD_PROJECT_NAME | commands::ADD_PROJECT_ABBR => {
			if argcount_check(args.len(), 3, 3) {
				return;
			}

			commands::add_project(args[2].as_str());
		},

		commands::SHOW_PROJECTS_NAME | commands::SHOW_PROJECTS_ABBR => {
			if argcount_check(args.len(), 2, 2) {
				return;
			}

			commands::show_projects();
		},

		commands::EDIT_PROJECT_NAME | commands::EDIT_PROJECT_ABBR => {
			if argcount_check(args.len(), 4, 4) {
				return;
			}

			let project_id = args[2].parse::<i64>().unwrap();

			commands::edit_project(project_id, args[3].as_str());
		},

		commands::DELETE_PROJECT_NAME => {
			if argcount_check(args.len(), 3, 4) {
				return;
			}

			let project_id = args[2].parse::<i64>().unwrap();
			let mut purge: bool = false;

			if args.len() > 3 {
				if args[3].to_lowercase() == "true" {
					purge = true;
				}
			}

			commands::delete_project(project_id, purge);
		},

		commands::RECORD_NAME | commands::RECORD_ABBR => {
			if argcount_check(args.len(), 3, 3) {
				return;
			}

			let project_id = args[2].parse::<i64>().unwrap();

			commands::record(project_id);
		},

		commands::STATUS_NAME => {
			if argcount_check(args.len(), 2, 2) {
				return;
			}

			commands::status();
		},

		commands::STOP_NAME | commands::STOP_ABBR => {
			if argcount_check(args.len(), 3, 3) {
				return;
			}

			commands::stop(args[2].as_str());
		},

		commands::EDIT_RECORD_PROJECT_NAME | commands::EDIT_RECORD_PROJECT_ABBR => {
			if argcount_check(args.len(), 4, 4) {
				return;
			}

			let record_id = args[2].parse::<i64>().unwrap();
			let project_id = args[3].parse::<i64>().unwrap();

			commands::edit_record_project(record_id, project_id);
		},

		commands::EDIT_RECORD_BEGIN_NAME | commands::EDIT_RECORD_BEGIN_ABBR => {
			if argcount_check(args.len(), 8, 8) {
				return;
			}

			let record_id = args[2].parse::<i64>().unwrap();
			let year = args[3].parse::<i64>().unwrap();
			let month = args[4].parse::<i64>().unwrap();
			let day = args[5].parse::<i64>().unwrap();
			let hour = args[6].parse::<i64>().unwrap();
			let minute = args[7].parse::<i64>().unwrap();

			commands::edit_record_begin(record_id, year, month, day, hour, minute);
		},

		commands::EDIT_RECORD_END_NAME | commands::EDIT_RECORD_END_ABBR => {
			if argcount_check(args.len(), 8, 8) {
				return;
			}

			let record_id = args[2].parse::<i64>().unwrap();
			let year = args[3].parse::<i64>().unwrap();
			let month = args[4].parse::<i64>().unwrap();
			let day = args[5].parse::<i64>().unwrap();
			let hour = args[6].parse::<i64>().unwrap();
			let minute = args[7].parse::<i64>().unwrap();

			commands::edit_record_end(record_id, year, month, day, hour, minute);
		},

		commands::EDIT_RECORD_DESCRIPTION_NAME | commands::EDIT_RECORD_DESCRIPTION_ABBR => {
			if argcount_check(args.len(), 4, 4) {
				return;
			}

			let record_id = args[2].parse::<i64>().unwrap();

			commands::edit_record_description(record_id, args[3].as_str());
		},

		commands::TRANSFER_PROJECT_RECORDS_NAME => {
			if argcount_check(args.len(), 4, 4) {
				return;
			}

			let src_prj_id = args[2].parse::<i64>().unwrap();
			let dest_prj_id = args[3].parse::<i64>().unwrap();

			commands::transfer_project_records(src_prj_id, dest_prj_id);
		},

		commands::SHOW_WEEK_NAME | commands::SHOW_WEEK_ABBR => {
			if argcount_check(args.len(), 5, 5) {
				return;
			}

			let year: i32;
			let month: u32;
			let day: u32;
			
			year = args[2].parse::<i32>().unwrap();
			month = args[3].parse::<u32>().unwrap();
			day = args[4].parse::<u32>().unwrap();

			commands::show_week(year, month, day);
		},
		
		_ => println!("Command not recognised."),
	}
}
