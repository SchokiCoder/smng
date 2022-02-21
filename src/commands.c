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

#include "commands.h"
#include "constants.h"
#include "tools.h"
#include "sql.h"
#include <stdio.h>
#include <sqlite3.h>

void cmd_help(void)
{
	// print every help message
	printf(MSG_HELP_APP, APP_NAME);
	printf("\n");

	printf(MSG_HELP_HELP, CMD_HELP, CMD_HELP_LONG);
	printf("\n");

	printf(MSG_HELP_ADD_PROJECT, CMD_ADD_PROJECT, CMD_ADD_PROJECT_LONG);
	printf("\n");

	printf(MSG_HELP_SHOW_PROJECTS, CMD_SHOW_PROJECTS, CMD_SHOW_PROJECTS_LONG);
	printf("\n");

	printf(MSG_HELP_EDIT_PROJECT, CMD_EDIT_PROJECT, CMD_EDIT_PROJECT_LONG);
	printf("\n");

	printf(MSG_HELP_DELETE_PROJECT, CMD_DELETE_PROJECT_LONG);
	printf("\n");

	printf(MSG_HELP_RECORD, CMD_RECORD, CMD_RECORD_LONG);
	printf("\n");

	printf(MSG_HELP_STATUS, CMD_STATUS_LONG);
	printf("\n");

	printf(MSG_HELP_STOP, CMD_STOP, CMD_STOP_LONG);
	printf("\n");

	printf(MSG_HELP_EDIT_RECORD_PROJECT, CMD_EDIT_RECORD_PROJECT, CMD_EDIT_RECORD_PROJECT_LONG);
	printf("\n");

	printf(MSG_HELP_EDIT_RECORD_BEGIN, CMD_EDIT_RECORD_BEGIN, CMD_EDIT_RECORD_BEGIN_LONG);
	printf("\n");

	printf(MSG_HELP_EDIT_RECORD_END, CMD_EDIT_RECORD_END, CMD_EDIT_RECORD_END_LONG);
	printf("\n");

	printf(MSG_HELP_EDIT_RECORD_DESC, CMD_EDIT_RECORD_DESC, CMD_EDIT_RECORD_DESC_LONG);
	printf("\n");

	printf(MSG_HELP_DELETE_RECORD, CMD_DELETE_RECORD_LONG);
	printf("\n");

	printf(MSG_HELP_TRANSFER_PROJECT_RECORDS, CMD_TRANSFER_PROJECT_RECORDS_LONG);
	printf("\n");

	printf(MSG_HELP_SHOW_WEEK, CMD_SHOW_WEEK, CMD_SHOW_WEEK_LONG);
	printf("\n");

	printf(MSG_HELP_SHOW_MONTH, CMD_SHOW_MONTH, CMD_SHOW_MONTH_LONG);
	printf("\n");

	printf(MSG_HELP_EXTRA);
	printf("\n");

	printf(
		MSG_HELP_APP_INFO,
		APP_NAME, APP_MAJOR, APP_MINOR, APP_PATCH, APP_LICENSE,
		APP_LICENSE_NOTICE,
		APP_SOURCE);
	printf("\n");
}

void cmd_add_project(const char *p_project_name)
{
	sqlite3 *db;
	sqlite3_stmt *stmt;
	int32_t rc_prepare;
	int32_t rc_step;
	int32_t rc_bind;

	// try open connection
	if (database_connect(&db) != 0)
		return;

	// prepare
	rc_prepare = sqlite3_prepare_v2(db, SQL_ADD_PROJECT, -1, &stmt, 0);

	// try binding parameters
	rc_bind = sqlite3_bind_text(stmt, 1, p_project_name, -1, NULL);

	if ((rc_prepare != SQLITE_OK) ||
		(rc_bind != SQLITE_OK))
	{
		// if not ok, error and stop
		printf("Sqlite-ERROR (%i, %i): Statement to add project could not be prepared.\n", rc_prepare, rc_bind);
		sqlite3_finalize(stmt);
		sqlite3_close(db);
		return;
	}

	// try to execute
	rc_step = sqlite3_step(stmt);

	// catch constraint failure
	switch (rc_step)
	{
		case SQLITE_CONSTRAINT:
			printf(
				"Sqlite-ERROR (%i): Statement failed on a constraint.\n"
				"Make sure \"%s\" is not already used as a project name.\n",
				rc_step, p_project_name);
			break;

		case SQLITE_DONE:
			printf("Project \"%s\" added.\n", p_project_name);
			break;

		default:
			printf("Sqlite-ERROR (%i): Unknown error. Code RED, burn the evidence and run!\n", rc_step);
			break;
	}

	// clean
	sqlite3_finalize(stmt);
	sqlite3_close(db);
}

