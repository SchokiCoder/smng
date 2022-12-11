/*
 * SchokiManager
 * Copyright (C) 2021  Andy Frank Schoknecht
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not see
 * <https://www.gnu.org/licenses/old-licenses/gpl-2.0.html>.
 */

#include <sys/stat.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <sqlite3.h>
#include <SM_string.h>
#include "app.h"
#include "config.h"
#include "commands.h"
#include "sql.h"
#include "tools.h"

#define PATH_MAX_LEN 256

#ifndef STATIC_DATABASE_PATH
#ifdef _WIN32
#define SLASH "\\"
#else
#define SLASH "/"
#endif

static const char PATH_BASE[] = "%s" SLASH ".%s";
static const char FILE_DATABASE[] = "worktimes.db";
#endif

void print_cmd_help(const Command cmd)
{
	printf("  %s:\n", DATA_COMMANDS[cmd].desc);

	SM_String cmd_naming = SM_String_from(DATA_COMMANDS[cmd].name);

	if (DATA_COMMANDS[cmd].has_abbr) {
		SM_String_append_cstr(&cmd_naming, ", ");
		SM_String_append_cstr(&cmd_naming, DATA_COMMANDS[cmd].abbr);
	}

	printf("  %-32s%s\n", cmd_naming.str, DATA_COMMANDS[cmd].args);

	printf("\n");

	SM_String_clear(&cmd_naming);
}

int32_t database_connect(sqlite3 ** db)
{
	int32_t rc_connect;
	int32_t rc_empty;
	int32_t rc_activate_fkeys;
	int32_t rc_create_work_records;
	int32_t rc_create_projects;
	int32_t rc_create_indices;
#ifndef STATIC_DATABASE_PATH
	int32_t rc_dir;
	char path[PATH_MAX_LEN] = "";
#endif				/* STATIC_DATABASE_PATH */

#ifndef STATIC_DATABASE_PATH
	// get path
	sprintf(path, PATH_BASE, getenv("HOME"), APP_NAME);

	// try create dir
	errno = 0;

#ifdef _WIN32
	rc_dir = mkdir(path);
#else
	rc_dir = mkdir(path, S_IRWXU);
#endif				/* _WIN32 */

	// if failure, stop
	if (rc_dir == -1) {
		if (errno != EEXIST) {
			printf
			    ("ERROR: Path to database does not exist and could not be created.\n");
			return 1;
		}
	}

	// get the rest of the path
	strcat(path, SLASH);
	strcat(path, FILE_DATABASE);

	// try connecting to db (with dynamic path)
	rc_connect = sqlite3_open(path, db);
#else
	// try connecting to db (with static path)
	rc_connect = sqlite3_open(PATH_DATABASE, db);
#endif				/* STATIC_DATABASE_PATH */

	// if no connection possible, end
	if (rc_connect != SQLITE_OK) {
		printf
		    ("Sqlite-ERROR (%i): The database is missing or access is not given.\n"
		     "Check the path in the config and make sure permissions are not missing.\n",
		     rc_connect);
		return 1;
	}

	// if db is empty, create tables etc.
	rc_empty =
	    sqlite3_table_column_metadata(*db, NULL, "tbl_work_records",
					  "work_record_id", NULL, NULL, NULL,
					  NULL, NULL);

	if (rc_empty == 1) {
		rc_activate_fkeys =
		    sqlite3_exec(*db, SQL_ACTIVATE_FKEYS, NULL, NULL, NULL);
		rc_create_work_records =
		    sqlite3_exec(*db, SQL_CREATE_WORKRECORDS, NULL, NULL, NULL);
		rc_create_projects =
		    sqlite3_exec(*db, SQL_CREATE_PROJECTS, NULL, NULL, NULL);
		rc_create_indices =
		    sqlite3_exec(*db, SQL_CREATE_INDICES, NULL, NULL, NULL);
	}

	// else activate fkeys and end
	else {
		rc_activate_fkeys =
		    sqlite3_exec(*db, SQL_ACTIVATE_FKEYS, NULL, NULL, NULL);

		if (rc_activate_fkeys != SQLITE_OK) {
			printf
			    ("Sqlite-ERROR (%i): The activation of foreign keys in this database did not work.\n",
			     rc_activate_fkeys);
			return 3;
		}

		return 0;
	}

	// if db creation fails, print error, end
	if ((rc_activate_fkeys != SQLITE_OK) ||
	    (rc_create_work_records != SQLITE_OK) ||
	    (rc_create_projects != SQLITE_OK) ||
	    (rc_create_indices != SQLITE_OK)) {
		printf
		    ("ERROR: The database was missing and an attempt to create it failed.\n");
		sqlite3_close(*db);
		*db = NULL;
		return 2;
	}

	// on success, print warning
	printf
	    ("WARNING: The database was missing and a new one was created.\n");
	return 0;
}

uint8_t is_prev_record_done(sqlite3 * db, uint32_t * record_id,
			    bool *record_done)
{
	sqlite3_stmt *stmt;
	int32_t rc_prepare;
	int32_t rc_step;

	// check if there is an open record left
	rc_prepare =
	    sqlite3_prepare_v2(db, SQL_CHECK_PREVIOUS_RECORD, -1, &stmt, 0);

	if (rc_prepare != SQLITE_OK) {
		printf
		    ("Sqlite-ERROR (%i): Statement to check validity of previous record could not prepared.\n",
		     rc_prepare);
		return 1;
	}

	rc_step = sqlite3_step(stmt);

	// if there are no previous records, just skip
	if (rc_step == SQLITE_DONE) {
		*record_id = 0;
		*record_done = true;
		sqlite3_finalize(stmt);
		return 0;
	} else if (rc_step != SQLITE_ROW) {
		printf
		    ("Sqlite-ERROR (%i): Statement to check validity of previous record could not be executed.\n",
		     rc_step);
		sqlite3_finalize(stmt);
		return 2;
	}

	// save values to output pointers
	*record_id = sqlite3_column_int(stmt, 0);
	*record_done = (bool)(sqlite3_column_int(stmt, 1));

	sqlite3_finalize(stmt);
	return 0;
}

