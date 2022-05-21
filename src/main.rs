/*
	SchokiManager
	Copyright (C) 2021	Andy Frank Schoknecht

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

pub mod app;
pub mod commands;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	// if not args given, print usage help and end
	if args.len() < 2 {
		println!("Usage {} COMMAND [ARGUMENTS]:", app::NAME);
		println!("Try '{} {}' for more information.", app::NAME, commands::DATA_HELP.name);
		return;
	}
	
	match args[1].as_str() {
		commands::DATA_HELP::name => commands::help(),
		_ => println!("Command not recognised."),
	}
}