void cmd_show_projects(void)
{
	sqlite3 *db;
	sqlite3_stmt *stmt;
	int32_t rc_prepare;
	int32_t rc_step;

	// try open connection
	if (database_connect(&db) != 0)
		return;

	// prepare
	rc_prepare = sqlite3_prepare_v2(db, SQL_SHOW_PROJECTS, -1, &stmt, 0);

	// if not ok, print error and stop
	if (rc_prepare != SQLITE_OK)
	{
		printf("Sqlite-ERROR (%i): Statement to show projects could not be prepared.\n", rc_prepare);
		sqlite3_close(db);
	}

	// if results are incoming, print header
	rc_step = sqlite3_step(stmt);

	if (rc_step == SQLITE_ROW)
	{
		printf("project_id, project_name\n");

		do
		{
			// print results
			printf("%i, %s\n", sqlite3_column_int(stmt, 0), sqlite3_column_text(stmt, 1));
		}
		while ((rc_step = sqlite3_step(stmt)) == SQLITE_ROW);
	}
	else
		printf("No projects available.\n");

	// clean
	sqlite3_finalize(stmt);
	sqlite3_close(db);
}

void cmd_edit_project(int32_t p_project_id, const char *p_project_name)
{
	sqlite3 *db;
	sqlite3_stmt *stmt;
	int32_t id;
	int32_t rc_prepare;
	int32_t rc_bind[2];
	int32_t rc_step;

	// connect to db
	if (database_connect(&db) != 0)
		return;

	// prepare
	rc_prepare = sqlite3_prepare_v2(db, SQL_EDIT_PROJECT, -1, &stmt, 0);

	// bind parameters
	if (parse_id(db, p_project_id, true, &id) != 0)
	{
		sqlite3_finalize(stmt);
		sqlite3_close(db);
		return;
	}

	rc_bind[0] = sqlite3_bind_text(stmt, 1, p_project_name, -1, NULL);
	rc_bind[1] = sqlite3_bind_int(stmt, 2, id);

	// if prepare failed, print error
	if ((rc_prepare != SQLITE_OK) ||
		(rc_bind[0] != SQLITE_OK) ||
		(rc_bind[1] != SQLITE_OK))
	{
		printf(
			"Sqlite-ERROR (%i, %i, %i): Statement to edit project could not be prepared.\n",
			rc_prepare, rc_bind[0], rc_bind[1]);
		sqlite3_finalize(stmt);
		sqlite3_close(db);
		return;
	}

	// execute
	rc_step = sqlite3_step(stmt);

	// catch constraint failure
	switch (rc_step)
	{
		case SQLITE_CONSTRAINT:
			printf(
				"Sqlite-ERROR (%i): Statement failed on a constraint.\n"
				"Make sure \"%s\" is not already used as a project name.\n",
				rc_step, p_project_name);
			break;

		case SQLITE_DONE:
			printf("Project %i name set to \"%s\".\n", id, p_project_name);
			break;

		default:
			printf("Sqlite-ERROR (%i): Statement to edit project could not be executed.\n", rc_step);
			break;
	}

	// clean
	sqlite3_finalize(stmt);
	sqlite3_close(db);
}

