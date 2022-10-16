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

const GNU_WEBSITE: &str = "https://www.gnu.org/licenses/";

pub struct Locale {
	pub error: String,
	pub warning: String,
	pub info: String,
	pub id: String,
	pub begin: String,
	pub end: String,
	pub time: String,
	pub project: String,
	pub description: String,
	pub projects: String,
	pub project_name: String,
	pub archived_projects: String,
	pub records: String,
	pub report: String,
	pub administration: String,
	
	pub app_about: String,
	
	pub cfg_not_open: String,
	pub cfg_not_found: String,
	pub cfg_create_appeal: String,
	pub cfg_at_local: String,
	pub cfg_at_user: String,
	pub cfg_at_global: String,
	
	pub db_conn_fail: String,
	pub db_create: String,
	pub db_merged: String,
	
	pub project_added: String,
	pub project_archived_noedit: String,
	pub project_archived_nouse: String,
	pub project_name_set: String,
	pub project_archived: String,
	pub project_unarchived: String,
	pub project_deleted: String,
	pub project_purged: String,
	
	pub record_last_done: String,
	pub record_last_not_done: String,
	pub record_started: String,
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
	pub unexpected_parameters: String,
	pub database_path: String,
	
	pub about_record: String,
}

impl Locale {	
	fn new_en() -> Locale {
		return Locale {
			error: "ERROR".to_string(),
			warning: "WARNING".to_string(),
			info: "Info".to_string(),
			id: "id".to_string(),
			begin: "begin".to_string(),
			end: "end".to_string(),
			time: "time".to_string(),
			project: "project".to_string(),
			description: "description".to_string(),
			projects: "Projects".to_string(),
			project_name: "project name".to_string(),
			archived_projects: "Archived projects".to_string(),
			records: "Records".to_string(),
			report: "Report".to_string(),
			administration: "Administration".to_string(),
			
			app_about: format!("{} {} is licensed under the {}\n\
				You should have received a copy of the license along with this program\n\
				If not see <{}>\n\n\
				The source code of this program is available at:\n\
				{}",
				env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_LICENSE"),
				GNU_WEBSITE, env!("CARGO_PKG_REPOSITORY")),
			
			cfg_not_open: "Config file could not be opened".to_string(),
			cfg_not_found: "No config file could be found or read".to_string(),
			cfg_create_appeal: "At least one config is needed".to_string(),
			cfg_at_local: "Config found in local space".to_string(),
			cfg_at_user: "Config found in user space".to_string(),
			cfg_at_global: "Config found in global space".to_string(),
			
			db_conn_fail: "Connection to database failed".to_string(),
			db_create: "Database does not exist and will be newly created".to_string(),
			db_merged: "All projects and work-records successfully moved".to_string(),
			
			project_added: "Project added".to_string(),
			project_archived_noedit: "Project is archived and can not be edited".to_string(),
			project_archived_nouse: "Project is archived and can not be used".to_string(),
			project_name_set: "Project name set".to_string(),
			project_archived: "Project has been archived".to_string(),
			project_unarchived: "Project has been unarchived".to_string(),
			project_deleted: "Project deleted".to_string(),
			project_purged: "Project and its records deleted".to_string(),
			
			record_last_done: "Last record is done".to_string(),
			record_last_not_done: "Last record is not yet done".to_string(),
			record_started: "Record for given project started".to_string(),
			record_none_available: "{error}: There are no records yet".to_string(),
			record_stopped: "Record stopped".to_string(),
			record_added: "Record added to project".to_string(),
			record_archived_noedit: "{error}: Record is archived and can not be edited".to_string(),
			record_archived_nodelete: "{error}: Record is archived and can not be deleted".to_string(),
			record_project_set: "Record project set".to_string(),
			record_begin_set: "Record begin set".to_string(),
			record_end_set: "Record end set".to_string(),
			record_description_set: "Record description set".to_string(),
			record_deleted: "Record deleted".to_string(),
			
			transfer_different_projects: "{error}: This command transfers the records of a project to another\n\
				A transfer needs two different projects".to_string(),
			transfer: "Records moved to other project".to_string(),
			
			swap_different_projects: "{error}: This command swaps the records of a project to another\n\
				A transfer needs two different projects".to_string(),
			swap: "Records of both projects swapped".to_string(),
			
			sum_worktime: "Summarized worktime".to_string(),
			unexpected_parameters: "Unexpected combination of parameters".to_string(),
			database_path: "Database path".to_string(),
			
			about_record: "Start recording work time.".to_string(),
		};
	}
	