uint8_t show_records(sqlite3 * db, const time_t begin, const time_t end)
{
	sqlite3_stmt *stmt;
	int32_t rc_prepare;
	int32_t rc_bind[2];
	int32_t rc_step;
	char timespan[2][14];
	char worked_time[16];
	struct tm *temp;
	uint32_t hours, minutes, seconds;
	uint32_t sum_seconds = 0;
	uint32_t sum_hours, sum_minutes;

	// prepare sql
	rc_prepare = sqlite3_prepare_v2(db, SQL_SHOW_RECORDS, -1, &stmt, 0);
	rc_bind[0] = sqlite3_bind_int(stmt, 1, begin);
	rc_bind[1] = sqlite3_bind_int(stmt, 2, end);

	if ((rc_prepare != SQLITE_OK) ||
	    (rc_bind[0] != SQLITE_OK) || (rc_bind[1] != SQLITE_OK)) {
		printf
		    ("Sqlite-ERROR (%i): Statement to show records could not be prepared.\n",
		     rc_prepare);
		sqlite3_finalize(stmt);
		return 1;
	}

	// if results are incoming
	rc_step = sqlite3_step(stmt);

	if (rc_step == SQLITE_ROW) {
		// print header
		temp = localtime(&begin);
		strftime(timespan[0], sizeof(timespan[0]), "%Y-%m-%d", temp);

		temp = localtime(&end);
		strftime(timespan[1], sizeof(timespan[1]), "%Y-%m-%d", temp);

		printf
		    ("Summarize from %s to %s:\n\nrec_id\tbegin	 end	  time	prj_id\tdesc\n",
		     timespan[0], timespan[1]);

		do {
			// convert seconds to hours and minutes
			seconds = sqlite3_column_int(stmt, 3);
			sum_seconds += seconds;
			minutes = seconds / 60;
			hours = minutes / 60;
			minutes = minutes % 60;

			// generate worked_time string as "hours(2):minutes(2)"
			sprintf(worked_time, "%02i:%02i", hours, minutes);

			// print results
			printf("%i\t%s %s %s %i\t%s\n",
			       sqlite3_column_int(stmt, 0),
			       sqlite3_column_text(stmt, 1),
			       sqlite3_column_text(stmt, 2),
			       worked_time,
			       sqlite3_column_int(stmt, 4),
			       sqlite3_column_text(stmt, 5));
		}
		while ((rc_step = sqlite3_step(stmt)) == SQLITE_ROW);

		// print sum time
		sum_minutes = sum_seconds / 60;
		sum_hours = sum_minutes / 60;
		sum_minutes = sum_minutes % 60;
		printf("\nSummarized work time: %02i:%02i\n", sum_hours,
		       sum_minutes);
	} else if (rc_step == SQLITE_DONE)
		printf("No records available.\n");
	else {
		printf
		    ("Sqlite-Error (%i): Statement to show work-records could not be executed.\n",
		     rc_step);
		sqlite3_finalize(stmt);
		return 2;
	}

	// clean
	sqlite3_finalize(stmt);
	return 0;
}

int32_t parse_id(sqlite3 * db, const int32_t raw, const bool is_project,
		 int32_t * result)
{
	sqlite3_stmt *stmt;
	int32_t rc_prep;
	int32_t rc_step;

	// if number is negative
	if (raw < 0) {
		// find real id
		if (is_project == true)
			rc_prep =
			    sqlite3_prepare_v2(db, SQL_MAX_PROJECT_ID, -1,
					       &stmt, 0);
		else
			rc_prep =
			    sqlite3_prepare_v2(db, SQL_MAX_RECORD_ID, -1, &stmt,
					       0);

		if (rc_prep != SQLITE_OK) {
			printf
			    ("Sqlite-ERROR (%i): Statement to find real id could not be prepared.\n",
			     rc_prep);
			sqlite3_finalize(stmt);
			return 1;
		}

		rc_step = sqlite3_step(stmt);

		if (rc_step != SQLITE_ROW) {
			printf
			    ("Sqlite-ERROR (%i): Statement to find real id could not be executed.\n",
			     rc_step);
			sqlite3_finalize(stmt);
			return 2;
		}

		*result = sqlite3_column_int(stmt, 0) + 1 + raw;
		sqlite3_finalize(stmt);
	} else {
		*result = raw;
	}

	return 0;
}

#ifdef SANITIZE_DATETIME
int32_t sanitize_datetime(const int16_t year, const int8_t month,
			  const int8_t day, const int8_t hour,
			  const int8_t minute)
{
	if (year < DT_YEAR_MIN || year > DT_YEAR_MAX) {
		printf("Given year %u is not allowed.\n", year);
		return 1;
	}

	if (month < DT_MONTH_MIN || month > DT_MONTH_MAX) {
		printf("Given month %u is not allowed.\n", month);
		return 2;
	}

	if (day < DT_DAY_MIN || day > DT_DAY_MAX) {
		printf("Given day %u is not allowed.\n", day);
		return 3;
	}

	if (hour < DT_HOUR_MIN || hour > DT_HOUR_MAX) {
		printf("Given hour %u is not allowed.\n", hour);
		return 4;
	}

	if (minute < DT_MINUTE_MIN || minute > DT_MINUTE_MAX) {
		printf("Given minute %u is not allowed.\n", minute);
		return 5;
	}

	return 0;
}
#endif