void cmd_delete_project(int32_t p_project_id, bool p_purge)
{
	sqlite3 *db;
	sqlite3_stmt *stmt;
	int32_t id;
	int32_t rc_prep, rc_bind, rc_step;
	char confirmation = 'n';

	// connect to db
	if (database_connect(&db) != 0)
		return;

	// parse id
	if (parse_id(db, p_project_id, true, &id) != 0)
	{
		sqlite3_close(db);
		return;
	}

	// if purge mode activated, get confirmation to proceed
	if (p_purge == true)
	{
		printf("WARNING: The purge option was given.\n"
			"This will delete every record linking to this project.\n"
			"Enter 'y' to continue.\n");
		scanf("%c", &confirmation);

		// check confirmation
		if (confirmation != 'y')
		{
			printf("Confirmation not given.\nSTOPPED.\n");
			return;
		}
	}

	// if purge mode activated and confirmation given
	if (p_purge == true && confirmation == 'y')
	{
		// prepare, deleting all records linking to project
		rc_prep = sqlite3_prepare_v2(db, SQL_DELETE_PROJECT_RECORDS, -1, &stmt, 0);
		rc_bind = sqlite3_bind_int(stmt, 1, id);

		if ((rc_prep != SQLITE_OK) ||
			(rc_bind != SQLITE_OK))
		{
			printf(
				"Sqlite-ERROR (%i, %i): Statement to delete project's records could not be prepared.\n",
				rc_prep, rc_bind);
			sqlite3_finalize(stmt);
			sqlite3_close(db);
			return;
		}

		// delete
		rc_step = sqlite3_step(stmt);

		if (rc_step != SQLITE_DONE)
			printf(
				"Sqlite-ERROR (%i): Statement to delete project's records could not be executed.\n",
				rc_step);
		else
			printf("Records of project %i deleted.\n", id);

		sqlite3_finalize(stmt);
	}

	// prepare project deletion
	rc_prep = sqlite3_prepare_v2(db, SQL_DELETE_PROJECT, -1, &stmt, 0);
	rc_bind = sqlite3_bind_int(stmt, 1, id);

	if ((rc_prep != SQLITE_OK) ||
		(rc_bind != SQLITE_OK))
	{
		printf(
			"Sqlite-ERROR (%i, %i): Statement to delete project could not be prepared.\n",
			rc_prep, rc_bind);
		sqlite3_finalize(stmt);
		sqlite3_close(db);
		return;
	}

	// exec
	rc_step = sqlite3_step(stmt);

	if (rc_step != SQLITE_DONE)
		printf(
			"Sqlite-ERROR (%i): Statement to delete project could not be executed.\n"
			"Make sure that no records link to this project anymore or pass the purge option.\n",
			rc_step);
	else
		printf("Project %i deleted.\n", id);

	sqlite3_finalize(stmt);
	sqlite3_close(db);
}

void cmd_record(int32_t p_project_id)
{
	sqlite3 *db;
	sqlite3_stmt *stmt;
	int32_t id;
	uint32_t prev_record_id;
	bool prev_record_done;
	int32_t rc_prepare;
	int32_t rc_bind[2];
	int32_t rc_step;
	time_t record_begin;

	// connect
	if (database_connect(&db) != 0)
		return;

	// check validity of last work record
	if (is_prev_record_done(db, &prev_record_id, &prev_record_done) != 0)
	{
		sqlite3_close(db);
		return;
	}

	if (prev_record_done == false)
	{
		// invalid record found, print
		printf(
			"ERROR: Before starting a new work-record finish the last one.\n"
			"last work_record_id: %i\n", prev_record_id);
		sqlite3_close(db);
		return;
	}

	// add new work record
	time(&record_begin);

	rc_prepare = sqlite3_prepare_v2(db, SQL_START_WORK_RECORD, -1, &stmt, 0);

	if (parse_id(db, p_project_id, true, &id) != 0)
	{
		sqlite3_finalize(stmt);
		sqlite3_close(db);
		return;
	}

	rc_bind[0] = sqlite3_bind_int(stmt, 1, id);
	rc_bind[1] = sqlite3_bind_int(stmt, 2, record_begin);

	if ((rc_prepare != SQLITE_OK) ||
		(rc_bind[0] != SQLITE_OK) ||
		(rc_bind[1] != SQLITE_OK))
	{
		printf(
			"Sqlite-ERROR (%i, %i, %i): Statement to add new work record could not be prepared.\n",
			rc_prepare, rc_bind[0], rc_bind[1]);
		sqlite3_finalize(stmt);
		sqlite3_close(db);
		return;
	}

	rc_step = sqlite3_step(stmt);

	if (rc_step != SQLITE_DONE)
		printf("Sqlite-ERROR (%i): Statement to add new  work record could not be executed.\n", rc_step);
	else
		printf("Work record for project %i added.\n", id);

	sqlite3_finalize(stmt);
	sqlite3_close(db);
}

