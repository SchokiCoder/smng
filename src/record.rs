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

fn main() {
	let base = crate::cmd::get_base();
	let (lcl, db, args): (Locale, sqlite::Connection, Vec<String>);
	
	if base.is_ok() {
		(lcl, db, args) = base.unwrap();
	}
	else {
		return;
	}
	
	// parse arg
	if cmd::arg_count_pass(&lcl, ) == false {
		return;
	}
	
	let project_id = args[1].parse::<i64>().unwrap();
	
	// if used project is archived, stop
	if data::projects::project_archived(&db, project_id) {
		println!("{} ({}).", lcl.project_archived_nouse(Error(&lcl.error())), project_id);
		return;
	}
	
	// if last record is not done, stop
	let rec_state = data::records::RecordState::last(&db);

	if rec_state.id != 0 {
		if rec_state.state == 0 {
			println!("{} {} ({}).", lcl.error(), lcl.record_last_not_done(), rec_state.id);
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

	println!("{} ({}).", lcl.record_started(), project_id);
}
