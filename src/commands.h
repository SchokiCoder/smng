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

#ifndef COMMANDS_H
#define COMMANDS_H

#include <SM_types.h>

typedef enum Command
{
	CMD_HELP,
	CMD_ADD_PROJECT,
	CMD_SHOW_PROJECTS,
	CMD_EDIT_PROJECT,
	CMD_DELETE_PROJECT,
	CMD_RECORD,
	CMD_STATUS,
	CMD_STOP,
	CMD_EDIT_RECORD_PROJECT,
	CMD_EDIT_RECORD_BEGIN,
	CMD_EDIT_RECORD_END,
	CMD_EDIT_RECORD_DESCRIPTION,
	CMD_DELETE_RECORD,
	CMD_TRANSFER_PROJECT_RECORDS,
	CMD_SHOW_WEEK,
	CMD_SHOW_MONTH,

	CMD_FIRST = CMD_HELP,
	CMD_LAST = CMD_SHOW_MONTH
} Command ;

typedef struct CommandData
{
	char *name;
	bool_t has_abbr;
	char *abbr;
	char *desc;
	bool_t has_args;
	char *args;
} CommandData ;

typedef enum CommandDJB2
{
	DJB2_HELP = 2900064876,
	DJB2_HELP_ABBR = 183058,

	DJB2_ADD_PROJECT = 1767124910,
	DJB2_ADD_PROJECT_ABBR = 6223846,

	DJB2_SHOW_PROJECTS = 2636365711,
	DJB2_SHOW_PROJECTS_ABBR = 6224458,

	DJB2_EDIT_PROJECT = 1380238766,
	DJB2_EDIT_PROJECT_ABBR = 6223982,

	DJB2_DELETE_PROJECT = 2913380526,

	DJB2_RECORD = 2854509100,
	DJB2_RECORD_ABBR = 183068,

	DJB2_STATUS = 2919916853,

	DJB2_STOP = 2900514662,
	DJB2_STOP_ABBR = 183069,

	DJB2_EDIT_RECORD_PROJECT = 1106117550,
	DJB2_EDIT_RECORD_PROJECT_ABBR = 211615568,

	DJB2_EDIT_RECORD_BEGIN = 2545332964,
	DJB2_EDIT_RECORD_BEGIN_ABBR = 211615554,

	DJB2_EDIT_RECORD_END = 1328592572,
	DJB2_EDIT_RECORD_END_ABBR = 211615557,

	DJB2_EDIT_RECORD_DESCRIPTION = 3260670992,
	DJB2_EDIT_RECORD_DESCRIPTION_ABBR = 211615556,

	DJB2_DELETE_RECORD = 3948397740,

	DJB2_TRANSFER_PROJECT_RECORDS = 4242603083,

	DJB2_SHOW_WEEK = 4158624913,
	DJB2_SHOW_WEEK_ABBR = 6224465,

	DJB2_SHOW_MONTH = 3941334064,
	DJB2_SHOW_MONTH_ABBR = 6224455,
} CommandDJB2 ;

static const CommandData DATA_COMMANDS[] = {
	{"help", TRUE, "h",
	"show this message",
	FALSE, "",},

	{"add-project", TRUE, "ap",
	"add a project",
	TRUE, "PROJECT_NAME"},

	{"show-projects", TRUE, "sp",
	"show projects",
	FALSE, ""},

	{"edit-project", TRUE, "ep",
	"edit project",
	TRUE, "PROJECT_ID PROJECT_NAME"},

	{"delete-project", FALSE, "",
	"delete project",
	TRUE, "PROJECT_ID [PURGE]"},

	{"record", TRUE, "r",
	"record work time",
	TRUE, "PROJECT_ID"},

	{"status", FALSE, "",
	"show current work status",
	FALSE, ""},

	{"stop", TRUE, "s",
	"stop recording work time",
	TRUE, "DESCRIPTION"},

	{"edit-record-project", TRUE, "erp",
	"edit work record's project",
	TRUE, "RECORD_ID PROJECT_ID"},

	{"edit-record-begin", TRUE, "erb",
	"edit work record's begin",
	TRUE,
#ifdef MAJOR_DATEFORMAT
	"RECORD_ID HOUR MINUTE DAY MONTH YEAR"
#else
	"RECORD_ID YEAR MONTH DAY HOUR MINUTE"
#endif
	},

	{"edit-record-end", TRUE, "ere",
	"edit work record's end",
	TRUE,
#ifdef MAJOR_DATEFORMAT
	"RECORD_ID HOUR MINUTE DAY MONTH YEAR"
#else
	"RECORD_ID YEAR MONTH DAY HOUR MINUTE"
#endif
	},

	{"edit-record-description", TRUE, "erd",
	"edit work record's description",
	TRUE, "RECORD_ID DESCRIPTION"},

	{"delete-record", FALSE, "",
	"delete a work record",
	TRUE, "RECORD_ID"},

	{"transfer-project-records", FALSE, "",
	"transfer all records from one project to another",
	TRUE, "OLD_PROJECT_ID NEW_PORJECT_ID"},

	{"show-week", TRUE, "sw",
	"show work records of a certain week",
	TRUE,
#ifdef MAJOR_DATEFORMAT
	"[DAY MONTH YEAR]"
#else
	"[YEAR MONTH DAY]"
#endif
	},

	{"show-month", TRUE, "sm",
	"show work records of a certain month",
	TRUE,
#ifdef MAJOR_DATEFORMAT
	"[MONTH YEAR]"
#else
	"[YEAR MONTH]"
#endif
	},
};

void cmd_help( void );

void cmd_add_project( const char *project_name );

void cmd_show_projects( void );

void cmd_edit_project( const sl32_t project_id, const char *project_name );

void cmd_delete_project( const sl32_t project_id, const bool_t purge );

void cmd_record( const sl32_t project_id );

void cmd_status( void );

void cmd_stop( const char *description );

void cmd_edit_record_project( const sl32_t record_id, const sl32_t project_id );

void cmd_edit_record_time(
	const bool_t work_record_begin, const sl32_t record_id,
	const sl16_t year, const sl8_t month, const sl8_t day,
	const sl8_t hour, const sl8_t minute );

void cmd_edit_record_description( const sl32_t record_id, const char *desc );

void cmd_delete_record( const sl32_t record_id );

void cmd_transfer_project_records( const sl32_t src_project_id, const sl32_t dest_project_id );

void cmd_show_records_month( const sl16_t year, const sl8_t month );

void cmd_show_records_week( const sl16_t year, const sl8_t month, const sl8_t day );

#endif /* COMMANDS_H */