	fn new_de() -> Locale {
		return Locale {
			error: "FEHLER".to_string(),
			warning: "WARNUNG".to_string(),
			info: "Info".to_string(),
			id: "Id".to_string(),
			begin: "Beginn".to_string(),
			end: "Ende".to_string(),
			time: "Zeit".to_string(),
			project: "Projekt".to_string(),
			description: "Beschreibung".to_string(),
			projects: "Projekte".to_string(),
			project_name: "Projekt-Name".to_string(),
			archived_projects: "Archivierte Projekte".to_string(),
			records: "Arbeitszeiten".to_string(),
			report: "Bericht".to_string(),
			administration: "Administration".to_string(),
			
			app_about: format!("{} {} ist lizensiert unter der {}\n\
				Sie sollten eine Kopie der Lizenz mit diesem Programm erhalten haben\n\
				Falls nicht, besuchen sie <{}>\n\n\
				Der Quellcode dieses Programmes ist verfügbar unter:\n{}",
				env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_LICENSE"),
				GNU_WEBSITE, env!("CARGO_PKG_REPOSITORY")),
			
			cfg_not_open: "Config Datei konnte nicht geöffnet werden".to_string(),
			cfg_not_found: "Keine Config Datei konnte gefunden oder gelesen werden".to_string(),
			cfg_create_appeal: "Es wird mindestens eine Config benötigt".to_string(),
			cfg_at_local: "Config gefunden im lokalem Bereich".to_string(),
			cfg_at_user: "Config gefunden im Nutzer-Bereich".to_string(),
			cfg_at_global: "Config gefunden im globalen Bereich".to_string(),
			
			db_conn_fail: "Verbindung zur Datenbank gescheitert".to_string(),
			db_create: "Datenbank existiert nicht und wird neu erstellt".to_string(),
			db_merged: "Alle Projekte und Arbeitszeiten erfolgreich verschoben".to_string(),
			
			project_added: "Projekt hinzugefügt".to_string(),
			project_archived_noedit: "Projekt ist archiviert und kann nicht bearbeitet werden".to_string(),
			project_archived_nouse: "Projekt ist archiviert und kann nicht genutzt werden".to_string(),
			project_name_set: "Projekt-Name gesetzt".to_string(),
			project_archived: "Projekt wurde archiviert".to_string(),
			project_unarchived: "Projekt wurde entarchiviert".to_string(),
			project_deleted: "Projekt gelöscht".to_string(),
			project_purged: "Projekt und Arbeitszeiten gelöscht".to_string(),
			
			record_last_done: "Letzte Arbeitszeit abgeschlossen".to_string(),
			record_last_not_done: "Letzte Arbeitszeit noch nicht abgeschlossen".to_string(),
			record_started: "Arbeitszeit für gegebenes Projekt gestartet".to_string(),
			record_none_available: "{error}: Es gibt noch keine Arbeitszeiten".to_string(),
			record_stopped: "Arbeitszeit gestoppt".to_string(),
			record_added: "Arbeitszeit zum Projekt hinzugefügt".to_string(),
			record_archived_noedit: "{error}: Arbeitszeit ist archiviert und kann nicht bearbeitet werden".to_string(),
			record_archived_nodelete: "{error}: Arbeitszeit ist archiviert und kann nicht gelöscht werden".to_string(),
			record_project_set: "Arbeitszeit-Projekt gesetzt".to_string(),
			record_begin_set: "Arbeitszeit-Beginn gesetzt".to_string(),
			record_end_set: "Arbeitszeit-Ende gesetzt".to_string(),
			record_description_set: "Arbeitszeit-Beschreibung gesetzt".to_string(),
			record_deleted: "Arbeitszeit gelöscht".to_string(),
			
			transfer_different_projects: "{error}: Dieser Befehl verschiebt Arbeitszeiten von einem Projekt zu einem anderen\n\
				Ein Transfer benötigt zwei verschiedene Projekte".to_string(),
			transfer: "Arbeitszeiten wurden zu anderem Project verschoben".to_string(),
			
			swap_different_projects: "{error}: Dieser Befehl tauscht Arbeitszeiten von einem Projekt mit einem anderen\n\
				Ein Tausch benötigt zwei verschiedene Projekte".to_string(),
			swap: "Arbeitszeiten von beiden Projekten getauscht".to_string(),
			
			sum_worktime: "Arbeitszeit insgesamt".to_string(),
			unexpected_parameters: "Unerwartete Kombination von Parametern".to_string(),
			database_path: "Datenbank-Pfad".to_string(),
			
			about_record: "Arbeitszeit-Aufnahme beginnen.".to_string(),
		};
	}
	
	pub fn new() -> Locale {
		// get user language
		let lcl: Locale;
		let lang = std::env::var("LANG");
		
		// if lang env var could be read		
		if lang.is_ok() == false {
			return Locale::new_en();
		}
		
		let lang = lang.unwrap();
		let lang = lang.split('.').next();
		
		if lang.is_some() == false {
			return Locale::new_en();
		}
		
		let lang = lang.unwrap();
			
		// get strings
		println!("{}", lang);
		
		match lang {
			"de_DE" => {
				lcl = Locale::new_de();
			}
			
			_ => {
				lcl = Locale::new_en();
			}
		}
		
		return lcl;
	}
}

