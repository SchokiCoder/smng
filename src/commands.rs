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

use super::app;

pub const HELP_INFO: &str = "prints all help messages";
pub const HELP_NAME: &str = "help";
pub const HELP_ABBR: &str = "h";

pub const ADD_PROJECT_INFO: &str = "add a project";
pub const ADD_PROJECT_NAME: &str = "add-project";
pub const ADD_PROJECT_ABBR: &str = "ap";
pub const ADD_PROJECT_ARGS: &str = "project_name";

pub const SHOW_PROJECTS_INFO: &str = "show projects";
pub const SHOW_PROJECTS_NAME: &str = "show-projects";
pub const SHOW_PROJECTS_ABBR: &str = "sp";

pub const EDIT_PROJECT_INFO: &str = "edit project name";
pub const EDIT_PROJECT_NAME: &str = "edit-project";
pub const EDIT_PROJECT_ABBR: &str = "ep";
pub const EDIT_PROJECT_ARGS: &str = "project_id project_name";

pub const DELETE_PROJECT_INFO: &str = "delete a project and if wished purge all records";
pub const DELETE_PROJECT_NAME: &str = "delete-project";
pub const DELETE_PROJECT_ARGS: &str = "project_id [purge]";

pub const RECORD_INFO: &str = "record work time on given project";
pub const RECORD_NAME: &str = "record";
pub const RECORD_ABBR: &str = "r";
pub const RECORD_ARGS: &str = "project_id";

pub const STATUS_INFO: &str = "show current work status";
pub const STATUS_NAME: &str = "status";

pub const STOP_INFO: &str = "stop recording work time";
pub const STOP_NAME: &str = "stop";
pub const STOP_ABBR: &str = "s";
pub const STOP_ARGS: &str = "description";

pub const EDIT_RECORD_PROJECT_INFO: &str = "edit record's project";
pub const EDIT_RECORD_PROJECT_NAME: &str = "edit-record-project";
pub const EDIT_RECORD_PROJECT_ABBR: &str = "erp";
pub const EDIT_RECORD_PROJECT_ARGS: &str = "record_id project_id";

pub const EDIT_RECORD_BEGIN_INFO: &str = "edit record's begin";
pub const EDIT_RECORD_BEGIN_NAME: &str = "edit-record-begin";
pub const EDIT_RECORD_BEGIN_ABBR: &str = "erb";
pub const EDIT_RECORD_BEGIN_ARGS: &str = "record_id year month day hour minute";

pub const EDIT_RECORD_END_INFO: &str = "edit record's end";
pub const EDIT_RECORD_END_NAME: &str = "edit-record-end";
pub const EDIT_RECORD_END_ABBR: &str = "ere";
pub const EDIT_RECORD_END_ARGS: &str = "record_id year month day hour minute";

pub const EDIT_RECORD_DESCRIPTION_INFO: &str = "edit record's description";
pub const EDIT_RECORD_DESCRIPTION_NAME: &str = "edit-record-description";
pub const EDIT_RECORD_DESCRIPTION_ABBR: &str = "erd";
pub const EDIT_RECORD_DESCRIPTION_ARGS: &str = "record_id description";

pub const DELETE_RECORD_INFO: &str = "delete given record";
pub const DELETE_RECORD_NAME: &str = "delete-record";
pub const DELETE_RECORD_ARGS: &str = "record_id";

pub const TRANSFER_PROJECT_RECORDS_INFO: &str = "transfer all records from one project to another";
pub const TRANSFER_PROJECT_RECORDS_NAME: &str = "transfer-project-records";
pub const TRANSFER_PROJECT_RECORDS_ARGS: &str = "source_project_id destination_project_id";

pub const SHOW_WEEK_INFO: &str = "show records of a certain week or current";
pub const SHOW_WEEK_NAME: &str = "show-week";
pub const SHOW_WEEK_ABBR: &str = "sw";
pub const SHOW_WEEK_ARGS: &str = "[year month day]";

