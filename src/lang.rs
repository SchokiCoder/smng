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
	pub record_last_done: String,
	pub record_none_available: String,
	pub record_stopped: String,
	pub record_added: String,
	pub record_archived_noedit: String,
	pub record_archived_nodelete: String,
	pub record_project_set: String,
	pub record_begin_set: String,
	pub record_end_set: String,
	pub record_description_set: String,
	pub record_deleted: String,
	
	pub transfer_different_projects: String,
	pub transfer: String,

	pub swap_different_projects: String,
	pub swap: String,
	
	pub sum_worktime: String,
	pub : String,
	
	
	pub error: String,
	pub warning: String,
	pub info: String,
	pub projects: String,
	pub archived_projects: String,
	pub records: String,
	pub report: String,
	pub administration: String,
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
			
			record_tbl_head: format!("{:9} | {:8} | {:8} | {:5} | {:9} | {}",
				"id", "begin", "end", "time", "project", "description"),
			record_last_done: "Last record is done".to_string(),
			record_last_not_done: "Last record is not yet done".to_string(),
			record_started: "Record for given project started".to_string(),
			record_none_available: format!("{} There are no records yet", err_str),
			record_stopped: "Record stopped".to_string(),
			record_added: "Record added to project".to_string(),
			record_archived_noedit: format!("{} Record is archived and can not be edited", err_str),
			record_archived_nodelete: format!("{} Record is archived and can not be deleted",
				err_str),
			record_project_set: "Record project set".to_string(),
			record_begin_set: "Record begin set".to_string(),
			record_end_set: "Record end set".to_string(),
			record_description_set: "Record description set".to_string(),
			record_deleted: "Record deleted".to_string(),
			
			transfer_different_projects: format!("{} This command transfers the records of a project to another.\n\
												A transfer needs two different projects.", err_str),
			transfer: "Records moved to other project".to_string(),
			
			swap_different_projects: "{} This command swaps the records of a project to another.\n\
									A transfer needs two different projects.".to_string(),
			swap: "Records of both project swapped".to_string(),
			
			sum_worktime: "Summarized worktime".to_string(),
			
			error: err_str,
			warning: warn_str,
			info: "Info".to_string(),
			projects: "Projects".to_string(),
			archived_projects: "Archived projects".to_string(),
			records: "Records".to_string(),
			report: "Report".to_string(),
			administration: "Administration".to_string(),
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

