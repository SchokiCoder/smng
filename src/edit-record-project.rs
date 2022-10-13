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

mod cmd;
mod data;
mod lang;
mod db;
mod cfg;
use lang::*;
use data::{record_archived, project_archived};

fn main() {
	// get basic data (language, db, args, cmd data)
	let base = cmd::get_base();
	let (lcl, db, args): (Locale, sqlite::Connection, Vec<String>);
		
	if base.is_ok() == false {
		return;
	}
	
	(lcl, db, args) = base.unwrap();
	
	let cmd_data = cmd::Command::new(
		"edit record's project",
		"edit-record-project",
		Some("record_id project_id"),
		2, 2, false);
	
	// check arg count
	if cmd_data.arg_count_pass(&lcl, args.len()) == false {
		return;
	}
	
	// parse
	let record_id = args[0].parse::<i64>().unwrap();
	let project_id = args[1].parse::<i64>().unwrap();
	
	// if used project is archived, stop
	if project_archived(&db, project_id) {
		println!("{} ({})", lcl.project_archived_nouse(Error(&lcl.error())), project_id);
		return;
	}
	
	// if record is assigned to archived project, stop
	if record_archived(&db, record_id) {
		println!("{} ({})", lcl.record_archived_noedit(Error(&lcl.error())), record_id);
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

	println!("{} ({}) = ({})", lcl.record_project_set(), record_id, project_id);
}