pub const SHOW_MONTH_INFO: &str = "show records of a certain month or current";
pub const SHOW_MONTH_NAME: &str = "show-month";
pub const SHOW_MONTH_ABBR: &str = "sm";
pub const SHOW_MONTH_ARGS: &str = "[year month]";

fn print_cmd_help(info: &str, name: &str, abbr: Option<&str>, args: Option<&str>) {
	println!("  {}:", info);
	print!("  {}", name);

	if abbr.is_some() {
		print!(", {}", abbr.unwrap());
	}

	if args.is_some() {
		print!(" | {}", args.unwrap());
	}

	println!("\n");	
}

fn database_open() -> sqlite::Connection {
	let mut db_empty: bool = false;
	let mut path = String::from(std::env::var("HOME").unwrap());
	path.push_str("/.");
	path.push_str(app::NAME);
	path.push_str("/worktimes.db");

	// if db doesn't exist, flag
	if std::fs::metadata(path.as_str()).is_ok() == false {
		db_empty = true;
	}

	// open db
	let db = {
		let temp = sqlite::open(path.as_str());

		// if connection ok, set db
		if temp.is_ok() {
			temp.unwrap()
		}

		// else panic
		else {
			panic!("Connection to database \"{}\" failed.", path.as_str());
		}
	};

	// activate foreign keys
	db
		.execute("PRAGMA foreign_keys = ON;")
		.unwrap();

	// if flagged, create database
	if db_empty {
		println!("WARNING: Database does not exist and will be newly created at \"{}\".", path.as_str());
		
		db
			.execute(
				"CREATE TABLE tbl_projects( \
				 project_id INTEGER PRIMARY KEY, \
				 project_name VARCHAR(32) NOT NULL UNIQUE);")
			.unwrap();

		db
			.execute(
				"CREATE TABLE tbl_work_records( \
				 work_record_id INTEGER PRIMARY KEY, \
				 project_id INTEGER NOT NULL REFERENCES tbl_projects(project_id), \
				 begin INTEGER NOT NULL, \
				 end INTEGER CHECK(end > begin), \
				 description VARCHAR(50));")
			.unwrap();

		db
			.execute(
				"CREATE INDEX idx_work_record_id ON tbl_work_records(work_record_id);")
			.unwrap();

		db
			.execute(
				"CREATE INDEX idx_project_id ON tbl_projects(project_id);")
			.unwrap();

		db
			.execute(
				"CREATE INDEX idx_begin ON tbl_work_records(begin);")
			.unwrap();

		db
			.execute(
				"CREATE INDEX idx_end ON tbl_work_records(end);")
			.unwrap();
	}

	return db;
}

