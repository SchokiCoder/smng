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
use data::records::RecordState;

fn main() {
	// get basic data (language, db, args, cmd data)
	let base = cmd::get_base();
	let (lcl, db, args): (Locale, sqlite::Connection, Vec<String>);
		
	if base.is_ok() == false {
		return;
	}
	
	(lcl, db, args) = base.unwrap();
	
	let cmd_data = cmd::Command::new(
		"stop recording work time",
		"stop",
		Some("description"),
		1, 1, false);
	
	// check arg count
	if cmd_data.arg_count_pass(&lcl, args.len()) == false {
		return;
	}
	
	// parse
	let description = args[0].as_str();
	
	// if last record is 0, stop
	let rec_state = RecordState::last(&db);

	if rec_state.id == 0 {
		println!("{}", lcl.record_none_available(Error(&lcl.error())));
		return;
	}
	
	// if last record is done, stop
	if rec_state.state == 1 {
		println!("{}: {} ({}).", lcl.error(), lcl.record_last_done(), rec_state.id);
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

	println!("{}: ({}).", lcl.record_stopped(), description);
}
