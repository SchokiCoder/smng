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

use crate::cmd;

pub enum Language {
	English,
	German,
}

pub struct EnvStrings {
	pub cargo_pkg_name: String,
	pub cargo_pkg_version: String,
	pub cargo_pkg_license: String,
	pub cargo_pkg_repository: String
}

impl EnvStrings {
	pub fn new() -> EnvStrings {
		return EnvStrings {
			cargo_pkg_name: env!("CARGO_PKG_NAME").to_string(),
			cargo_pkg_version: env!("CARGO_PKG_NAME").to_string(),
			cargo_pkg_license: env!("CARGO_PKG_LICENSE").to_string(),
			cargo_pkg_repository: env!("CARGO_PKG_REPOSITORY").to_string()
		};
	}
}

pub struct LocalStrings {
	pub app_usage: String,
	pub app_about: String,
	
	pub cmd_unknown: String,
	
	pub too_many_args: String,
	pub too_little_args: String,
	pub not_enough_args: String,
	
	pub cfg_not_open: String,
	pub cfg_not_found: String,
	
	pub db_conn_fail: String,
	pub db_create: String,
	
	pub info: String,
	pub projects: String,
	pub archived_projects: String,
	pub records: String,
	pub report: String,
	pub administration: String,

	pub project_added: String,
	pub project_tbl_head: String,
	pub project_name_set: String,
	pub project_archived_noedit: String,
	pub project_archived_nouse: String,
	pub project_archived: String,
	pub project_unarchived: String,
	pub project_deleted: String,
	pub project_purged: String,
	
	pub record_last_not_done: String,
	pub record_started: String,
	pub record_: String,
	pub : String,
	pub : String,
	pub : String,
	pub : String,
	pub : String,
	pub : String,
}

impl LocalStrings {
	pub fn new_english() -> LocalStrings {
		let env_strings = EnvStrings::new();
		let warn_str = "WARNING:";
		let err_str = "ERROR:";
		let gnu_licenses = "https://www.gnu.org/licenses/";
		
		return LocalStrings {
			app_usage: format!("Usage:\n\
								{} command [arguments]\n\
								Try '{} {}' for more information.",
								env_strings.cargo_pkg_name,
								env_strings.cargo_pkg_name,
								cmd::HELP.name),
			app_about: format!("{} {} is licensed under the {}.\n\
								You should have received a copy of the license along with this \
								program.\n\
								If not see <{}>\n\
								\n\
								The source code of this program is available at:\n\
								{}",
								env_strings.cargo_pkg_name,
								env_strings.cargo_pkg_version,
								env_strings.cargo_pkg_license,
								gnu_licenses,
								env_strings.cargo_pkg_repository),
			
			cmd_unknown: "Command not recognised.".to_string(),
			
			too_many_args: format!("{} Too many arguments were given.\n\
									Additional arguments will be ignored.",
									warn_str),
			too_little_args: format!("{} Not enough arguments given.", err_str),
			not_enough_args: format!("{} Arguments given but not enough.", err_str),
			
			cfg_not_open: "Config file could not be opened".to_string(),
			cfg_not_found: "No config file could be found or read.\n\
							Create at least one!".to_string(),
			
			db_conn_fail: "Connection to database failed".to_string,
			db_create: format!("{} Database does not exist and will be newly created", warn_str),
			
			info: "Info".to_string(),
			projects: "Projects".to_string(),
			archived_projects: "Archived projects".to_string(),
			records: "Records".to_string(),
			report: "Report".to_string(),
			administration: "Administration".to_string(),
			
			project_added: "Project added".to_string(),
			project_tbl_head: format!("{:9} | {}", "id", "project name"),
			project_archived_noedit: format!("{} Project is archived and can not be edited",
											err_str),
			project_archived_nouse: format!("{} Project is archived and can not be used", err_str),
			project_name_set: "Project name set".to_string,
			project_archived: "Project has been archived".to_string(),
			project_unarchived: "Project has been unarchived".to_string(),
			project_deleted: "Project deleted".to_string(),
			project_purged: "Project and its records deleted".to_string(),
			
			record_last_not_done: format!("{} Last record is not yet done", err_str),
			record_started: "Record for given project started".to_string(),
		};
	}
	
	pub fn new_german() -> LocalStrings {
		let env_strings = EnvStrings::new();
		let warn_str = "WARNUNG:";
		let err_str = "FEHLER:";
		let gnu_licenses = "https://www.gnu.org/licenses/";
		
		return LocalStrings {
			app_usage: format!("Nutzung: {} Kommando [Argumente]:\n\
								 Versuche '{} {}' für mehr Informationen.",
								env_strings.cargo_pkg_name,
								env_strings.cargo_pkg_name,
								cmd::HELP.name),
		};
	}
}

