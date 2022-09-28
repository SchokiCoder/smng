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

use chrono::prelude::*;

pub struct Command<'a> {
	pub info: &'a str,
	pub name: &'a str,
	pub abbr: Option<&'a str>,
	pub args: Option<&'a str>,
	pub min_args: usize,
	pub max_args: usize,
	pub args_all_or_none: bool,
}

impl Command<'_> {
	pub fn print_help(&self) {
		println!("  {}:", self.info);
		print!("  {}", self.name);

		if self.abbr.is_some() {
			print!(", {}", self.abbr.unwrap());
		}

		if self.args.is_some() {
			print!(" | {}", self.args.unwrap());
		}

		println!("\n");	
	}
	
	pub fn arg_count_pass(&self, arg_count: usize) -> bool {
		if arg_count > self.max_args {
			println!("WARNING: Too many arguments were given.\n\
					  Additional arguments will be ignored.");
			return true;
		}
		else if arg_count < self.min_args {
			if arg_count == 0 {
				self.print_help();
				return false;
			}
			else {
				println!("ERROR: Not enough arguments given.");
				return false;
			}
		}
		else {
			if self.args_all_or_none {
				if arg_count != self.min_args && arg_count != self.max_args {
					println!("ERROR: Arguments given but not enough.");
					return false;
				}
			}
			
			return true;
		}
	}
}

impl PartialEq for Command<'_> {
	fn eq(&self, other: &Self) -> bool {
		if self.name == other.name {
			return true;
		}
		
		if self.abbr.is_some() & other.abbr.is_some() {
			if self.abbr.unwrap() == other.abbr.unwrap() {
				return true;
			}
		}
		
		return false;
	}
}

impl Eq for Command<'_> {}

pub const HELP: Command = Command {
	info: "prints all help messages",
	name: "help",
	abbr: Some("h"),
	args: None,
	min_args: 0,
	max_args: 0,
	args_all_or_none: false,
};

pub const ABOUT: Command = Command {
	info: "prints information about the application",
	name: "about",
	abbr: Some("abt"),
	args: None,
	min_args: 0,
	max_args: 0,
	args_all_or_none: false,
};

pub const ADD_PROJECT: Command = Command {
	info: "add a project",
	name: "add-project",
	abbr: Some("ap"),
	args: Some("project_name"),
	min_args: 1,
	max_args: 1,
	args_all_or_none: false,
};

pub const SHOW_PROJECTS: Command = Command {
	info: "show projects",
	name: "show-projects",
	abbr: Some("sp"),
	args: None,
	min_args: 0,
	max_args: 0,
	args_all_or_none: false,
};

pub const EDIT_PROJECT: Command = Command {
	info: "edit project name",
	name: "edit-project",
	abbr: Some("ep"),
	args: Some("project_id project_name"),
	min_args: 2,
	max_args: 2,
	args_all_or_none: false,
};

pub const ARCHIVE_PROJECT: Command = Command {
	info: "archive or unarchive a project",
	name: "archive-project",
	abbr: None,
	args: Some("project_id"),
	min_args: 1,
	max_args: 1,
	args_all_or_none: false,
};

pub const DELETE_PROJECT: Command = Command {
	info: "delete a project and if wished purge all records",
	name: "delete-project",
	abbr: None,
	args: Some("project_id [purge]"),
	min_args: 1,
	max_args: 2,
	args_all_or_none: false,
};

pub const RECORD: Command = Command {
	info: "record work time on given project",
	name: "record",
	abbr: Some("r"),
	args: Some("project_id"),
	min_args: 1,
	max_args: 1,
	args_all_or_none: false,
};

pub const STATUS: Command = Command {
	info: "show current work status",
	name: "status",
	abbr: None,
	args: None,
	min_args: 0,
	max_args: 0,
	args_all_or_none: false,
};

pub const STOP: Command = Command {
	info: "stop recording work time",
	name: "stop",
	abbr: Some("s"),
	args: Some("description"),
	min_args: 1,
	max_args: 1,
	args_all_or_none: false,
};

pub const ADD_RECORD: Command = Command {
	info: "add a new complete record",
	name: "add-record",
	abbr: Some("ar"),
	args: Some("project_id description year month day hour minute year month day hour minute"),
	min_args: 12,
	max_args: 12,
	args_all_or_none: false,
};

pub const EDIT_RECORD_PROJECT: Command = Command {
	info: "edit record's project",
	name: "edit-record-project",
	abbr: Some("erp"),
	args: Some("record_id project_id"),
	min_args: 2,
	max_args: 2,
	args_all_or_none: false,
};

pub const EDIT_RECORD_BEGIN: Command = Command {
	info: "edit record's begin",
	name: "edit-record-begin",
	abbr: Some("erb"),
	args: Some("record_id year month day hour minute"),
	min_args: 6,
	max_args: 6,
	args_all_or_none: false,
};

