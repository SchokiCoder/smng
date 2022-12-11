/*
 * SchokiManager
 * Copyright (C) 2021 - 2022  Andy Frank Schoknecht
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not see
 * <https://www.gnu.org/licenses/old-licenses/gpl-2.0.html>.
 */

#![allow(dead_code)]

use crate::lang::*;

pub fn database_open(path: &str) -> Result<sqlite::Connection, std::io::ErrorKind> {
	let lcl = cur_locale();
	
	// if db doesn't exist, flag
	let db_empty: bool;
	
	if std::fs::metadata(path).is_ok() == false {
		db_empty = true;
	}
	else {
		db_empty = false;
	}

	// open db
	let db = {
		let temp = sqlite::open(path);

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
	db.execute("PRAGMA foreign_keys = ON;").unwrap();

	// if flagged, create database	
	if db_empty {
		println!("{}: {}", lcl.warning(), lcl.db_create());
		
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

	return Ok(db);
}
