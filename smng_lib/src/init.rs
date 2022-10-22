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

pub fn init() -> Result<(sqlite::Connection, Locale), ()> {
	let lcl = cur_locale();
	
	// read db path config
	let path_result = crate::cfg::read_cfg_db_path();
	
	if path_result.is_ok() == false {
		println!("{}: {}", lcl.error(), lcl.cfg_not_open());
		return Err(());
	}
	
	let db_path = path_result.unwrap();
	
	// open db
	let db = crate::db::database_open(db_path.as_str());
	
	if db.is_ok() == false {
		println!("{}: {}\n({})", lcl.error(), lcl.db_conn_fail(), db_path);
		return Err(());
	}
	
	let db = db.unwrap();
	
	return Ok((db, lcl));
}