pub const EDIT_RECORD_END: Command = Command {
	info: "edit record's end",
	name: "edit-record-end",
	abbr: Some("ere"),
	args: Some("record_id year month day hour minute"),
	min_args: 6,
	max_args: 6,
	args_all_or_none: false,
};

pub const EDIT_RECORD_DESCRIPTION: Command = Command {
	info: "edit record's description",
	name: "edit-record-description",
	abbr: Some("erd"),
	args: Some("record_id description"),
	min_args: 2,
	max_args: 2,
	args_all_or_none: false,
};

pub const DELETE_RECORD: Command = Command {
	info: "delete given record",
	name: "delete-record",
	abbr: None,
	args: Some("record_id"),
	min_args: 1,
	max_args: 1,
	args_all_or_none: false,
};

pub const TRANSFER_PROJECT_RECORDS: Command = Command {
	info: "transfer all records from one project to another",
	name: "transfer-project-records",
	abbr: None,
	args: Some("source_project_id destination_project_id"),
	min_args: 2,
	max_args: 2,
	args_all_or_none: false,
};

pub const SWAP_PROJECT_RECORDS: Command = Command {
	info: "swap records of two projects",
	name: "swap-project-records",
	abbr: None,
	args: Some("project_id_a project_id_b"),
	min_args: 2,
	max_args: 2,
	args_all_or_none: false,
};

pub const SHOW_WEEK: Command = Command {
	info: "show records of a certain week or current",
	name: "show-week",
	abbr: Some("sw"),
	args: Some("[year month day]"),
	min_args: 0,
	max_args: 3,
	args_all_or_none: true,
};

pub const SHOW_MONTH: Command = Command {
	info: "show records of a certain month or current",
	name: "show-month",
	abbr: Some("sm"),
	args: Some("[year month]"),
	min_args: 0,
	max_args: 2,
	args_all_or_none: true,
};

pub const SHOW_PROJECT_RECORDS: Command = Command {
	info: "show records of a certain project",
	name: "show-project-records",
	abbr: Some("spr"),
	args: Some("project_id"),
	min_args: 1,
	max_args: 1,
	args_all_or_none: false,
};

pub const MERGE_DB: Command = Command {
	info: "merges projects and records of two databases",
	name: "merge-db",
	abbr: Some("mdb"),
	args: Some("source_database_path destination_database_path"),
	min_args: 2,
	max_args: 2,
	args_all_or_none: false,
};

pub const SHOW_ETC_PATH: Command = Command {
	info: "show path of currently used config",
	name: "show-cfg-path",
	abbr: Some("scp"),
	args: None,
	min_args: 0,
	max_args: 0,
	args_all_or_none: false,
};

pub const SHOW_DB_PATH: Command = Command {
	info: "show path of currently configured database",
	name: "show-db-path",
	abbr: Some("sdbp"),
	args: None,
	min_args: 0,
	max_args: 0,
	args_all_or_none: false,
};

const GLOB_ETC_DB_PATH: &str = "/etc/smng.d/db_path";
const USER_ETC_DB_PATH: &str = "/.config/smng/db_path"; // (appendage)
const LOCL_ETC_DB_PATH: &str = "db_path"; // (appendage)

enum ConfigPos {
	None,
	Global (String),
	User (String),
	Local (String),
}

fn find_cfg_file() -> ConfigPos {
	// try current working dir (next to binary)
	let mut temp: String;
	let args: Vec<String> = std::env::args().collect();

	if args.len() > 0 {
		temp = String::from(&args[0]);
		let temppos = temp.rfind('/');
		if temppos.is_some() {
			temp.truncate(temppos.unwrap());
		}
		temp.push('/');
		temp.push_str(LOCL_ETC_DB_PATH);

		// if exists, return
		if std::path::Path::new(&temp).exists() {
			return ConfigPos::Local (temp);
		}
	}

	// try user config dir
	temp = env!("HOME").to_string();
	temp.push_str(USER_ETC_DB_PATH);

	if std::path::Path::new(&temp).exists() {
		return ConfigPos::User (temp);
	}
	
	// try global config
	temp = GLOB_ETC_DB_PATH.to_string();
	
	if std::path::Path::new(GLOB_ETC_DB_PATH).exists() {
		return ConfigPos::Global (temp);
	}
	else {
		return ConfigPos::None;
	}
}

fn get_cfg_file() -> std::fs::File {
	let cfgpos = find_cfg_file();
	
	match cfgpos {
		// if cfg exists, try open
		ConfigPos::Local (path) |
		ConfigPos::User (path) |
		ConfigPos::Global (path) => {
			let f = std::fs::File::open(&path);
			
			if !f.is_ok() {
				panic!("Config file at \"{}\" could not be opened", &path);
			}
			
			return f.unwrap();
		},
		
		// if not, crash and burn
		ConfigPos::None => {
			panic!("No config file could not be found or read\n\
			Create at least one on path \"{}\"", GLOB_ETC_DB_PATH);
		},
	}
}

