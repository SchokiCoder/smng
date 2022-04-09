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

#include <stdint.h>
#include <stdbool.h>

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
	bool has_abbr;
	char *abbr;
	char *desc;
	bool has_args;
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
	{"help", true, "h",
	"show this message",
	false, "",},

	{"add-project", true, "ap",
	"add a project",
	true, "PROJECT_NAME"},

	{"show-projects", true, "sp",
	"show projects",
	false, ""},

	{"edit-project", true, "ep",
	"edit project",
	true, "PROJECT_ID PROJECT_NAME"},

	{"delete-project", false, "",
	"delete project",
	true, "PROJECT_ID [PURGE]"},

	{"record", true, "r",
	"record work time",
	true, "PROJECT_ID"},

	{"status", false, "",
	"show current work status",
	false, ""},

	{"stop", true, "s",
	"stop recording work time",
	true, "DESCRIPTION"},

	{"edit-record-project", true, "erp",
	"edit work record's project",
	true, "RECORD_ID PROJECT_ID"},

	{"edit-record-begin", true, "erb",
	"edit work record's begin",
	true,
#ifdef MAJOR_DATEFORMAT
	"RECORD_ID HOUR MINUTE DAY MONTH YEAR"
#else
	"RECORD_ID YEAR MONTH DAY HOUR MINUTE"
#endif
	},

	{"edit-record-end", true, "ere",
	"edit work record's end",
	true,
#ifdef MAJOR_DATEFORMAT
	"RECORD_ID HOUR MINUTE DAY MONTH YEAR"
#else
	"RECORD_ID YEAR MONTH DAY HOUR MINUTE"
#endif
	},

	{"edit-record-description", true, "erd",
	"edit work record's description",
	true, "RECORD_ID DESCRIPTION"},

	{"delete-record", false, "",
	"delete a work record",
	true, "RECORD_ID"},

	{"transfer-project-records", false, "",
	"transfer all records from one project to another",
	true, "OLD_PROJECT_ID NEW_PORJECT_ID"},

	{"show-week", true, "sw",
	"show work records of a certain week",
	true,
#ifdef MAJOR_DATEFORMAT
	"[DAY MONTH YEAR]"
#else
	"[YEAR MONTH DAY]"
#endif
	},

	{"show-month", true, "sm",
	"show work records of a certain month",
	true,
#ifdef MAJOR_DATEFORMAT
	"[MONTH YEAR]"
#else
	"[YEAR MONTH]"
#endif
	},
};

void cmd_help(void);

void cmd_add_project(const char *p_project_name);

void cmd_show_projects(void);

void cmd_edit_project(int32_t p_project_id, const char *p_project_name);

void cmd_delete_project(int32_t p_project_id, bool p_purge);

void cmd_record(int32_t p_project_id);

void cmd_status(void);

void cmd_stop(const char *p_description);

void cmd_edit_record_project(int32_t p_work_record_id, int32_t p_project_id);

void cmd_edit_record_time(
	bool p_work_record_begin, int32_t p_work_record_id,
	int16_t p_year, int8_t p_month, int8_t p_day,
	int8_t p_hour, int8_t p_minute);

void cmd_edit_record_description(int32_t p_work_record_id, const char *p_desc);

void cmd_delete_record(int32_t p_record_id);

void cmd_transfer_project_records(int32_t p_src_project_id, int32_t p_dest_project_id);

void cmd_show_records_month(int16_t p_year, int8_t p_month);

void cmd_show_records_week(int16_t p_year, int8_t p_month, int8_t p_day);

#endif /* COMMANDS_H */
