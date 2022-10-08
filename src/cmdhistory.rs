/*
	"SchokiManager" in short "smng"
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

/*
const CMD_HISTORY_PATH: &str = "/.smng/cmd_history"; // (appendage)

fn cmd_history_open_append() -> std::fs::File {
	let mut path = env!("HOME").to_string();
	path.push_str(CMD_HISTORY_PATH);

	let f: Result<std::fs::File, std::io::Error>;

	f = std::fs::File::options().append(true).create(true).open(path);
	
	if f.is_ok() == false {
		panic!("Command history could not be opened");
	}
	
	let f = f.unwrap();
	
	return f;
}

fn cmd_history_append(cmd: &str) {
	use std::io::Write;
	
	let mut cmdh = cmd_history_open_append();
	
	if writeln!(cmdh, "{}", cmd).is_ok() == false {
		panic!("Could not write to command history");
	}
}
*/