fn read_etc_db() -> String {
	use std::io::Read;
	
	let mut f = get_cfg_file();
	let path: String;
	let mut etc_raw = [0; 255];
	let n = f.read(&mut etc_raw[..]).unwrap();
	let temp = std::str::from_utf8(&etc_raw[..n]).unwrap();
	path = String::from(String::from(temp).trim());
	
	return path;
}

fn database_open() -> sqlite::Connection {
	// read db path config
	let path = read_etc_db();

	// if db doesn't exist, flag
	let db_empty: bool;
	
	if std::fs::metadata(path.as_str()).is_ok() == false {
		db_empty = true;
	}
	else {
		db_empty = false;
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
			panic!("Connection to database \"{}\" failed", path.as_str());
		}
	};

	// activate foreign keys
	db.execute("PRAGMA foreign_keys = ON;")
		.unwrap();

	// if flagged, create database
	if db_empty {
		println!(
			"WARNING: Database does not exist and will be newly created at \"{}\".",
			path.as_str());
		
		db.execute(
			"CREATE TABLE tbl_projects( \
			 project_id INTEGER PRIMARY KEY, \
			 project_name VARCHAR(32) NOT NULL UNIQUE, \
			 project_archived INTEGER);").unwrap();

		db.execute(
			"CREATE TABLE tbl_work_records( \
			 work_record_id INTEGER PRIMARY KEY, \
			 project_id INTEGER NOT NULL REFERENCES tbl_projects(project_id), \
			 begin INTEGER NOT NULL, \
			 end INTEGER CHECK(end > begin), \
			 description VARCHAR(50));").unwrap();

		db.execute(
			"CREATE INDEX idx_work_record_id ON tbl_work_records(work_record_id);").unwrap();

		db.execute(
			"CREATE INDEX idx_project_id ON tbl_projects(project_id);").unwrap();

		db.execute(
			"CREATE INDEX idx_begin ON tbl_work_records(begin);").unwrap();

		db.execute(
			"CREATE INDEX idx_end ON tbl_work_records(end);").unwrap();
	}

	return db;
}

pub fn help() {
	println!("Usage:");
	println!("{} command [arguments]", env!("CARGO_PKG_NAME"));
	println!("");

	println!("-- Info --");
	HELP.print_help();
	ABOUT.print_help();

	println!("-- Projects --");
	ADD_PROJECT.print_help();
	SHOW_PROJECTS.print_help();
	EDIT_PROJECT.print_help();
	ARCHIVE_PROJECT.print_help();
	DELETE_PROJECT.print_help();

	println!("-- Records --");
	RECORD.print_help();
	STATUS.print_help();
	STOP.print_help();
	ADD_RECORD.print_help();
	EDIT_RECORD_PROJECT.print_help();
	EDIT_RECORD_BEGIN.print_help();
	EDIT_RECORD_END.print_help();
	EDIT_RECORD_DESCRIPTION.print_help();
	DELETE_RECORD.print_help();
	TRANSFER_PROJECT_RECORDS.print_help();
	SWAP_PROJECT_RECORDS.print_help();

	println!("-- Report --");
	SHOW_WEEK.print_help();
	SHOW_MONTH.print_help();
	SHOW_PROJECT_RECORDS.print_help();

	println!("-- Administration --");
	MERGE_DB.print_help();
	SHOW_ETC_PATH.print_help();
	SHOW_DB_PATH.print_help();
}

