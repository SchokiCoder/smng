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

pub struct CommandData {
	short: &'static str,
	name: &'static str,
	abbr: Option<&'static str>,
	args: Option<&'static str>,
}

pub const DATA_HELP: CommandData = CommandData {
	short: "prints all help messages",
	name: "help",
	abbr: Some("h"),
	args: None,
};

pub const DATA_ADD_PROJECT: CommandData = CommandData {
	short: "add a project",
	name: "add-project",
	abbr: Some("ap"),
	args: Some("project_name"),
};

pub const DATA_SHOW_PROJECTS: CommandData = CommandData {
	short: "show projects",
	name: "show-project",
	abbr: Some("sp"),
	args: None,
};

pub const DATA_EDIT_PROJECT: CommandData = CommandData {
	short: "edit project name",
	name: "edit-project",
	abbr: Some("ep"),
	args: Some("project_name"),
};

pub const DATA_DELETE_PROJECT: CommandData = CommandData {
	short: "delete a project and if wished purge all entries",
	name: "delete-project",
	abbr: None,
	args: Some("project_id [purge]"),
};

pub const DATA_RECORD: CommandData = CommandData {
	short: "record work time on given project",
	name: "record",
	abbr: Some("r"),
	args: Some("project_id"),
};

pub const DATA_STATUS: CommandData = CommandData {
	short: "show current work status",
	name: "status",
	abbr: None,
	args: None,
};

pub const DATA_STOP: CommandData = CommandData {
	short: "stop recording work time",
	name: "stop",
	abbr: Some("s"),
	args: Some("description"),
};

pub const DATA_EDIT_RECORD_PROJECT: CommandData = CommandData {
	short: "edit record's project",
	name: "edit-record-project",
	abbr: Some("erp"),
	args: Some("record_id project_id"),
};

pub const DATA_EDIT_RECORD_BEGIN: CommandData = CommandData {
	short: "edit record's begin",
	name: "edit-record-begin",
	abbr: Some("erb"),
	args: Some("record_id year month day hour minute"),
};

pub const DATA_EDIT_RECORD_END: CommandData = CommandData {
	short: "edit record's end",
	name: "edit-record-end",
	abbr: Some("ere"),
	args: Some("record_id year month day hour minute"),
};

pub const DATA_EDIT_RECORD_DESCRIPTION: CommandData = CommandData {
	short: "edit record's description",
	name: "edit-record-description",
	abbr: Some("erd"),
	args: Some("record_id description"),
};

pub const DATA_DELETE_RECORD: CommandData = CommandData {
	short: "delete given record",
	name: "delete-record",
	abbr: None,
	args: Some("record_id"),
};

pub const DATA_TRANSFER_PROJECT_RECORDS: CommandData = CommandData {
	short: "transfer all records from one project to another",
	name: "transfer-project-records",
	abbr: None,
	args: Some("source_project_id destination_project_id"),
};

pub const DATA_SHOW_WEEK: CommandData = CommandData {
	short: "show record's of a certain week or current",
	name: "show-week",
	abbr: Some("sw"),
	args: Some("[year month day]"),
};

pub const DATA_SHOW_MONTH: CommandData = CommandData {
	short: "show work record's of a certain month or current",
	name: "show-month",
	abbr: Some("sm"),
	args: Some("[year month]"),
};

fn print_cmd_help(cmd: CommandData) {
	println!("  {}:", cmd.short);
	print!("  {}", cmd.name);

	if cmd.abbr.is_some() {
		print!(", {}", cmd.abbr.unwrap());
	}

	if cmd.args.is_some() {
		print!(" | {}", cmd.args.unwrap());
	}

	println!("\n");	
}

fn database_open() -> sqlite::Connection {

	// open db
	#[cfg(all(static_db_path))]
	let db = {
		sqlite::open(cfg(static_db_path)).unwrap()
	};

	#[cfg(not(static_db_path))]
	let db = {
		let mut path = String::from(std::env::var("HOME").unwrap());
		path.push_str("/.");
		path.push_str(app::NAME);
		path.push_str("/worktimes.db");

		sqlite::open(path).unwrap()
	};

	// activate foreign keys
	db
		.execute("PRAGMA foreign_keys = ON;")
		.unwrap();

	return db;
}