pub fn help() {
	println!("Usage:");
	println!("{} [COMMAND] [ARGS]", app::NAME);
	println!("");

	print_cmd_help(HELP_INFO, HELP_NAME, Some(HELP_ABBR), None);
	print_cmd_help(
		ADD_PROJECT_INFO,
		ADD_PROJECT_NAME,
		Some(ADD_PROJECT_ABBR),
		Some(ADD_PROJECT_ARGS));
	print_cmd_help(
		SHOW_PROJECTS_INFO,
		SHOW_PROJECTS_NAME,
		Some(SHOW_PROJECTS_ABBR),
		None);
	print_cmd_help(
		EDIT_PROJECT_INFO,
		EDIT_PROJECT_NAME,
		Some(EDIT_PROJECT_ABBR),
		Some(EDIT_PROJECT_ARGS));
	print_cmd_help(
		DELETE_PROJECT_INFO,
		DELETE_PROJECT_NAME,
		None,
		Some(DELETE_PROJECT_ARGS));
	print_cmd_help(
		RECORD_INFO,
		RECORD_NAME,
		Some(RECORD_ABBR),
		Some(RECORD_ARGS));
	print_cmd_help(STATUS_INFO, STATUS_NAME, None, None);
	print_cmd_help(STOP_INFO, STOP_NAME, Some(STOP_ABBR), Some(STOP_ARGS));
	print_cmd_help(
		EDIT_RECORD_PROJECT_INFO,
		EDIT_RECORD_PROJECT_NAME,
		Some(EDIT_RECORD_PROJECT_ABBR),
		Some(EDIT_RECORD_PROJECT_ARGS));
	print_cmd_help(
		EDIT_RECORD_BEGIN_INFO,
		EDIT_RECORD_BEGIN_NAME,
		Some(EDIT_RECORD_BEGIN_ABBR),
		Some(EDIT_RECORD_BEGIN_ARGS));
	print_cmd_help(
		EDIT_RECORD_END_INFO,
		EDIT_RECORD_END_NAME,
		Some(EDIT_RECORD_END_ABBR),
		Some(EDIT_RECORD_END_ARGS));
	print_cmd_help(
		EDIT_RECORD_DESCRIPTION_INFO,
		EDIT_RECORD_DESCRIPTION_NAME,
		Some(EDIT_RECORD_DESCRIPTION_ABBR),
		Some(EDIT_RECORD_DESCRIPTION_ARGS));
	print_cmd_help(
		DELETE_RECORD_INFO,
		DELETE_RECORD_NAME,
		None,
		Some(DELETE_RECORD_ARGS));
	print_cmd_help(
		TRANSFER_PROJECT_RECORDS_INFO,
		TRANSFER_PROJECT_RECORDS_NAME,
		None,
		Some(TRANSFER_PROJECT_RECORDS_ARGS));
	print_cmd_help(
		SHOW_WEEK_INFO,
		SHOW_WEEK_NAME,
		Some(SHOW_WEEK_ABBR),
		Some(SHOW_WEEK_ARGS));
	print_cmd_help(
		SHOW_MONTH_INFO,
		SHOW_MONTH_NAME,
		Some(SHOW_MONTH_ABBR),
		Some(SHOW_MONTH_ARGS));

	println!("You can also use negative id's to count from the other end.");
}

pub fn add_project(project_name: &str) {
	let db = database_open();

	let mut stmt = db
		.prepare(
			"INSERT INTO tbl_projects(project_name)\n\
		 	 VALUES (?);")
		.unwrap();

	stmt.bind(1, project_name).unwrap();
	stmt.next().unwrap();

	println!("Project \"{}\" added.", project_name);
}

pub fn show_projects() {
	let db = database_open();

	let mut stmt = db
		.prepare(
			"SELECT project_id, project_name\n\
			 FROM tbl_projects;")
		.unwrap();

	println!("{:9} | {}", "id", "project name");

	while let sqlite::State::Row = stmt.next().unwrap() {
		println!(
			"{:9} | {}",
			stmt.read::<String>(0).unwrap(),
			stmt.read::<String>(1).unwrap());
	}
}

pub fn edit_project(project_id: i64, project_name: &str) {
	let db = database_open();

	let mut stmt = db
		.prepare(
			"UPDATE tbl_projects\n\
			 SET project_name = ?\n\
			 WHERE project_id = ?;")
		.unwrap();

	stmt.bind(1, project_name).unwrap();
	stmt.bind(2, project_id).unwrap();
	stmt.next().unwrap();

	println!("Project ({}) name set to \"{}\".", project_id, project_name);
}

pub fn delete_project(project_id: i64, purge: bool) {
	let db = database_open();

	// if wished delete all records from this project
	if purge {
		let mut stmt = db
			.prepare(
				"DELETE FROM tbl_work_records\n\
			 	 WHERE work_record_id = ?;")
			.unwrap();

		stmt.bind(1, project_id).unwrap();
		while stmt.next().unwrap() != sqlite::State::Done {}
	}

	let mut stmt = db
		.prepare(
			"DELETE FROM tbl_projects\n\
		 	 WHERE project_id = ?;")
		.unwrap();

	stmt.bind(1, project_id).unwrap();
	stmt.next().unwrap();

	if purge {
		println!("Project ({}) and its records deleted.", project_id);
	}
	else {
		println!("Project ({}) deleted.", project_id);
	}
}

