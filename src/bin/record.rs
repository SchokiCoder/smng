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
use data::RecordState;
use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(author, version, about = None, long_about = None)]
struct Args {
   #[arg(short, long)]
   project_id: i64,
}

fn main() {
	// get basic data and set command about
	let lcl = Locale::new();
	let base = cmd::get_base();
	
	if base.is_ok() == false {
		return;
	}
	
	let db = base.unwrap();
	
	let _matches = command!()
		.about(lcl.about_record)
		.get_matches();
	
	let args = Args::parse();
		
	// if used project is archived, stop
	if data::project_archived(&db, args.project_id) {
		println!("{}: {} ({})", lcl.error, lcl.project_archived_nouse, args.project_id);
		return;
	}
	
	// if last record is not done, stop
	let rec_state = RecordState::last(&db);

	if rec_state.id != 0 {
		if rec_state.done == false {
			println!("{}: {} ({})", lcl.error, lcl.record_last_not_done, rec_state.id);
			return;
		}
	}

	let mut stmt = db
		.prepare(
			"INSERT INTO tbl_work_records(project_id, begin)\n \
	 		 VALUES(?, strftime('%s', 'now', 'localtime'));")
	 	.unwrap();

	stmt.bind(1, args.project_id).unwrap();
	stmt.next().unwrap();

	println!("{} ({})", lcl.record_started, args.project_id);
}

