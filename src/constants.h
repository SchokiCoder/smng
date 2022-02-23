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

#include <stdint.h>
#include "config.h"

/* application defintions */
static const char APP_NAME[] = "smng";
static const char APP_LICENSE[] = "GPLv3";
static const char APP_LICENSE_NOTICE[] =
"You should have received a copy of the GNU General Public License\n" \
"along with this program.  If not, see <https://www.gnu.org/licenses/>.\n";
static const char APP_SOURCE[] = "https://github.com/SchokiCoder/smng";
static const uint32_t APP_MAJOR = 1;
static const uint32_t APP_MINOR = 2;
static const uint32_t APP_PATCH = 1;

/* path definitions */
#ifdef _WIN32
static const char SLASH[] = "\\";
#else
static const char SLASH[] = "/";
#endif

#define PATH_MAX_LEN 256

/* if static database path is not enabled, define dynamic path */
#ifndef STATIC_DATABASE_PATH
static const char PATH_BASE[] = "%s/.%s";
static const char FILE_DATABASE[] = "worktimes.db";
#endif

/* commands */
static const char CMD_HELP[] = "h";
static const char CMD_HELP_LONG[] = "help";
static const char CMD_ADD_PROJECT[] = "ap";
static const char CMD_ADD_PROJECT_LONG[] = "add-project";
static const char CMD_SHOW_PROJECTS[] = "sp";
static const char CMD_SHOW_PROJECTS_LONG[] = "show-projects";
static const char CMD_EDIT_PROJECT[] = "ep";
static const char CMD_EDIT_PROJECT_LONG[] = "edit-project";
static const char CMD_DELETE_PROJECT_LONG[] = "delete-project";
static const char CMD_RECORD[] = "r";
static const char CMD_RECORD_LONG[] = "record";
static const char CMD_STATUS_LONG[] = "status";
static const char CMD_STOP[] = "s";
static const char CMD_STOP_LONG[] = "stop";
static const char CMD_EDIT_RECORD_PROJECT[] = "erp";
static const char CMD_EDIT_RECORD_PROJECT_LONG[] = "edit-record-project";
static const char CMD_EDIT_RECORD_BEGIN[] = "erb";
static const char CMD_EDIT_RECORD_BEGIN_LONG[] = "edit-record-begin";
static const char CMD_EDIT_RECORD_END[] = "ere";
static const char CMD_EDIT_RECORD_END_LONG[] = "edit-record-end";
static const char CMD_EDIT_RECORD_DESC[] = "erd";
static const char CMD_EDIT_RECORD_DESC_LONG[] = "edit-record-description";
static const char CMD_DELETE_RECORD_LONG[] = "delete-record";
static const char CMD_TRANSFER_PROJECT_RECORDS_LONG[] = "transfer-project-records";
static const char CMD_SHOW_WEEK[] = "sw";
static const char CMD_SHOW_WEEK_LONG[] = "show-week";
static const char CMD_SHOW_MONTH[] = "sm";
static const char CMD_SHOW_MONTH_LONG[] = "show-month";

/* help */
static const char MSG_HELP_APP[] =
"Usage:\n"
"  %s [COMMAND] [ARGS]\n";

static const char MSG_HELP_HELP[] =
"  show this message:\n" \
"  %s,\t%s\t\n";

static const char MSG_HELP_ADD_PROJECT[] =
"  add a project:\n" \
"  %s,\t%s\t\tPROJECT_NAME\n";

static const char MSG_HELP_SHOW_PROJECTS[] =
"  show projects:\n" \
"  %s,\t%s\n";

static const char MSG_HELP_EDIT_PROJECT[] =
"  edit project:\n" \
"  %s,\t%s\t\tPROJECT_ID PROJECT_NAME\n";

static const char MSG_HELP_DELETE_PROJECT[] =
"  delete project:\n" \
"  %s\t\tPROJECT_ID [PURGE]\n";

static const char MSG_HELP_RECORD[] =
"  record work time:\n" \
"  %s,\t%s\t\t\tPROJECT_ID\n";

static const char MSG_HELP_STATUS[] =
"  show current work status:\n" \
"  %s\n";

static const char MSG_HELP_STOP[] =
"  stop recording work time:\n" \
"  %s,\t%s\t\t\tDESCRIPTION\n";

static const char MSG_HELP_EDIT_RECORD_PROJECT[] =
"  edit work record's project:\n" \
"  %s,\t%s\tRECORD_ID PROJECT_ID\n";

static const char MSG_HELP_EDIT_RECORD_BEGIN[] =
"  edit work record's begin:\n" \

#ifdef MAJOR_DATEFORMAT
"  %s,\t%s\tRECORD_ID HOUR MINUTE DAY MONTH YEAR\n";
#else
"  %s,\t%s\tRECORD_ID YEAR MONTH DAY HOUR MINUTE\n";
#endif

static const char MSG_HELP_EDIT_RECORD_END[] =
"  edit work record's end:\n" \

#ifdef MAJOR_DATEFORMAT
"  %s,\t%s\t\tRECORD_ID HOUR MINUTE DAY MONTH YEAR\n";
#else
"  %s,\t%s\t\tRECORD_ID YEAR MONTH DAY HOUR MINUTE\n";
#endif

static const char MSG_HELP_EDIT_RECORD_DESC[] =
"  edit work record's description:\n" \
"  %s,\t%s\tRECORD_ID DESCRIPTION\n";

static const char MSG_HELP_DELETE_RECORD[] =
"  delete a work record:\n" \
"  %s\t\t\tRECORD_ID\n";

static const char MSG_HELP_TRANSFER_PROJECT_RECORDS[] =
"  transfer all records from one project to another:\n" \
"  %s\tOLD_PROJECT_ID NEW_PORJECT_ID\n";

static const char MSG_HELP_SHOW_WEEK[] =
"  show work records of the week:\n" \

#ifdef MAJOR_DATEFORMAT
"  %s,\t%s\t\t[DAY MONTH YEAR]\n";
#else
"  %s,\t%s\t\t[YEAR MONTH DAY]\n";
#endif

static const char MSG_HELP_SHOW_MONTH[] =
"  show work records of a certain month:\n" \

#ifdef MAJOR_DATEFORMAT
"  %s,\t%s\t\t[MONTH YEAR]\n";
#else
"  %s,\t%s\t\t[YEAR MONTH]\n";
#endif

static const char MSG_HELP_EXTRA[] =
"Id's:\n" \
"If you use negative numbers as id's then the most recent tuple is used.\n" \
"For example -1 is the newest and -2 the one that was created before.\n" \
"But don't use 0. That would do nothing.\n";

static const char MSG_HELP_APP_INFO[] =
"%s %u.%u.%u is licensed under the %s.\n" \
"%s" \
"The source code of this program is available at\n" \
"%s\n";

#endif /* CONSTANTS_H */
