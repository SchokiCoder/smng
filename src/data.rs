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

pub struct RecordState {
	pub id: i64,
	pub state: i64,
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

pub fn record_archived(db: &sqlite::Connection, record_id: i64) -> bool {
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