struct RecordState {
	id: i64,
	state: i64,
}

impl RecordState {
	pub fn last() -> RecordState {
		let db = database_open();

		let mut stmt = db
			.prepare(
				"SELECT work_record_id, \
				 (CASE WHEN end IS NULL THEN 0 ELSE 1 END) as record_complete\n \
				 FROM tbl_work_records\n \
				 ORDER BY work_record_id DESC LIMIT 1;")
			.unwrap();

		stmt.next().unwrap();

		return RecordState {
			id: stmt.read::<i64>(0).unwrap(),
			state: stmt.read::<i64>(1).unwrap(),
		};
	}
}

pub fn record(project_id: i64) {
	// if last record is not done, stop
	let rec_state = RecordState::last();

	if rec_state.state == 0 {
		println!("ERROR: Last record ({}) is not yet done.", rec_state.id);
		return;
	}

	// exec
	let db = database_open();

	let mut stmt = db
		.prepare(
			"INSERT INTO tbl_work_records(project_id, begin)\n \
	 		 VALUES(?, strftime('%s'));")
	 	.unwrap();

	stmt.bind(1, project_id).unwrap();
	stmt.next().unwrap();

	println!("Record for project ({}) started.", project_id);
}

pub fn status() {
	let rec_state = RecordState::last();

	let state_str: &str;

	if rec_state.state == 1 {
		state_str = "";
	}
	else {
		state_str = "NOT ";
	}

	println!("Last record ({}) is {}done.", rec_state.id, state_str);
}

pub fn stop(description: &str) {
	// if last record is done, stop
	let rec_state = RecordState::last();

	if rec_state.state == 1 {
		println!("ERROR: Last record ({}) is already done.", rec_state.id);
		return;
	}

	//exec
	let db = database_open();

	let mut stmt = db
		.prepare(
			"UPDATE tbl_work_records\n\
		 	 SET end = strftime('%s'), description = ?\n\
		 	 WHERE work_record_id = (SELECT MAX(work_record_id) FROM tbl_work_records);")
		.unwrap();

	stmt.bind(1, description).unwrap();
	stmt.next().unwrap();

	println!("Record stopped with descritpion \"{}\".", description);
}

pub fn edit_record_project(record_id: i64, project_id: i64) {
	let db = database_open();

	let mut stmt = db
		.prepare(
			"UPDATE tbl_work_records\n\
			 SET project_id = ?\n\
			 WHERE work_record_id = ?;")
		.unwrap();

	stmt.bind(1, project_id).unwrap();
	stmt.bind(2, record_id).unwrap();
	stmt.next().unwrap();

	println!("Record ({}) project set to ({}).", record_id, project_id);
}

fn edit_record_time(begin: bool, record_id: i64, year: i64, month: i64, day: i64,
                    hour: i64, minute: i64) {
	let db = database_open();

	let mut sql = String::from("UPDATE tbl_work_records\nSET ");

	if begin == true {
		sql.push_str("begin");
	}
	else {
		sql.push_str("end");
	}

	sql.push_str(" = strftime('%s', printf('%04i-%02i-%02i %02i:%02i:00', ?, ?, ?, ?, ?))");
	sql.push_str("WHERE work_record_id = ?;");

	let mut stmt = db
		.prepare(sql.as_str())
	 	.unwrap();
	
	stmt.bind(1, year).unwrap();
	stmt.bind(2, month).unwrap();
	stmt.bind(3, day).unwrap();
	stmt.bind(4, hour).unwrap();
	stmt.bind(5, minute).unwrap();
	stmt.bind(6, record_id).unwrap();
	stmt.next().unwrap();
}

