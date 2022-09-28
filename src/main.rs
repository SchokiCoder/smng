/*
	"SchokiManager" in short "smng"
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

pub mod cmd;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	// if not args given, print usage help and end
	if args.len() < 2 {
		println!("Usage: {} command [arguments]:", env!("CARGO_PKG_NAME"));
		println!("Try '{} {}' for more information.",
			env!("CARGO_PKG_NAME"),
			cmd::HELP.name);
		return;
	}
	
	let cur_cmd = cmd::Command{
		info: "",
		name: args[1].as_str(),
		abbr: Some(args[1].as_str()),
		args: None,
		min_args: 0,
		max_args: 0,
		args_all_or_none: false,
	};
	let cur_args = &args[2..];
	
	if cur_cmd == cmd::HELP {
		if cmd::HELP.arg_count_pass(cur_args.len()) {
			cmd::help();
		}
		else {
			return;
		}
	}
	
	else if cur_cmd == cmd::ABOUT {
		if cmd::ABOUT.arg_count_pass(cur_args.len()) {
			cmd::about();
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::ADD_PROJECT {
		if cmd::ADD_PROJECT.arg_count_pass(cur_args.len()) {
			cmd::add_project(cur_args[0].as_str());
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::SHOW_PROJECTS {
		if cmd::SHOW_PROJECTS.arg_count_pass(cur_args.len()) {
			cmd::show_projects();
		}
		else {
			return
		}
	}

	else if cur_cmd == cmd::EDIT_PROJECT {
		if cmd::EDIT_PROJECT.arg_count_pass(cur_args.len()) {
			let project_id = cur_args[0].parse::<i64>().unwrap();

			cmd::edit_project(project_id, cur_args[1].as_str());
		}
		else {
			return;
		}
	}
		
	else if cur_cmd == cmd::ARCHIVE_PROJECT {
		if cmd::ARCHIVE_PROJECT.arg_count_pass(cur_args.len()) {
			let project_id = cur_args[0].parse::<i64>().unwrap();
			
			cmd::archive_project(project_id);
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::DELETE_PROJECT {
		if cmd::DELETE_PROJECT.arg_count_pass(cur_args.len()) {
			let project_id = cur_args[0].parse::<i64>().unwrap();
			let mut purge: bool = false;

			if cur_args.len() > 1 {
				if cur_args[1].to_lowercase() == "true" {
					purge = true;
				}
			}

			cmd::delete_project(project_id, purge);
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::RECORD {
		if cmd::RECORD.arg_count_pass(cur_args.len()) {
			let project_id = cur_args[0].parse::<i64>().unwrap();

			cmd::record(project_id);
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::STATUS {
		if cmd::STATUS.arg_count_pass(cur_args.len()) {
			cmd::status();
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::STOP {
		if cmd::STOP.arg_count_pass(cur_args.len()) {
			cmd::stop(cur_args[0].as_str());
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::ADD_RECORD {
		if cmd::ADD_RECORD.arg_count_pass(cur_args.len()) {
			let project_id = cur_args[0].parse::<i64>().unwrap();
			let b_year = cur_args[2].parse::<i64>().unwrap();
			let b_month = cur_args[3].parse::<i64>().unwrap();
			let b_day = cur_args[4].parse::<i64>().unwrap();
			let b_hour = cur_args[5].parse::<i64>().unwrap();
			let b_minute = cur_args[6].parse::<i64>().unwrap();
			let e_year = cur_args[7].parse::<i64>().unwrap();
			let e_month = cur_args[8].parse::<i64>().unwrap();
			let e_day = cur_args[9].parse::<i64>().unwrap();
			let e_hour = cur_args[10].parse::<i64>().unwrap();
			let e_minute = cur_args[11].parse::<i64>().unwrap();

			cmd::add_record(
				project_id, cur_args[1].as_str(),
				b_year, b_month, b_day, b_hour, b_minute,
				e_year, e_month, e_day, e_hour, e_minute);
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::EDIT_RECORD_PROJECT {
		if cmd::EDIT_RECORD_PROJECT.arg_count_pass(cur_args.len()) {
			let record_id = cur_args[0].parse::<i64>().unwrap();
			let project_id = cur_args[1].parse::<i64>().unwrap();

			cmd::edit_record_project(record_id, project_id);
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::EDIT_RECORD_BEGIN {
		if cmd::EDIT_RECORD_BEGIN.arg_count_pass(cur_args.len()) {
			let record_id = cur_args[0].parse::<i64>().unwrap();
			let year = cur_args[1].parse::<i64>().unwrap();
			let month = cur_args[2].parse::<i64>().unwrap();
			let day = cur_args[3].parse::<i64>().unwrap();
			let hour = cur_args[4].parse::<i64>().unwrap();
			let minute = cur_args[5].parse::<i64>().unwrap();

			cmd::edit_record_begin(record_id, year, month, day, hour, minute);
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::EDIT_RECORD_END {
		if cmd::EDIT_RECORD_END.arg_count_pass(cur_args.len()) {
			let record_id = cur_args[0].parse::<i64>().unwrap();
			let year = cur_args[1].parse::<i64>().unwrap();
			let month = cur_args[2].parse::<i64>().unwrap();
			let day = cur_args[3].parse::<i64>().unwrap();
			let hour = cur_args[4].parse::<i64>().unwrap();
			let minute = cur_args[5].parse::<i64>().unwrap();

			cmd::edit_record_end(record_id, year, month, day, hour, minute);
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::EDIT_RECORD_DESCRIPTION {
		if cmd::EDIT_RECORD_DESCRIPTION.arg_count_pass(cur_args.len()) {
			let record_id = cur_args[0].parse::<i64>().unwrap();

			cmd::edit_record_description(record_id, cur_args[1].as_str());
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::DELETE_RECORD {
		if cmd::DELETE_RECORD.arg_count_pass(cur_args.len()) {
			let record_id = cur_args[0].parse::<i64>().unwrap();

			cmd::delete_record(record_id);
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::TRANSFER_PROJECT_RECORDS {
		if cmd::TRANSFER_PROJECT_RECORDS.arg_count_pass(cur_args.len()) {
			let src_prj_id = cur_args[0].parse::<i64>().unwrap();
			let dest_prj_id = cur_args[1].parse::<i64>().unwrap();

			cmd::transfer_project_records(src_prj_id, dest_prj_id);
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::SWAP_PROJECT_RECORDS {
		if cmd::SWAP_PROJECT_RECORDS.arg_count_pass(cur_args.len()) {
			let project_id_a = cur_args[0].parse::<i64>().unwrap();
			let project_id_b = cur_args[1].parse::<i64>().unwrap();
			
			cmd::swap_project_records(project_id_a, project_id_b);
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::SHOW_WEEK {
		if cmd::SHOW_WEEK.arg_count_pass(cur_args.len()) {
			if args.len() > 2 {
				let year = cur_args[0].parse::<i32>().unwrap();
				let month = cur_args[1].parse::<u32>().unwrap();
				let day = cur_args[2].parse::<u32>().unwrap();

				cmd::show_week(year, month, day);
			}
			else {
				cmd::show_week_cur();
			}
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::SHOW_MONTH {
		if cmd::SHOW_MONTH.arg_count_pass(cur_args.len()) {
			if args.len() > 2 {
				let year = cur_args[0].parse::<i32>().unwrap();
				let month = cur_args[1].parse::<u32>().unwrap();

				cmd::show_month(year, month);
			}
			else {
				cmd::show_month_cur();
			}
		}
		else {
			return;
		}
	}
		
	else if cur_cmd == cmd::SHOW_PROJECT_RECORDS {
		if cmd::SHOW_PROJECT_RECORDS.arg_count_pass(cur_args.len()) {
			let project_id = cur_args[0].parse::<i64>().unwrap();
			
			cmd::show_project_records(project_id);
		}
		else {
			return;
		}
	}

	else if cur_cmd == cmd::MERGE_DB {
		if cmd::MERGE_DB.arg_count_pass(cur_args.len()) {
			cmd::merge_db(&cur_args[0], &cur_args[1]);
		}
		else {
			return;
		}
	}
		
	else if cur_cmd == cmd::SHOW_ETC_PATH {
		if cmd::SHOW_ETC_PATH.arg_count_pass(cur_args.len()) {
			cmd::show_etc_path();
		}
		else {
			return;
		}
	}
		
	else if cur_cmd == cmd::SHOW_DB_PATH {
		if cmd::SHOW_DB_PATH.arg_count_pass(cur_args.len()) {
			cmd::show_db_path();
		}
		else {
			return;
		}
	}
	
	else {
		println!("Command not recognised.");
	}
}