pub fn about() {
	println!("{} {} is licensed under the {}.",
		env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_LICENSE"));
	println!("You should have received a copy of the license along with this program.");
	println!("If not see <{}>\n",
		"https://www.gnu.org/licenses/");
	println!("The source code of this program is available at:\n{}\n",
		env!("CARGO_PKG_REPOSITORY"));
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

fn print_project_header() {
	println!("{:9} | {}", "id", "project name");
}

pub fn show_projects() {
	let db = database_open();

	let mut stmt = db
		.prepare(
			"SELECT project_id, project_name, project_archived\n\
			 FROM tbl_projects;")
		.unwrap();

	println!("Projects:");
	print_project_header();

	// all projects
	let mut archived_projects = Vec::<(String, String)>::new();
	
	while let sqlite::State::Row = stmt.next().unwrap() {
		// if project archived: buffer, else: print
		if stmt.read::<i64>(2).unwrap() != 0 {
			archived_projects.push(
				(stmt.read::<String>(0).unwrap(),
				stmt.read::<String>(1).unwrap()));
		}
		else {
			println!(
				"{:9} | {}",
				stmt.read::<String>(0).unwrap(),
				stmt.read::<String>(1).unwrap());
		}
	}
	
	// if no archived projects, end
	if archived_projects.len() == 0 {
		return;
	}
	
	// print archived projects
	println!("\nArchived projects:");
	print_project_header();
	
	for (id, name) in archived_projects {
		println!("{:9} | {}", id, name);
	}
}

fn project_archived(db: &sqlite::Connection, project_id: i64) -> bool {
	let mut stmt = db
		.prepare(
			"SELECT project_archived\n\
			 FROM tbl_projects\n\
			 WHERE project_id = ?;")
		.unwrap();

	stmt.bind(1, project_id).unwrap();
	stmt.next().unwrap();
	
	return stmt.read::<i64>(0).unwrap() != 0;
}

fn record_archived(db: &sqlite::Connection, record_id: i64) -> bool {
	let mut stmt = db
		.prepare(
			"SELECT project_id\n\
			 FROM tbl_work_records\n\
			 WHERE work_record_id = ?;")
		.unwrap();

	stmt.bind(1, record_id).unwrap();
	stmt.next().unwrap();
	
	return project_archived(db, stmt.read::<i64>(0).unwrap());
}

pub fn edit_project(project_id: i64, project_name: &str) {
	let db = database_open();
	
	// if project archived, print and stop
	if project_archived(&db, project_id) {
		println!("ERROR: Project ({}) is archived and can not be edited.", project_id);
		return;
	}

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

pub fn archive_project(project_id: i64) {
	let db = database_open();

	// find out if targeted project is already archived	
	let archived = project_archived(&db, project_id);
	let arch_num: i64;
	let new_state_str: &str;
	
	// if archived, unarchive
	if archived {
		arch_num = 0;
		new_state_str = "unarchived";
	}
	else {
		arch_num = 1;
		new_state_str = "archived";
	}
	
	let mut stmt = db
		.prepare(
			"UPDATE tbl_projects\n\
			 SET project_archived = ?\n\
			 WHERE project_id = ?;")
		.unwrap();

	stmt.bind(1, arch_num).unwrap();
	stmt.bind(2, project_id).unwrap();
	stmt.next().unwrap();
	
	// end print
	println!("Project ({}) has been {}.", project_id, new_state_str);
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
	pub fn last(db: &sqlite::Connection) -> RecordState {
		let mut stmt = db.prepare(
				"SELECT work_record_id, \
				 (CASE WHEN end IS NULL THEN 0 ELSE 1 END) as record_complete\n \
				 FROM tbl_work_records\n \
				 ORDER BY work_record_id DESC LIMIT 1;").unwrap();

		stmt.next().unwrap();

		return RecordState {
			id: stmt.read::<i64>(0).unwrap(),
			state: stmt.read::<i64>(1).unwrap(),
		};
	}
}

pub fn record(project_id: i64) {
	let db = database_open();
	
	// if used project is archived, stop
	if project_archived(&db, project_id) {
		println!("ERROR: Project ({}) is archived and can not be used.", project_id);
		return;
	}
	
	// if last record is not done, stop
	let rec_state = RecordState::last(&db);

	if rec_state.id != 0 {
		if rec_state.state == 0 {
			println!("ERROR: Last record ({}) is not yet done.", rec_state.id);
			return;
		}
	}

	let mut stmt = db
		.prepare(
			"INSERT INTO tbl_work_records(project_id, begin)\n \
	 		 VALUES(?, strftime('%s', 'now', 'localtime'));")
	 	.unwrap();

	stmt.bind(1, project_id).unwrap();
	stmt.next().unwrap();

	println!("Record for project ({}) started.", project_id);
}

pub fn status() {
	let db = database_open();
	let rec_state = RecordState::last(&db);

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
	let db = database_open();
	let rec_state = RecordState::last(&db);

	// if last record is 0, stop
	if rec_state.id == 0 {
		println!("ERROR: There are no records yet.");
		return;
	}
	
	// if last record is done, stop
	if rec_state.state == 1 {
		println!("ERROR: Last record ({}) is already done.", rec_state.id);
		return;
	}

	//exec
	let mut stmt = db
		.prepare(
			"UPDATE tbl_work_records\n\
		 	 SET end = strftime('%s', 'now', 'localtime'), description = ?\n\
		 	 WHERE work_record_id = (SELECT MAX(work_record_id) FROM tbl_work_records);")
		.unwrap();

	stmt.bind(1, description).unwrap();
	stmt.next().unwrap();

	println!("Record stopped with description \"{}\".", description);
}

pub fn add_record(project_id: i64, description: &str,
	b_year: i64, b_month: i64, b_day: i64, b_hour: i64, b_minute: i64,
	e_year: i64, e_month: i64, e_day: i64, e_hour: i64, e_minute: i64)
{
	let db = database_open();
	
	// if used project is archived, stop
	if project_archived(&db, project_id) {
		println!("ERROR: Project ({}) is archived and can not be used.", project_id);
		return;
	}

	// exec
	let mut stmt = db
		.prepare(
			"INSERT INTO tbl_work_records(project_id, description, begin, end)\n\
			 VALUES(?, ?,
			 strftime('%s', printf('%04i-%02i-%02i %02i:%02i:00', ?, ?, ?, ?, ?)),
			 strftime('%s', printf('%04i-%02i-%02i %02i:%02i:00', ?, ?, ?, ?, ?)));")
		.unwrap();

	stmt.bind(1, project_id).unwrap();
	stmt.bind(2, description).unwrap();
	stmt.bind(3, b_year).unwrap();
	stmt.bind(4, b_month).unwrap();
	stmt.bind(5, b_day).unwrap();
	stmt.bind(6, b_hour).unwrap();
	stmt.bind(7, b_minute).unwrap();
	stmt.bind(8, e_year).unwrap();
	stmt.bind(9, e_month).unwrap();
	stmt.bind(10, e_day).unwrap();
	stmt.bind(11, e_hour).unwrap();
	stmt.bind(12, e_minute).unwrap();
	stmt.next().unwrap();

	println!("Record added to project ({}).", project_id);
}

pub fn edit_record_project(record_id: i64, project_id: i64) {
	let db = database_open();
	
	// if used project is archived, stop
	if project_archived(&db, project_id) {
		println!("ERROR: Project ({}) is archived and can not be used.", project_id);
		return;
	}
	
	// if record is assigned to archived project, stop
	if record_archived(&db, record_id) {
		println!("ERROR: Record ({}) is archived and can not be edited.", record_id);
		return;
	}

	// exec
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

fn edit_record_time(
	begin: bool, record_id: i64,
	year: i64, month: i64, day: i64,
	hour: i64, minute: i64)
	-> bool
{
	let db = database_open();
	
	// if record is assigned to archived project, stop
	if record_archived(&db, record_id) {
		println!("ERROR: Record ({}) is archived and can not be edited.", record_id);
		return false;
	}

	// exec
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
	
	return true;
}

pub fn edit_record_begin(record_id: i64, year: i64, month: i64, day: i64,
                         hour: i64, minute: i64) {
	if edit_record_time(true, record_id, year, month, day, hour, minute) {
		println!("Record ({}) begin set to {:04}-{:02}-{:02} {:02}:{:02}.",
			record_id,
			year, month, day,
			hour, minute);
	}
}

pub fn edit_record_end(record_id: i64, year: i64, month: i64, day: i64,
                       hour: i64, minute: i64) {
	if edit_record_time(false, record_id, year, month, day, hour, minute) {
		println!("Record ({}) end set to {:04}-{:02}-{:02} {:02}:{:02}.",
			record_id,
			year, month, day,
			hour, minute);
	}
}

pub fn edit_record_description(record_id: i64, description: &str) {
	let db = database_open();

	// if record is assigned to archived project, stop
	if record_archived(&db, record_id) {
		println!("ERROR: Record ({}) is archived and can not be edited.", record_id);
		return;
	}

	// exec
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

pub fn delete_record(record_id: i64) {
	let db = database_open();
	
	// if record is assigned to archived project, stop
	if record_archived(&db, record_id) {
		println!("ERROR: Record ({}) is archived and can not be deleted.", record_id);
		return;
	}

	// exec
	let mut stmt = db
		.prepare(
			"DELETE\n\
			 FROM tbl_work_records\n\
			 WHERE work_record_id = ?;")
		.unwrap();

	stmt.bind(1, record_id).unwrap();
	stmt.next().unwrap();

	println!("Record ({}) deleted.", record_id);
}

pub fn transfer_project_records(src_project_id: i64, dest_project_id: i64) {
	// if project id's are equal, educate user and stop
	if src_project_id == dest_project_id {
		println!(
			"ERROR: This command transfers the records of a project to another.\n\
			A transfer needs two different projects.");
		return;
	}
	
	let db = database_open();
	
	// if src project is archived, stop
	if project_archived(&db, src_project_id) {
		println!("ERROR: Source project ({}) is archived and can not be edited.", src_project_id);
		return;
	}
	
	// if dest project is archived, stop
	if project_archived(&db, dest_project_id) {
		println!(
			"ERROR: Destination project ({}) is archived and can not be edited.",
			dest_project_id);
		return;
	}

	// exec
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

pub fn swap_project_records(project_id_a: i64, project_id_b: i64) {	
	// if project id's are equal, educate user and stop
	if project_id_a == project_id_b {
		println!(
			"ERROR: This command swaps the projects and records given.\n\
			A swap needs two different projects.");
		return;
	}
	
	let db = database_open();
	
	// if one project is archived, stop
	if project_archived(&db, project_id_a) {
		println!("ERROR: Project ({}) is archived and can not be edited.", project_id_a);
		return;
	}
	
	if project_archived(&db, project_id_b) {
		println!("ERROR: Project ({}) is archived and can not be edited.", project_id_b);
		return;
	}
	
	// create temp project, get temp project id
	db
		.execute(
			"INSERT INTO tbl_projects(project_name)\n\
			 VALUES('__swap');")
		.unwrap();
	
	let mut stmt = db
		.prepare(
			"SELECT project_id\n\
			 FROM tbl_projects\n\
			 WHERE project_name = '__swap';")
		.unwrap();
	
	stmt.next().unwrap();
	
	let tempid = stmt.read::<i64>(0).unwrap();

	// move a to temp
	let mut stmt = db
		.prepare(
			"UPDATE tbl_work_records\n\
			 SET project_id = ?\n\
			 WHERE project_id = ?;")
		.unwrap();
	
	stmt.bind(1, tempid).unwrap();
	stmt.bind(2, project_id_a).unwrap();
	
	while stmt.next().unwrap() != sqlite::State::Done {}
	
	// move b to a
	let mut stmt = db
		.prepare(
			"UPDATE tbl_work_records\n\
			 SET project_id = ?\n\
			 WHERE project_id = ?;")
		.unwrap();
		
	stmt.bind(1, project_id_a).unwrap();
	stmt.bind(2, project_id_b).unwrap();
	
	while stmt.next().unwrap() != sqlite::State::Done {}
	
	// move temp to b
	let mut stmt = db
		.prepare(
			"UPDATE tbl_work_records\n\
			 SET project_id = ?\n\
			 WHERE project_id = ?;")
		.unwrap();
	
	stmt.bind(1, project_id_b).unwrap();
	stmt.bind(2, tempid).unwrap();
	
	while stmt.next().unwrap() != sqlite::State::Done {}
	
	// delete temp project
	let mut stmt = db
		.prepare(
			"DELETE FROM tbl_projects\n\
			 WHERE project_id = ?;")
		.unwrap();
	
	stmt.bind(1, tempid).unwrap();
	stmt.next().unwrap();

	println!("Records of project ({}) swapped with project ({}).",
		project_id_a, project_id_b);
}

fn show_record(stmt: &sqlite::Statement, win_width: usize) -> i64 {
	let seconds = stmt.read::<i64>(5).unwrap();
	let minutes: u32 = seconds as u32 / 60;
	let hours: u32 = minutes / 60;

	// if record has an end, print
	let rec_end = stmt.read::<String>(4);
	
	if rec_end.is_ok() {
		print!(
			"{:9} | {:8} | {:8} | {:02}:{:02} | {:9} | ",
			stmt.read::<i64>(0).unwrap(),
			stmt.read::<String>(2).unwrap(),
			rec_end.unwrap(),
			hours, (minutes % 60),
			stmt.read::<i64>(6).unwrap());
	}
	else {
		return 0;
	}

	// print desc
	let desc = stmt.read::<String>(7).unwrap();
	let mut i = 0;
	let mut pos = 0;

	while i < desc.len() {
		if pos + 55 > win_width {
			print!("\n{:9} | {:8} | {:8} | {:02} {:02} | {:9} | ",
				"", "", "", "", "", "");
			pos = 0;
		}

		print!("{}", desc.chars().nth(i).unwrap());

		i += 1;
		pos += 1;
	}

	println!("");

	return seconds;
}

fn print_char(num: u32, c: char) {
	for _ in 0..num {
		print!("{}", c);
	}
}

fn print_day_summary(dash_len_first: u32, dash_len_second: u32, term_w: usize, worktime: &str) {
	let row_len_min = dash_len_first + 7 + dash_len_second;
	let row_filler: isize = term_w as isize - row_len_min as isize;
	
	print_char(dash_len_first, '-');
	print!(" {:5} ", worktime);
	print_char(dash_len_second, '-');
	
	if row_filler > 0 {
		print_char(row_filler as u32, '-');
	}
	
	println!("");
}

fn show_records(
	ts_begin: Option<i64>,
	ts_end: Option<i64>,
	project_id: Option<i64>)
{
	let term_w = term_size::dimensions_stdout().unwrap().0;
	let db = database_open();

	// build sql string, prepare, bind (depending on which params given)
	let mut stmt: sqlite::Statement;
	
	let mut sql = String::from(
		"SELECT work_record_id, \
		 strftime('%Y.%m.%d', begin, 'unixepoch') as begin_day, \
		 strftime('%H:%M', begin, 'unixepoch') as begin_time, \
		 strftime('%d', end, 'unixepoch') as end_day, \
		 strftime('%H:%M', end, 'unixepoch') as end_time, \
		 end - begin AS worktime, \
		 project_id, description\n \
		 FROM tbl_work_records\n");
	
	// all params
	if ts_begin.is_some() && ts_end.is_some() && project_id.is_some() {
		sql.push_str(
		"WHERE begin > strftime('%s', ?, 'unixepoch', 'localtime') \
		 AND end < strftime('%s', ?, 'unixepoch', 'localtime') \
		 AND project_id = ?;");
		 
		 stmt = db
			.prepare(&sql)
			.unwrap();
		
		stmt.bind(1, ts_begin).unwrap();
		stmt.bind(2, ts_end).unwrap();
		stmt.bind(3, project_id).unwrap();
	}
	// only begin and end
	else if ts_begin.is_some() && ts_end.is_some() && !project_id.is_some() {
		sql.push_str(
		"WHERE begin > strftime('%s', ?, 'unixepoch', 'localtime') \
		 AND end < strftime('%s', ?, 'unixepoch', 'localtime');");
		 
		 stmt = db
			.prepare(&sql)
			.unwrap();
		
		stmt.bind(1, ts_begin).unwrap();
		stmt.bind(2, ts_end).unwrap();
	}
	// only project id
	else if !ts_begin.is_some() && !ts_end.is_some() && project_id.is_some() {
		sql.push_str(
		"WHERE project_id = ?;");
		
		stmt = db
			.prepare(&sql)
			.unwrap();
		
		stmt.bind(1, project_id).unwrap();
	}
	else {
		panic!("Unexpected combination of parameters.");
	}

	// print header
	println!("{:9} | {:8} | {:8} | {:5} | {:9} | {}",
		"id", "begin", "end", "time", "project", "description");

	let mut sum_seconds: u32 = 0;
	let mut pre_day: String;
	let mut cur_day = String::from("");
	let mut day_seconds: u32 = 0;

	// print first record
	// (because the if (cur day change) would needlessly print day worktime)
	if stmt.next().unwrap() == sqlite::State::Row {
		cur_day = stmt.read::<String>(1).unwrap();
		println!("- {} -", cur_day);
		
		let seconds = show_record(&stmt, term_w);
		
		sum_seconds += seconds as u32;
		day_seconds += seconds as u32;
	}

	const DASH_LEN_FIRST: u32 = 33;
	const DASH_LEN_SECOND: u32 = 25;

	// other records
	while stmt.next().unwrap() == sqlite::State::Row {
		// if current day changes
		pre_day = cur_day;
		cur_day = stmt.read::<String>(1).unwrap();

		if cur_day != pre_day {
			// print new day line
			let minutes: u32 = day_seconds / 60;
			let hours: u32 = minutes / 60;
			
			let worktime = Utc
				.ymd(1970, 1, 1)
				.and_hms(hours, minutes - hours * 60, 0)
				.format("%H:%M")
				.to_string();
			
			print_day_summary(DASH_LEN_FIRST, DASH_LEN_SECOND, term_w, worktime.as_str());
			println!("- {} -", cur_day);

			// reset day worktime
			day_seconds = 0;
		}

		let seconds = show_record(&stmt, term_w);
		
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
	
	print_day_summary(DASH_LEN_FIRST, DASH_LEN_SECOND, term_w, worktime.as_str());

	// summarized worktime
	let minutes: u32 = sum_seconds / 60;
	let hours: u32 = minutes / 60;
			
	println!("Summarized worktime: {:02}:{:02}.", hours, (minutes - hours * 60));
}

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
			end: date.and_hms(23, 59, 59).timestamp() + ((7 - weekday - 1) * DAY_SECONDS as i64),
		};

		return result;
	}
}

pub fn show_week_cur() {
	// print
	let week = WeekBeginAndEnd::from_date(Local::today());
	show_records(Some(week.begin), Some(week.end), None);
}

pub fn show_week(year: i32, month: u32, day: u32) {
	// print
	let week = WeekBeginAndEnd::from_date(Local.ymd(year, month, day));
	show_records(Some(week.begin), Some(week.end), None);
}

struct MonthBeginAndEnd {
	begin: i64,
	end: i64,
}

impl MonthBeginAndEnd {
	pub fn from_date(date: Date<Local>) -> MonthBeginAndEnd {
		// begin, from 28 on check the next 4 days for adding to ts_end
		let ts_begin = Local.ymd(date.year(), date.month(), 1).and_hms(0, 0, 0).timestamp();
		let ts_temp = Local.ymd(date.year(), date.month(), 28).and_hms(23, 59, 59).timestamp();
		let mut ts_end: i64 = ts_temp;

		for i in 1..4 {
			// if next day is in cur month, add one days seconds to ts_end
			if Local.timestamp(ts_begin, 0).month() ==
			   Local.timestamp(ts_end + (i * DAY_SECONDS as i64), 0).month()
		   	{
				ts_end += DAY_SECONDS as i64;
			}
			else {
				break;
			}
		}

		// add one more day to ts_end
		ts_end += DAY_SECONDS as i64;

		return MonthBeginAndEnd {
			begin: ts_begin,
			end: ts_end,
		};
	}
}

pub fn show_month_cur() {
	// print
	let month = MonthBeginAndEnd::from_date(Local::today());
	show_records(Some(month.begin), Some(month.end), None);
}

pub fn show_month(year: i32, month: u32) {
	// print
	let month = MonthBeginAndEnd::from_date(Local.ymd(year, month, 1));
	show_records(Some(month.begin), Some(month.end), None);
}

pub fn show_project_records(project_id: i64) {
	// print
	show_records(None, None, Some(project_id));
}

pub fn merge_db(src_db_path: &str, dest_db_path: &str) {
	use std::collections::HashMap;

	// try connecting to src db
	let src_db = sqlite::open(src_db_path);

	if src_db.is_ok() == false {
		panic!("ERROR: Could not connect to source database at \"{}\".", src_db_path);
	}

	let src_db = src_db.unwrap();

	// try connecting to dest db
	let dest_db = sqlite::open(dest_db_path);

	if dest_db.is_ok() == false {
		panic!("ERROR: Could not connect to destination database at \"{}\".", dest_db_path);
	}

	let dest_db = dest_db.unwrap();

	// create a project_id map, so we can redirect records
	// if project_ids differ but name doesn't
	let mut src_projects = HashMap::<String, i64>::new();
	let mut dest_projects = HashMap::<String, i64>::new();
	let mut dest_highest_prjid: i64 = 0;

	let mut stmt = src_db
		.prepare(
			"SELECT project_id, project_name\n\
			 FROM tbl_projects\n\
			 ORDER BY project_id;")
		.unwrap();

	while stmt.next().unwrap() == sqlite::State::Row {
		src_projects.insert(
			String::from(stmt.read::<String>(1).unwrap()),
			stmt.read::<i64>(0).unwrap());
	}
		
	let mut stmt = dest_db
		.prepare(
			"SELECT project_id, project_name\n\
			 FROM tbl_projects\n\
			 ORDER BY project_id;")
		.unwrap();

	while stmt.next().unwrap() == sqlite::State::Row {
		let temp = stmt.read::<i64>(0).unwrap();

		if temp > dest_highest_prjid {
			dest_highest_prjid = temp;
		}

		dest_projects.insert(
			stmt.read::<String>(1).unwrap(),
			temp);
	}

	// in dest, create all projects that dest doesn't have
	let mut stmt = dest_db
		.prepare(
			"INSERT INTO tbl_projects(project_name)\n\
			 VALUES(?);")
		.unwrap();

	for prj in src_projects {
		// if dest doesn't have this project
		if dest_projects.contains_key(&prj.0) == false {
			// bring over
			stmt.bind(1, prj.0.as_str()).unwrap();
			stmt.next().unwrap();

			// add to map
			dest_highest_prjid += 1;
			dest_projects.insert(prj.0, dest_highest_prjid);
		}
	}

	// bring all records over
	let mut read_stmt = src_db
		.prepare(
			"SELECT project_id, begin, end, description\n\
			 FROM tbl_work_records;")
		.unwrap();

	let mut write_stmt = dest_db
		.prepare(
			"INSERT INTO tbl_work_records(project_id, begin, end, description)\n\
			 VALUES(?, ?, ?, ?);")
		.unwrap();

	while read_stmt.next().unwrap() == sqlite::State::Row {
		let prj_id = read_stmt.read::<i64>(0).unwrap();
		let begin = read_stmt.read::<i64>(1).unwrap();
		let end = read_stmt.read::<i64>(2).unwrap();
		let desc = read_stmt.read::<String>(3).unwrap();

		write_stmt.bind(1, prj_id).unwrap();
		write_stmt.bind(2, begin).unwrap();
		write_stmt.bind(3, end).unwrap();
		write_stmt.bind(4, desc.as_str()).unwrap();
		write_stmt.next().unwrap();
	}

	println!("All projects and work-records successfully carried\nfrom: \"{}\"\nto  : \"{}\"",
		src_db_path, dest_db_path);
}

pub fn show_etc_path() {
	let cfgpos = find_cfg_file();
	
	match cfgpos {
		ConfigPos::Local (path) => {
			println!("Config at local space:\n\"{}\"", &path);
		},
		
		ConfigPos::User (path) => {
			println!("Config at user space:\n\"{}\"", &path);
		},
		
		ConfigPos::Global (path) => {
			println!("Config at global space:\n\"{}\"", &path);
		},
		
		ConfigPos::None => {
			println!("WARNING: Config not found.");
		},
	}
}

pub fn show_db_path() {
	let path = read_etc_db();
	
	println!("Database path:\n\"{}\"", &path);
}
