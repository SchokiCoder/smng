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

pub fn get_base() -> Result<(Locale, sqlite::Connection, Vec<String>), ()> {
	let lcl = crate::lang::get_locale();
	let db = crate::db::database_open();
	
	if db.is_ok() == false {
		println!("{}", lcl.db_conn_fail());
		return Err(());
	}
	
	let db = db.unwrap().unwrap(&lcl);
	
	let all_args: Vec<String> = std::env::args().collect();
	let args = all_args[1..].to_vec();
	
	return Ok((lcl, db, args));
}

pub struct Command<'a> {
	pub info: &'a str,
	pub name: &'a str,
	pub args: Option<&'a str>,
	pub min_args: usize,
	pub max_args: usize,
	pub args_all_or_none: bool,
}

impl Command<'_> {
	pub fn new<'a>(
		info: &'static str,
		name: &'a str,
		args: Option<&'a str>,
		min_args: usize,
		max_args: usize,
		args_all_or_none: bool)
		-> Command<'a>
	{
		return Command {
			info,
			name,
			args,
			min_args,
			max_args,
			args_all_or_none,
		};
	}
	
	pub fn print_help(&self) {
		println!("  {}:", self.info);
		print!("  {}", self.name);

		if self.args.is_some() {
			print!(" | {}", self.args.unwrap());
		}

		println!("\n");	
	}
	
	pub fn arg_count_pass(&self, lcl: &Locale,	arg_count: usize) -> bool {
		if arg_count > self.max_args {
			println!("{}", lcl.too_many_args(Warning(&lcl.warning())));
			return true;
		}
		else if arg_count < self.min_args {
			if arg_count == 0 {
				self.print_help();
				return false;
			}
			else {
				println!("{}", lcl.too_little_args(Error(&lcl.error())));
				return false;
			}
		}
		else {
			if self.args_all_or_none {
				if arg_count != self.min_args && arg_count != self.max_args {
					println!("{}", lcl.not_enough_args(Error(&lcl.error())));
					return false;
				}
			}
			
			return true;
		}
	}
}

