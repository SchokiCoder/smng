/*
	"SchokiManager" in short "smng"
	Copyright (C) 2021 - 2022	Andy Frank Schoknecht

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

use crate::lang::*;

pub struct DatabaseResult {
	new: bool,
	db: sqlite::Connection,
}

impl DatabaseResult {
	pub fn unwrap(self, lcl: &Locale) -> sqlite::Connection {
		if self.new {
			println!("{}", lcl.db_create(Warning(&lcl.warning())));
		}
		
		return self.db;
	}
}

pub fn database_open() -> Result<DatabaseResult, std::io::ErrorKind> {
	// read db path config
	let path_result = crate::cfg::read_cfg_db_path();
	
	if path_result.is_ok() == false {
		return Err(path_result.err().unwrap());
	}
	
	let path = path_result.unwrap();

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

		// else return err
		else {
			return Err(std::io::ErrorKind::Other);
		}
	};

	// activate foreign keys
	db.execute("PRAGMA foreign_keys = ON;")
		.unwrap();

	// if flagged, create database
	let db_new: bool;
	
	if db_empty {
		db_new = true;
		
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
	else {
		db_new = false;
	}

	return Ok(DatabaseResult {
		new: db_new,
		db: db,
	});
}