pub fn edit_record_begin(record_id: i64, year: i64, month: i64, day: i64,
                         hour: i64, minute: i64) {
	edit_record_time(true, record_id, year, month, day, hour, minute);

	println!("Record ({}) begin set to {}-{}-{} {}:{}.",
		record_id,
		year, month, day,
		hour, minute);
}

pub fn edit_record_end(record_id: i64, year: i64, month: i64, day: i64,
                       hour: i64, minute: i64) {
	edit_record_time(false, record_id, year, month, day, hour, minute);

	println!("Record ({}) end set to {}-{}-{} {}:{}.",
		record_id,
		year, month, day,
		hour, minute);
}

pub fn edit_record_description(record_id: i64, description: &str) {
	let db = database_open();

	let mut stmt = db
		.prepare(
			"UPDATE tbl_work_records\n \
			 SET description = ?\n \
			 WHERE work_record_id = ?;")
		.unwrap();

	stmt.bind(1, description).unwrap();
	stmt.bind(2, record_id).unwrap();
	stmt.next().unwrap();

	println!("Record ({}) description set to \"{}\".", record_id, description);
}

pub fn transfer_project_records(src_project_id: i64, dest_project_id: i64) {
	let db = database_open();

	let mut stmt = db
		.prepare(
			"UPDATE tbl_work_records\n \
			 SET project_id = ? \
			 WHERE project_id = ?;")
		.unwrap();

	stmt.bind(1, dest_project_id).unwrap();
	stmt.bind(2, src_project_id).unwrap();

	while sqlite::State::Row == stmt.next().unwrap() {}

	println!("Records of project ({}) moved to project ({}).",
		src_project_id, dest_project_id);
}

fn show_record(stmt: &sqlite::Statement, win_width: usize) -> i64 {
	// format seconds in worktime
	let seconds = stmt.read::<i64>(5).unwrap();
	let minutes: u32 = seconds as u32 / 60;
	let hours: u32 = minutes / 60;

	let worktime = Utc
		.ymd(1970, 1, 1)
		.and_hms(hours, minutes - hours * 60, 0)
		.format("%H:%M")
		.to_string();

	// print record
	print!(
		"{:9} | {:8} | {:8} | {:5} | {:9} | ",
		stmt.read::<i64>(0).unwrap(),
		stmt.read::<String>(2).unwrap(),
		stmt.read::<String>(4).unwrap(),
		worktime,
		stmt.read::<i64>(6).unwrap());

	// print desc
	let desc = stmt.read::<String>(7).unwrap();
	let mut i = 0;
	let mut pos = 0;

	while i < desc.len() {
		if pos + 55 > win_width {
			print!("\n{:9} | {:8} | {:8} | {:5} | {:9} | ",
				"", "", "", "", "");
			pos = 0;
		}

		print!("{}", desc.chars().nth(i).unwrap());

		i += 1;
		pos += 1;
	}

	println!("");

	return seconds;
}