pub fn help() {
	println!("Usage:");
	println!("{} [COMMAND] [ARGS]", app::NAME);
	println!("");

	print_cmd_help(DATA_HELP);
	print_cmd_help(DATA_ADD_PROJECT);
	print_cmd_help(DATA_SHOW_PROJECTS);
	print_cmd_help(DATA_EDIT_PROJECT);
	print_cmd_help(DATA_DELETE_PROJECT);
	print_cmd_help(DATA_RECORD);
	print_cmd_help(DATA_STATUS);
	print_cmd_help(DATA_STOP);
	print_cmd_help(DATA_EDIT_RECORD_PROJECT);
	print_cmd_help(DATA_EDIT_RECORD_BEGIN);
	print_cmd_help(DATA_EDIT_RECORD_END);
	print_cmd_help(DATA_EDIT_RECORD_DESCRIPTION);
	print_cmd_help(DATA_DELETE_RECORD);
	print_cmd_help(DATA_TRANSFER_PROJECT_RECORDS);
	print_cmd_help(DATA_SHOW_WEEK);
	print_cmd_help(DATA_SHOW_MONTH);

	println!("You can also use negative id's to count from the other end.");
}

pub fn add_project(project_name: String) {
	let db = database_open();

	let mut stmt = db
		.prepare(
			"INSERT INTO tbl_projects(project_name)\n\
		 	 VALUES (?);")
		.unwrap();

	stmt.bind(1, project_name.as_str()).unwrap();
	stmt.next().unwrap();
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

pub fn edit_project(project_id: i64, project_name: String) {
	let db = database_open();

	let mut stmt = db
		.prepare(
			"UPDATE tbl_projects\n\
			 SET project_name = ?\n\
			 WHERE project_id = ?;")
		.unwrap();

	stmt.bind(1, project_name.as_str()).unwrap();
	stmt.bind(2, project_id).unwrap();
	stmt.next().unwrap();
}

pub fn delete_project(project_id: i64, purge: bool) {
	let db = database_open();

	// if wished delete all records from this project
	if purge {
		let mut stmt = db
			.prepare(
				"DELETE FROM tbl_work_records\n\
			 	 WHERE record_id = ?;")
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
}

pub fn record(project_id: i64) {
	let db = database_open();

	let mut stmt = db
		.prepare(
			"INSERT INTO tbl_work_records(project_id, begin)\n \
	 		 VALUES(?, strftime('%s'));")
	 	.unwrap();

	stmt.bind(1, project_id).unwrap();
	stmt.next().unwrap();
}

pub fn status() {
	let db = database_open();

	let mut stmt = db
		.prepare(
			"SELECT work_record_id, \
			 (CASE WHEN end IS NULL THEN 0 ELSE 1 END) as record_complete\n \
			 FROM tbl_work_records\n \
			 ORDER BY work_record_id DESC LIMIT 1;")
		.unwrap();

	stmt.next().unwrap();

	let state_str: &str;

	if stmt.read::<i64>(1).unwrap() == 1 {
		state_str = "";
	}
	else {
		state_str = "NOT ";
	}

	println!("Last record ({}) is {}done.",
		stmt.read::<String>(0).unwrap(),
		state_str);
}

pub fn stop(description: String) {
	let db = database_open();

	let mut stmt = db
		.prepare(
			"UPDATE tbl_work_records\n\
		 	 SET end = strftime('%s'), description = ?\n\
		 	 WHERE work_record_id = (SELECT MAX(work_record_id) FROM tbl_work_records);")
		.unwrap();

	stmt.bind(1, description.as_str()).unwrap();
	stmt.next().unwrap();
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
}

pub fn edit_record_begin(record_id: i64, year: i64, month: i64, day: i64, hour: i64, minute: i64) {
	let db = database_open();

	let mut stmt = db
		.prepare(
			"UPDATE tbl_work_records\n\
	 		 SET begin = unixepoch('?-?-? ?:?:00')\n\
	 		 WHERE work_record_id = ?;")
	 	.unwrap();

	stmt.bind(1, year).unwrap();
	stmt.bind(2, month).unwrap();
	stmt.bind(3, day).unwrap();
	stmt.bind(4, hour).unwrap();
	stmt.bind(5, minute).unwrap();
	stmt.bind(6, record_id).unwrap();
	stmt.next().unwrap();
}