void cmd_status(void)
{
	sqlite3 *db;
	bool prev_record_done;
	uint32_t prev_record_id;

	// try connect to db
	if (database_connect(&db) != 0)
		return;

	// check validity of last work record
	if (is_prev_record_done(db, &prev_record_id, &prev_record_done) != 0)
	{
		sqlite3_close(db);
		return;
	}

	// print result
	printf(
		"Previous work record (%u) is " "%s" "done.\n",
		prev_record_id,
		(prev_record_done == false ? "NOT " : ""));
}

void cmd_stop(const char *p_description)
{
	sqlite3 *db;
	sqlite3_stmt *stmt;
	int32_t rc_prepare;
	int32_t rc_bind[2];
	int32_t rc_step;
	bool prev_record_done;
	uint32_t prev_record_id;
	time_t record_end;

	// try connect to db
	if (database_connect(&db) != 0)
		return;

	// check validity of last work record
	if (is_prev_record_done(db, &prev_record_id, &prev_record_done) != 0)
	{
		sqlite3_close(db);
		return;
	}

	// if last work record is closed, print and stop
	if (prev_record_done == true)
	{
		printf("ERROR: Your previous record is already finished.\nlast work_record_id: %i\n", prev_record_id);
		sqlite3_close(db);
		return;
	}

	// update latest work record
	time(&record_end);

	rc_prepare = sqlite3_prepare_v2(db, SQL_FINISH_WORK_RECORD, -1, &stmt, 0);
	rc_bind[0] = sqlite3_bind_int(stmt, 1, record_end);
	rc_bind[1] = sqlite3_bind_text(stmt, 2, p_description, -1, NULL);

	if ((rc_prepare != SQLITE_OK) ||
		(rc_bind[0] != SQLITE_OK) ||
		(rc_bind[1] != SQLITE_OK))
	{
		printf("Sqlite-ERROR (%i): Statement to finish the work-record could not be prepared.\n", rc_prepare);
		sqlite3_close(db);
		return;
	}

	rc_step = sqlite3_step(stmt);

	if (rc_step != SQLITE_DONE)
		printf("Sqlite-ERROR (%i): Statement to finish the work record could not be executed.\n", rc_step);
	else
		printf("Work record finished with following description:\n%s\n", p_description);

	sqlite3_finalize(stmt);
	sqlite3_close(db);
}