fn show_records(ts_begin: i64, ts_end: i64) {
	// get window width
	let win_width = term_size::dimensions_stdout().unwrap().0;

	// execute sql
	let db = database_open();

	let mut stmt = db.prepare(
		"SELECT work_record_id, \
		 strftime('%d', begin, 'unixepoch') as begin_day, \
		 strftime('%H:%M', begin, 'unixepoch') as begin_time, \
		 strftime('%d', end, 'unixepoch') as end_day, \
		 strftime('%H:%M', end, 'unixepoch') as end_time, \
		 end - begin AS worktime, \
		 project_id, description\n \
		 FROM tbl_work_records\n \
		 WHERE begin > ? AND end < ?;")
		.unwrap();

	stmt.bind(1, ts_begin).unwrap();
	stmt.bind(2, ts_end).unwrap();

	// print header
	println!("{:9} | {:8} | {:8} | {:5} | {:9} | {}",
		"id", "begin", "end", "time", "project", "description");

	let mut sum_seconds: u32 = 0;
	let mut pre_day: i64;
	let mut cur_day: i64 = 0;
	let mut day_seconds: u32 = 0;

	// print first record
	// (because the if (cur_day change) would needlessly print day_worktime)
	if stmt.next().unwrap() == sqlite::State::Row {
		cur_day = stmt.read::<i64>(1).unwrap();
		println!("- day {:3} -", cur_day);
		
		let seconds = show_record(&stmt, win_width);
		
		sum_seconds += seconds as u32;
		day_seconds += seconds as u32;
	}

	// other records
	while stmt.next().unwrap() == sqlite::State::Row {
		// if current day changes
		pre_day = cur_day;
		cur_day = stmt.read::<i64>(1).unwrap();

		if cur_day != pre_day {
			
			// print new day line
			let minutes: u32 = day_seconds / 60;
			let hours: u32 = minutes / 60;
			
			let worktime = Utc
				.ymd(1970, 1, 1)
				.and_hms(hours, minutes - hours * 60, 0)
				.format("%H:%M")
				.to_string();
		
			println!("{:32}- {:5} -", "", worktime);
			println!("- day {:3} -", cur_day);

			// reset day worktime
			day_seconds = 0;
		}

		let seconds = show_record(&stmt, win_width);
		
		sum_seconds += seconds as u32;
		day_seconds += seconds as u32;
	}

	// last day worktime
	let minutes: u32 = day_seconds / 60;
	let hours: u32 = minutes / 60;
			
	let worktime = Utc
		.ymd(1970, 1, 1)
		.and_hms(hours, minutes - hours * 60, 0)
		.format("%H:%M")
		.to_string();
		
	println!("{:32}- {:5} -", "", worktime);

	// summarized worktime
	let minutes: u32 = sum_seconds / 60;
	let hours: u32 = minutes / 60;
	let worktime = Utc
			.ymd(1970, 1, 1)
			.and_hms(hours, minutes - hours * 60, 0)
			.format("%H:%M")
			.to_string();
			
	println!("Summarized worktime: {}.", worktime);
}

use chrono::prelude::*;

const DAY_SECONDS: u32 = 60 * 60 * 24;

struct WeekBeginAndEnd {
	begin: i64,
	end: i64,
}

impl WeekBeginAndEnd {
	pub fn from_date(date: Date<Local>) -> WeekBeginAndEnd {
		let weekday = date.weekday().num_days_from_sunday() as i64;

		let result = WeekBeginAndEnd {
			begin: date.and_hms(0, 0, 0).timestamp() - (weekday * DAY_SECONDS as i64),
			end: date.and_hms(23, 59, 59).timestamp() + ((7 - weekday) * DAY_SECONDS as i64),
		};

		return result;
	}
}

pub fn show_week_cur() {
	// print
	let week = WeekBeginAndEnd::from_date(Local::today());
	show_records(week.begin, week.end);
}

pub fn show_week(year: i32, month: u32, day: u32) {
	// print
	let week = WeekBeginAndEnd::from_date(Local.ymd(year, month, day));
	show_records(week.begin, week.end);
}

pub fn show_month(year: i32, month: u32) {
	// find begin and end of month
	let ts_begin = Utc
		.ymd(year, month, 1)
		.and_hms(0, 0, 0)
		.timestamp();
		
	let ts_temp = Utc
		.ymd(year, month, 28)
		.and_hms(0, 0, 0)
		.timestamp();
		
	let mut ts_end: i64 = 0;

	for i in 1..4 {
		if Utc.timestamp(ts_temp + (i * DAY_SECONDS as i64), 0).month() !=
		   Utc.timestamp(ts_temp, 0).month() {
			ts_end = ts_temp + ((i - 1) * DAY_SECONDS as i64);
		}
	}

	// print
	show_records(ts_begin, ts_end);
}
