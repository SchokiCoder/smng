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

const GLOB_CFG_DB_PATH: &str = "/etc/smng.d/db_path";
const USER_CFG_DB_PATH: &str = "/.config/smng/db_path";	// appendage
const LOCL_CFG_DB_PATH: &str = "db_path"; 				// appendage

pub enum ConfigPos {
	None,
	Global (String),
	User (String),
	Local (String),
}

pub fn find_cfg_file() -> ConfigPos {
	// try current working dir (next to binary)
	let mut temp: String;
	let args: Vec<String> = std::env::args().collect();

	if args.len() > 0 {
		temp = String::from(&args[0]);
		let temppos = temp.rfind('/');
		if temppos.is_some() {
			temp.truncate(temppos.unwrap());
		}
		temp.push('/');
		temp.push_str(LOCL_CFG_DB_PATH);

		// if exists, return
		if std::path::Path::new(&temp).exists() {
			return ConfigPos::Local(temp);
		}
	}

	// try user config dir
	temp = env!("HOME").to_string();
	temp.push_str(USER_CFG_DB_PATH);

	if std::path::Path::new(&temp).exists() {
		return ConfigPos::User(temp);
	}
	
	// try global config
	temp = GLOB_CFG_DB_PATH.to_string();
	
	if std::path::Path::new(GLOB_CFG_DB_PATH).exists() {
		return ConfigPos::Global(temp);
	}
	else {
		return ConfigPos::None;
	}
}

pub fn get_cfg_file() -> Result<std::fs::File, std::io::ErrorKind> {
	let cfgpos = find_cfg_file();
	
	match cfgpos {
		// if cfg exists, try open
		ConfigPos::Local (path) |
		ConfigPos::User (path) |
		ConfigPos::Global (path) => {
			let f = std::fs::File::open(&path);
			
			if !f.is_ok() {
				return Err(std::io::ErrorKind::Other);
			}
			
			return Ok(f.unwrap());
		},
		
		// if not, err
		ConfigPos::None => {
			return Err(std::io::ErrorKind::NotFound);
		},
	}
}

pub fn read_cfg_db_path() -> Result<String, std::io::ErrorKind> {
	use std::io::Read;
	
	let cfg_result = get_cfg_file();
	
	if cfg_result.is_ok() == false {
		return Err(cfg_result.err().unwrap());
	}
	
	let mut f = cfg_result.unwrap();
	
	let path: String;
	let mut etc_raw = [0; 255];
	let n = f.read(&mut etc_raw[..]).unwrap();
	
	let temp = std::str::from_utf8(&etc_raw[..n]).unwrap();
	path = String::from(String::from(temp).trim());
	
	return Ok(path);
}