void cmd_edit_record_project(int32_t p_work_record_id, int32_t p_project_id)
{
	sqlite3 *db;
	sqlite3_stmt *stmt;
	int32_t rec_id;
	int32_t pro_id;
	int32_t rc_prepare;
	int32_t rc_bind[2];
	int32_t rc_step;

	// connect
	if (database_connect(&db) != 0)
		return;

	// update record
	rc_prepare = sqlite3_prepare_v2(db, SQL_EDIT_RECORD_PROJECT, -1, &stmt, 0);

	if ((parse_id(db, p_project_id, true, &pro_id) != 0) ||
		(parse_id(db, p_work_record_id, false, &rec_id) != 0))
	{
		sqlite3_finalize(stmt);
		sqlite3_close(db);
		return;
	}

	rc_bind[0] = sqlite3_bind_int(stmt, 1, pro_id);
	rc_bind[1] = sqlite3_bind_int(stmt, 2, rec_id);

	if ((rc_prepare != SQLITE_OK) ||
		(rc_bind[0] != SQLITE_OK) ||
		(rc_bind[1] != SQLITE_OK))
	{
		printf(
			"Sqlite-ERROR (%i, %i, %i): Statement to edit the work-record could not be prepared.\n",
			rc_prepare, rc_bind[0], rc_bind[1]);
		sqlite3_close(db);
		return;
	}

	rc_step = sqlite3_step(stmt);

	switch (rc_step)
	{
		case SQLITE_DONE:
			printf("Record %i project set to %i.\n", rec_id, pro_id);
			break;

		case SQLITE_CONSTRAINT:
			printf(
				"Sqlite-ERROR (%i): Statement failed on a constraint.\n"
				"Make sure project %i exists.\n", rc_step, p_project_id);
			break;

		default:
			printf("Sqlite-ERROR (%i): Statement to edit the work-record could not be executed.\n", rc_step);
			break;
	}

	sqlite3_finalize(stmt);
	sqlite3_close(db);
}

void cmd_edit_record_time(
	bool p_work_record_begin, int32_t p_work_record_id,
	int16_t p_year, int8_t p_month, int8_t p_day,
	int8_t p_hour, int8_t p_minute)
{
	sqlite3 *db;
	sqlite3_stmt *stmt;
	time_t ts_now;
	struct tm *now;
	time_t record_time;
	struct tm dt;
	int32_t id;
	int32_t rc_prepare;
	int32_t rc_bind[2];
	int32_t rc_step;

	// connect
	if (database_connect(&db) != 0)
		return;

	// if enabled, make sure given datetime makes sense
#ifdef SANITIZE_DATETIME
	if (sanitize_datetime(p_year, p_month, p_day, p_hour, p_minute) != 0)
		return;
#endif

	// parse datetime arguments and convert to unixepoch
	time(&ts_now);
	now = localtime(&ts_now);

	if (p_year < 0)
		dt.tm_year = now->tm_year;
	else
		dt.tm_year = p_year - 1900;

	if (p_month < 0)
		dt.tm_mon = now->tm_mon;
	else
		dt.tm_mon = p_month - 1;

	if (p_day < 0)
		dt.tm_mday = now->tm_mday;
	else
		dt.tm_mday = p_day;

	dt.tm_hour = p_hour;
	dt.tm_min = p_minute;
	dt.tm_sec = 0;

	record_time = mktime(&dt);

	// prepare sql
	if (parse_id(db, p_work_record_id, false, &id) != 0)
	{
		sqlite3_close(db);
		return;
	}

	rc_prepare = sqlite3_prepare_v2(
		db,
		(p_work_record_begin == true ? SQL_EDIT_RECORD_BEGIN : SQL_EDIT_RECORD_END),
		-1,
		&stmt,
		0);
	rc_bind[0] = sqlite3_bind_int(stmt, 1, record_time);
	rc_bind[1] = sqlite3_bind_int(stmt, 2, id);

	if ((rc_prepare != SQLITE_OK) ||
		(rc_bind[0] != SQLITE_OK) ||
		(rc_bind[1] != SQLITE_OK))
	{
		printf(
			"Sqlite-ERROR (%i, %i, %i): Statement to edit the work-record could not be prepared.\n",
			rc_prepare, rc_bind[0], rc_bind[1]);
		sqlite3_finalize(stmt);
		sqlite3_close(db);
		return;
	}

	// exec sql
	rc_step = sqlite3_step(stmt);

	if (rc_step == SQLITE_CONSTRAINT)
		printf(
			"Sqlite-ERROR (%i): Statement failed on a constraint."
			"\nMake sure the record's begin is before the end.\n", rc_step);
	else if (rc_step != SQLITE_DONE)
		printf("Sqlite-ERROR (%i): Statement to edit work-record could not be executed.\n", rc_step);
	else
		printf(
			"Record %i project %s set to %i-%02i-%02i %02i:%02i.\n",
			id, (p_work_record_begin == true ? "begin" : "end"),
			(1900 + dt.tm_year), (1 + dt.tm_mon), dt.tm_mday, p_hour, p_minute);

	sqlite3_finalize(stmt);
	sqlite3_close(db);
}

