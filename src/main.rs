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
		println!("Usage: {} command [arguments]:", env!("CARGO_PKG_NAME"));
		println!("Try '{} {}' for more information.",
			env!("CARGO_PKG_NAME"),
			commands::HELP_NAME);
		return;
	}
	
	match args[1].as_str() {
		commands::HELP_NAME | commands::HELP_ABBR => commands::help(),

		commands::ABOUT_NAME | commands::ABOUT_ABBR => commands::about(),

		commands::ADD_PROJECT_NAME | commands::ADD_PROJECT_ABBR => {
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::ADD_PROJECT_INFO,
					commands::ADD_PROJECT_NAME,
					Some(commands::ADD_PROJECT_ABBR),
					Some(commands::ADD_PROJECT_ARGS));
				return;
			}
			
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
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::EDIT_PROJECT_INFO,
					commands::EDIT_PROJECT_NAME,
					Some(commands::EDIT_PROJECT_ABBR),
					Some(commands::EDIT_PROJECT_ARGS));
				return;
			}
			
			if argcount_check(args.len(), 4, 4) {
				return;
			}

			let project_id = args[2].parse::<i64>().unwrap();

			commands::edit_project(project_id, args[3].as_str());
		},

		commands::DELETE_PROJECT_NAME => {
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::DELETE_PROJECT_INFO,
					commands::DELETE_PROJECT_NAME,
					None,
					Some(commands::DELETE_PROJECT_ARGS));
				return;
			}
			
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
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::RECORD_INFO,
					commands::RECORD_NAME,
					Some(commands::RECORD_ABBR),
					Some(commands::RECORD_ARGS));
				return;
			}
			
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
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::STOP_INFO,
					commands::STOP_NAME,
					Some(commands::STOP_ABBR),
					Some(commands::STOP_ARGS));
				return;
			}

			if argcount_check(args.len(), 3, 3) {
				return;
			}

			commands::stop(args[2].as_str());
		},

		commands::ADD_RECORD_NAME | commands::ADD_RECORD_ABBR => {
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::ADD_RECORD_INFO,
					commands::ADD_RECORD_NAME,
					Some(commands::ADD_RECORD_ABBR),
					Some(commands::ADD_RECORD_ARGS));
				return;
			}

			if argcount_check(args.len(), 14, 14) {
				return;
			}

			let project_id = args[2].parse::<i64>().unwrap();
			let b_year = args[4].parse::<i64>().unwrap();
			let b_month = args[5].parse::<i64>().unwrap();
			let b_day = args[6].parse::<i64>().unwrap();
			let b_hour = args[7].parse::<i64>().unwrap();
			let b_minute = args[8].parse::<i64>().unwrap();
			let e_year = args[9].parse::<i64>().unwrap();
			let e_month = args[10].parse::<i64>().unwrap();
			let e_day = args[11].parse::<i64>().unwrap();
			let e_hour = args[12].parse::<i64>().unwrap();
			let e_minute = args[13].parse::<i64>().unwrap();

			commands::add_record(
				project_id, args[3].as_str(),
				b_year, b_month, b_day, b_hour, b_minute,
				e_year, e_month, e_day, e_hour, e_minute);
		},

		commands::EDIT_RECORD_PROJECT_NAME |
		commands::EDIT_RECORD_PROJECT_ABBR => {
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::EDIT_RECORD_PROJECT_INFO,
					commands::EDIT_RECORD_PROJECT_NAME,
					Some(commands::EDIT_RECORD_PROJECT_ABBR),
					Some(commands::EDIT_RECORD_PROJECT_ARGS));
				return;
			}

			if argcount_check(args.len(), 4, 4) {
				return;
			}

			let record_id = args[2].parse::<i64>().unwrap();
			let project_id = args[3].parse::<i64>().unwrap();

			commands::edit_record_project(record_id, project_id);
		},

		commands::EDIT_RECORD_BEGIN_NAME | commands::EDIT_RECORD_BEGIN_ABBR => {
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::EDIT_RECORD_BEGIN_INFO,
					commands::EDIT_RECORD_BEGIN_NAME,
					Some(commands::EDIT_RECORD_BEGIN_ABBR),
					Some(commands::EDIT_RECORD_BEGIN_ARGS));
				return;
			}

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
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::EDIT_RECORD_END_INFO,
					commands::EDIT_RECORD_END_NAME,
					Some(commands::EDIT_RECORD_END_ABBR),
					Some(commands::EDIT_RECORD_END_ARGS));
				return;
			}

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

		commands::EDIT_RECORD_DESCRIPTION_NAME |
		commands::EDIT_RECORD_DESCRIPTION_ABBR => {
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::EDIT_RECORD_DESCRIPTION_INFO,
					commands::EDIT_RECORD_DESCRIPTION_NAME,
					Some(commands::EDIT_RECORD_DESCRIPTION_ABBR),
					Some(commands::EDIT_RECORD_DESCRIPTION_ARGS));
				return;
			}

			if argcount_check(args.len(), 4, 4) {
				return;
			}

			let record_id = args[2].parse::<i64>().unwrap();

			commands::edit_record_description(record_id, args[3].as_str());
		},

		commands::DELETE_RECORD_NAME => {
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::DELETE_RECORD_INFO,
					commands::DELETE_RECORD_NAME,
					None,
					Some(commands::DELETE_RECORD_ARGS));
				return;
			}

			if argcount_check(args.len(), 3, 3) {
				return;
			}

			let record_id = args[2].parse::<i64>().unwrap();

			commands::delete_record(record_id);
		}

		commands::TRANSFER_PROJECT_RECORDS_NAME => {
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::TRANSFER_PROJECT_RECORDS_INFO,
					commands::TRANSFER_PROJECT_RECORDS_NAME,
					None,
					Some(commands::TRANSFER_PROJECT_RECORDS_ARGS));
				return;
			}

			if argcount_check(args.len(), 4, 4) {
				return;
			}

			let src_prj_id = args[2].parse::<i64>().unwrap();
			let dest_prj_id = args[3].parse::<i64>().unwrap();

			commands::transfer_project_records(src_prj_id, dest_prj_id);
		},

		commands::SWAP_PROJECT_RECORDS_NAME => {
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::SWAP_PROJECT_RECORDS_INFO,
					commands::SWAP_PROJECT_RECORDS_NAME,
					None,
					Some(commands::SWAP_PROJECT_RECORDS_ARGS));
				return;
			}
			
			if argcount_check(args.len(), 4, 4) {
				return;
			}
			
			let project_id_a = args[2].parse::<i64>().unwrap();
			let project_id_b = args[3].parse::<i64>().unwrap();
			
			commands::swap_project_records(project_id_a, project_id_b);
		},

		commands::SHOW_WEEK_NAME | commands::SHOW_WEEK_ABBR => {
			if argcount_check(args.len(), 2, 5) {
				return;
			}

			if args.len() > 2 {
				let year = args[2].parse::<i32>().unwrap();
				let month = args[3].parse::<u32>().unwrap();
				let day = args[4].parse::<u32>().unwrap();

				commands::show_week(year, month, day);
			}
			else {
				commands::show_week_cur();
			}
		},

		commands::SHOW_MONTH_NAME | commands::SHOW_MONTH_ABBR => {
			if argcount_check(args.len(), 2, 4) {
				return;
			}

			if args.len() > 2 {
				let year = args[2].parse::<i32>().unwrap();
				let month = args[3].parse::<u32>().unwrap();

				commands::show_month(year, month);
			}
			else {
				commands::show_month_cur();
			}
		},
		
		commands::SHOW_PROJECT_RECORDS_NAME |
		commands::SHOW_PROJECT_RECORDS_ABBR => {
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::SHOW_PROJECT_RECORDS_INFO,
					commands::SHOW_PROJECT_RECORDS_NAME,
					Some(commands::SHOW_PROJECT_RECORDS_ABBR),
					Some(commands::SHOW_PROJECT_RECORDS_ARGS));
				return;
			}
			
			if argcount_check(args.len(), 3, 3) {
				return;
			}
			
			let project_id = args[2].parse::<i64>().unwrap();
			
			commands::show_project_records(project_id);
		},

		commands::MERGE_DB_NAME | commands::MERGE_DB_ABBR => {
			if args.len() == 2 {
				commands::print_cmd_help(
					commands::MERGE_DB_INFO,
					commands::MERGE_DB_NAME,
					Some(commands::MERGE_DB_ABBR),
					Some(commands::MERGE_DB_ARGS));
				return;
			}

			if argcount_check(args.len(), 4, 4) {
				return;
			}

			commands::merge_db(&args[2], &args[3]);
		},
		
		commands::SHOW_ETC_PATH_NAME | commands::SHOW_ETC_PATH_ABBR => {
			if argcount_check(args.len(), 2, 2) {
				return;
			}
			
			commands::show_etc_path();
		}
		
		commands::SHOW_DB_PATH_NAME | commands::SHOW_DB_PATH_ABBR => {
			if argcount_check(args.len(), 2, 2) {
				return;
			}
			
			commands::show_db_path();
		}
		
		_ => println!("Command not recognised."),
	}
}
