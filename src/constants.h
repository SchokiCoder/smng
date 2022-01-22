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

#ifndef CONSTANTS_H
#define CONSTANTS_H

/* application defintions */
#define APP_NAME "smng"
#define APP_LICENSE "GPLv3"
#define APP_LICENSE_NOTICE "You should have received a copy of the GNU General Public License\n" \
"along with this program.  If not, see <https://www.gnu.org/licenses/>.\n"
#define APP_SOURCE "https://github.com/SchokiCoder/smng"
#define APP_MAJOR "1"
#define APP_MINOR "1"
#define APP_PATCH "0"

/* path definitions */
#ifdef _WIN32
	#define SLASH "\\"
#else
	#define SLASH "/"
#endif

#define PATH_MAX_LEN 256

/* if static database path is not enabled, define dynamic path */
#ifndef STATIC_DATABASE_PATH
	#define PATH_BASE "%s/." APP_NAME
	#define FILE_DATABASE "worktimes.db"
#endif

/* commands */
#define CMD_HELP "h"
#define CMD_HELP_LONG "help"
#define CMD_ADD_PROJECT "ap"
#define CMD_ADD_PROJECT_LONG "add-project"
#define CMD_SHOW_PROJECTS "sp"
#define CMD_SHOW_PROJECTS_LONG "show-projects"
#define CMD_EDIT_PROJECT "ep"
#define CMD_EDIT_PROJECT_LONG "edit-project"
#define CMD_DELETE_PROJECT_LONG "delete-project"
#define CMD_RECORD "r"
#define CMD_RECORD_LONG "record"
#define CMD_STATUS_LONG "status"
#define CMD_STOP "s"
#define CMD_STOP_LONG "stop"
#define CMD_EDIT_RECORD_PROJECT "erp"
#define CMD_EDIT_RECORD_PROJECT_LONG "edit-record-project"
#define CMD_EDIT_RECORD_BEGIN "erb"
#define CMD_EDIT_RECORD_BEGIN_LONG "edit-record-begin"
#define CMD_EDIT_RECORD_END "ere"
#define CMD_EDIT_RECORD_END_LONG "edit-record-end"
#define CMD_EDIT_RECORD_DESC "erd"
#define CMD_EDIT_RECORD_DESC_LONG "edit-record-description"
#define CMD_DELETE_RECORD_LONG "delete-record"
#define CMD_SHOW_WEEK "sw"
#define CMD_SHOW_WEEK_LONG "show-week"
#define CMD_SHOW_MONTH "sm"
#define CMD_SHOW_MONTH_LONG "show-month"

/* help */
#define HELP_TEXT "Usage:\n" \
"  " APP_NAME " [COMMAND] [ARGS]\n" \
"\n" \
"  show this message:\n" \
"  " CMD_HELP ",\t" CMD_HELP_LONG "\t\n" \
"\n" \
"  add a project:\n" \
"  " CMD_ADD_PROJECT ",\t" CMD_ADD_PROJECT_LONG "\t\tPROJECT_NAME\n" \
"\n" \
"  show projects:\n" \
"  " CMD_SHOW_PROJECTS ",\t" CMD_SHOW_PROJECTS_LONG "\n" \
"\n" \
"  edit project:\n" \
"  " CMD_EDIT_PROJECT ",\t" CMD_EDIT_PROJECT_LONG "\t\tPROJECT_ID PROJECT_NAME\n" \
"\n" \
"  delete project:\n" \
"  " CMD_DELETE_PROJECT_LONG "\t\tPROJECT_ID [FORCE]\n" \
"\n" \
"  record work time:\n" \
"  " CMD_RECORD ",\t" CMD_RECORD_LONG "\t\t\tPROJECT_ID\n" \
"\n" \
"  show current work status:\n" \
"  " CMD_STATUS_LONG "\n" \
"\n" \
"  stop recording work time:\n" \
"  " CMD_STOP ",\t" CMD_STOP_LONG "\t\t\tDESCRIPTION\n" \
"\n" \
"  edit work record's project:\n" \
"  " CMD_EDIT_RECORD_PROJECT ",\t" CMD_EDIT_RECORD_PROJECT_LONG "\tRECORD_ID PROJECT_ID\n" \
"\n" \
"  edit work record's begin:\n" \
"  " CMD_EDIT_RECORD_BEGIN ",\t" CMD_EDIT_RECORD_BEGIN_LONG "\tRECORD_ID YEAR MONTH DAY HOUR MINUTE\n" \
"\n" \
"  edit work record's end:\n" \
"  " CMD_EDIT_RECORD_END ",\t" CMD_EDIT_RECORD_END_LONG "\t\tRECORD_ID YEAR MONTH DAY HOUR MINUTE\n" \
"\n" \
"  edit work record's description:\n" \
"  " CMD_EDIT_RECORD_DESC ",\t" CMD_EDIT_RECORD_DESC_LONG "\tRECORD_ID DESCRIPTION\n" \
"\n" \
"  delete a work record:\n" \
"  " CMD_DELETE_RECORD_LONG "\t\t\tRECORD_ID\n" \
"\n" \
"  show work records of the week:\n" \
"  " CMD_SHOW_WEEK ",\t" CMD_SHOW_WEEK_LONG "\t\t[YEAR MONTH DAY]\n" \
"\n" \
"  show work records of a certain month:\n" \
"  " CMD_SHOW_MONTH ",\t" CMD_SHOW_MONTH_LONG "\t\t[MONTH] [YEAR]\n" \
"\n" \
"Id's:\n" \
"If you use negative numbers as id's then the most recent tuple is used.\n" \
"For example -1 is the newest and -2 the one that was created before.\n" \
"But don't use 0. That would do nothing.\n" \
"\n" \
APP_NAME " " APP_MAJOR "." APP_MINOR "." APP_PATCH " is licensed under the " APP_LICENSE ".\n" \
APP_LICENSE_NOTICE \
"The source code of this program is available at\n" \
APP_SOURCE "\n"

#endif