void cmd_edit_record_description(int32_t p_work_record_id, const char *p_desc)
{
	sqlite3 *db;
	sqlite3_stmt *stmt;
	int32_t id;
	int32_t rc_prep, rc_step, rc_bind[2];

	// connect
	if (database_connect(&db) != 0)
		return;

	// prepare
	if (parse_id(db, p_work_record_id, false, &id) != 0)
	{
		sqlite3_close(db);
		return;
	}

	rc_prep = sqlite3_prepare_v2(db, SQL_EDIT_RECORD_DESC, -1, &stmt, 0);
	rc_bind[0] = sqlite3_bind_text(stmt, 1, p_desc, -1, NULL);
	rc_bind[1] = sqlite3_bind_int(stmt, 2, id);

	if ((rc_prep != SQLITE_OK) ||
		(rc_bind[0] != SQLITE_OK) ||
		(rc_bind[1] != SQLITE_OK))
	{
		printf(
			"Sqlite-ERROR (%i, %i, %i): Statement to edit record description could not be prepared.\n",
			rc_prep, rc_bind[0], rc_bind[1]);
		sqlite3_finalize(stmt);
		sqlite3_close(db);
		return;
	}

	// exec
	rc_step = sqlite3_step(stmt);

	if (rc_step != SQLITE_DONE)
		printf("Sqlite-ERROR (%i): Statement to edit record description could not be executed.\n", rc_step);
	else
		printf("Record %i description changed to:\n%s\n", id, p_desc);

	sqlite3_finalize(stmt);
	sqlite3_close(db);
}

void cmd_delete_record(int32_t p_record_id)
{
	sqlite3 *db;
	sqlite3_stmt *stmt;
	int32_t id;
	int32_t rc_prep, rc_bind, rc_step;

	// connect to db
	if (database_connect(&db) != 0)
		return;

	// prepare
	if (parse_id(db, p_record_id, false, &id) != 0)
	{
		sqlite3_close(db);
		return;
	}

	rc_prep = sqlite3_prepare_v2(db, SQL_DELETE_RECORD, -1, &stmt, 0);
	rc_bind = sqlite3_bind_int(stmt, 1, id);

	if ((rc_prep != SQLITE_OK) ||
		(rc_bind != SQLITE_OK))
	{
		printf(
			"Sqlite-ERROR (%i, %i): Statement to delete record could not be prepared.\n",
			rc_prep, rc_bind);
		sqlite3_finalize(stmt);
		sqlite3_close(db);
		return;
	}

	// exec
	rc_step = sqlite3_step(stmt);

	if (rc_step != SQLITE_DONE)
		printf("Sqlite-ERROR (%i): Statement to delete record could not be executed.\n", rc_step);
	else
		printf("Record %i deleted.\n", id);

	sqlite3_finalize(stmt);
	sqlite3_close(db);
}

