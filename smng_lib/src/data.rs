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

pub fn project_archived(db: &sqlite::Connection, project_id: i64) -> bool {
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

const SQL_GET_RECORD_STATE: &str = "SELECT work_record_id, \
	(CASE WHEN end IS NULL THEN 0 ELSE 1 END) as record_complete, 
	(SELECT project_archived \
	 FROM tbl_projects
	 WHERE tbl_projects.project_id = tbl_work_records.project_id)\n \
	FROM tbl_work_records";

pub struct RecordState {
	pub id: i64,
	pub done: bool,
	pub archived: bool,
}

impl RecordState {
	pub fn by_id(db: &sqlite::Connection, record_id: i64) -> RecordState {
		let mut stmt = db
			.prepare(format!("{}\nWHERE work_record_id = ?;", SQL_GET_RECORD_STATE))
			.unwrap();

		stmt.bind(1, record_id).unwrap();
		stmt.next().unwrap();

		return RecordState {
			id: stmt.read::<i64>(0).unwrap(),
			done: if stmt.read::<i64>(1).unwrap() == 0 {false} else {true},
			archived: if stmt.read::<i64>(2).unwrap() == 0 {false} else {true},
		};
	}
	
	pub fn last(db: &sqlite::Connection) -> RecordState {
		let mut stmt = db
			.prepare(format!("{}\nORDER BY work_record_id DESC LIMIT 1;", SQL_GET_RECORD_STATE))
			.unwrap();

		stmt.next().unwrap();

		return RecordState {
			id: stmt.read::<i64>(0).unwrap(),
			done: if stmt.read::<i64>(1).unwrap() == 0 {false} else {true},
			archived: if stmt.read::<i64>(2).unwrap() == 0 {false} else {true},
		};
	}
}

