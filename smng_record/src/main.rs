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

use smng_lib::{init, data, cmdinfo};
use data::RecordState;
use clap::{Parser};

/*#[derive(Parser, Debug)]
#[command(author, version, about = None, long_about = None)]
struct Args {
   #[arg(short, long)]
   project_id: i64,
}*/

fn main() {
	// get basic data
	let base = init::init();
	
	if base.is_ok() == false {
		return;
	}
	
	let (db, lcl) = base.unwrap();
	
	let cmd = cmdinfo::record(
		&lcl,
		env!("CARGO_PKG_NAME"),
		env!("CARGO_PKG_VERSION"),
		env!("CARGO_PKG_AUTHORS"))
		.get_matches();
	//let args = Args::parse();
	let args = cmd.parse();

	// if used project is archived, stop
	if data::project_archived(&db, args.project_id) {
		println!("{}: {} ({})", lcl.error(), lcl.project_archived_nouse(), args.project_id);
		return;
	}
	
	// if last record is not done, stop
	let rec_state = RecordState::last(&db);

	if rec_state.id != 0 {
		if rec_state.done == false {
			println!("{}: {} ({})", lcl.error(), lcl.record_last_not_done(), rec_state.id);
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

	println!("{} ({})", lcl.record_started(), args.project_id);
}