void cmd_transfer_project_records(int32_t p_old_project_id, int32_t p_new_project_id)
{
	sqlite3 *db;
	sqlite3_stmt *stmt;
	int32_t rc_prep, rc_bind[2], rc_step;
	char confirmation = 'n';

	// get user confirmation
	printf(
		"WARNING: This can cause data-manipulation which is hard to reverse.\n"
		"Enter 'y' to continue.\n");
	scanf("%c", &confirmation);

	if (confirmation != 'y')
		return;

	// connect to db
	if (database_connect(&db) != 0)
		return;

	// prepare sql
	rc_prep = sqlite3_prepare_v2(db, SQL_TRANSFER_PROJECT_RECORDS, -1, &stmt, 0);
	rc_bind[0] = sqlite3_bind_int(stmt, 1, p_new_project_id);
	rc_bind[1] = sqlite3_bind_int(stmt, 2, p_old_project_id);

	if ((rc_prep != SQLITE_OK) ||
		(rc_bind[0] != SQLITE_OK) ||
		(rc_bind[1] != SQLITE_OK))
	{
		printf("Sqlite-ERROR (%i): Statement to transfer records could not be prepared.\n", rc_prep);
		sqlite3_finalize(stmt);
		return;
	}

	// execute sql
	rc_step = sqlite3_step(stmt);

	while (rc_step == SQLITE_ROW)
	{
		rc_step = sqlite3_step(stmt);
	}

	if (rc_step != SQLITE_DONE)
	{
		printf("Sqlite-ERROR (%i): Statement to transfer records failed.\n", rc_step);
	}

	// done
	sqlite3_finalize(stmt);
	sqlite3_close(db);
}

void cmd_show_records_month(int16_t p_year, int8_t p_month)
{
	sqlite3 *db;
	time_t begin, end;
	struct tm temp_begin, temp_end;
	struct tm *now;
	time_t ts_now;

	// connect to db
	if (database_connect(&db) != 0)
		return;

	// if no date given, get current time
	time(&ts_now);
	now = localtime(&ts_now);

	if (p_year < 0 || p_month < 0)
	{
		temp_begin.tm_year = now->tm_year;
		temp_end.tm_year = now->tm_year;
		temp_begin.tm_mon = now->tm_mon;
		temp_end.tm_mon = now->tm_mon + 1;
	}
	else
	{
		// if enabled sanitize date
#ifdef SANITIZE_DATETIME
		if (sanitize_datetime(p_year, p_month, 1, 0, 0) != 0)
			return;
#endif

		temp_begin.tm_year = p_year - 1900;
		temp_end.tm_year = p_year -1900;
		temp_begin.tm_mon = p_month - 1;
		temp_end.tm_mon = p_month;
	}

	// get first and last second of month
	temp_begin.tm_mday = 1;
	temp_begin.tm_hour = 0;
	temp_begin.tm_min = 0;
	temp_begin.tm_sec = 0;
	temp_begin.tm_isdst = -1;

	temp_end.tm_mday = 0;
	temp_end.tm_hour = 23;
	temp_end.tm_min = 59;
	temp_end.tm_sec = 59;
	temp_end.tm_isdst = -1;

	begin = mktime(&temp_begin);
	end = mktime(&temp_end);

	// show
	show_records(db, begin, end);

	// close db
	sqlite3_close(db);
}

void cmd_show_records_week(int16_t p_year, int8_t p_month, int8_t p_day)
{
	sqlite3 *db;
	time_t begin, end;
	struct tm *date;

	// connect
	if (database_connect(&db) != 0)
		return;

	// if date not given, get current time
	time(&begin);
	date = localtime(&begin);
	date->tm_isdst = -1;

	if (p_year < 0 || p_month < 0 || p_day < 0)
	{
		date->tm_hour = 0;
		date->tm_min = 0;
		date->tm_sec = 0;
		begin = mktime(date);

		date->tm_hour = 23;
		date->tm_min = 59;
		date->tm_sec = 59;
		end = mktime(date);
	}
	else
	{
		// if enabled sanitize date
#ifdef SANITIZE_DATETIME
	if (sanitize_datetime(p_year, p_month, p_day, 0, 0) != 0)
		return;
#endif

		date->tm_year = p_year - 1900;
		date->tm_mon = p_month - 1;
		date->tm_mday = p_day;

		date->tm_hour = 0;
		date->tm_min = 0;
		date->tm_sec = 0;
		begin = mktime(date);

		date->tm_hour = 23;
		date->tm_min = 59;
		date->tm_sec = 59;
		end = mktime(date);
	}

	// find begin and end of week
	begin -= (date->tm_wday) * (60 * 60 * 24);
	end += (6 - date->tm_wday) * (60 * 60 * 24);

	// show
	show_records(db, begin, end);

	// close db
	sqlite3_close(db);
}
