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
