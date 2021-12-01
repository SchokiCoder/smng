/*
    SchokiManager
    Copyright (C) 2021  Andy Frank Schoknecht

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

void cmd_help()
{
    //print help text
    printf("%s", HELP_TEXT);
}

void cmd_add_project(char* p_project_name)
{
    sqlite3* db;
    sqlite3_stmt* stmt;
    int32_t rc_prepare;
    int32_t rc_step;
    int32_t rc_bind;

    //try open connection
    if (database_connect(&db) != 0)
        return;

    //prepare
    rc_prepare = sqlite3_prepare_v2(db, SQL_ADD_PROJECT, -1, &stmt, 0);

    //try binding parameters
    rc_bind = sqlite3_bind_text(stmt, 1, p_project_name, -1, NULL);

    if ((rc_prepare != SQLITE_OK) |
        (rc_bind != SQLITE_OK))
    {
        //if not ok, error and stop
        printf("ERROR: Statement to add project could not be prepared. (%i, %i)\n", rc_prepare, rc_bind);
        sqlite3_finalize(stmt);
        sqlite3_close(db);
        return;
    }

    //try to execute
    rc_step = sqlite3_step(stmt);

    //catch constraint failure
    switch (rc_step)
    {
        case SQLITE_CONSTRAINT:
        printf("ERROR: Statement failed on a constraint. (%i)\nMake sure \"%s\" is not already used as a project name.\n", rc_step, p_project_name);
        break;

        case SQLITE_DONE:
        printf("Project \"%s\" added.\n", p_project_name);
        break;

        default:
        printf("ERROR: Unknown error. Code RED, burn the evidence and run! (%i)\n", rc_step);
    }

    //clean
    sqlite3_finalize(stmt);
    sqlite3_close(db);
}

void cmd_show_projects()
{
    sqlite3* db;
    sqlite3_stmt* stmt;
    int32_t rc_prepare;
    int32_t rc_step;

    //try open connection
    if (database_connect(&db) != 0)
        return;

    //prepare
    rc_prepare = sqlite3_prepare_v2(db, SQL_SHOW_PROJECTS, -1, &stmt, 0);

    //if not ok, print error and stop
    if (rc_prepare != SQLITE_OK)
    {
        printf("ERROR: Statement to show projects could not be prepared. (%i)\n", rc_prepare);
        sqlite3_close(db);
    }

    //if results are incoming, print header
    rc_step = sqlite3_step(stmt);

    if (rc_step == SQLITE_ROW)
    {
        printf("project_id, project_name\n");

        do
        {
            //print results
            printf("%i, %s\n", sqlite3_column_int(stmt, 0), sqlite3_column_text(stmt, 1));
        }
        while ((rc_step = sqlite3_step(stmt)) == SQLITE_ROW);
    }
    else
        printf("No projects available.\n");

    //clean
    sqlite3_finalize(stmt);
    sqlite3_close(db);
}

void cmd_edit_project(int32_t p_project_id, char* p_project_name)
{
    sqlite3* db;
    sqlite3_stmt* stmt;
    int32_t id;
    int32_t rc_prepare;
    int32_t rc_bind[2];
    int32_t rc_step;

    //connect to db
    if (database_connect(&db) != 0)
        return;

    //prepare
    rc_prepare = sqlite3_prepare_v2(db, SQL_EDIT_PROJECT, -1, &stmt, 0);

    //bind parameters
    if (parse_id(db, p_project_id, true, &id) != 0)
    {
        sqlite3_finalize(stmt);
        sqlite3_close(db);
        return;
    }

    rc_bind[0] = sqlite3_bind_text(stmt, 1, p_project_name, -1, NULL);
    rc_bind[1] = sqlite3_bind_int(stmt, 2, id);

    //if prepare failed, print error
    if ((rc_prepare != SQLITE_OK) |
        (rc_bind[0] != SQLITE_OK) |
        (rc_bind[1] != SQLITE_OK))
    {
        printf("ERROR: Statement to edit project could not be prepared. (%i, %i, %i)\n", rc_prepare, rc_bind[0], rc_bind[1]);
        sqlite3_finalize(stmt);
        sqlite3_close(db);
        return;
    }

    //execute
    rc_step = sqlite3_step(stmt);

    //catch constraint failure
    switch (rc_step)
    {
        case SQLITE_CONSTRAINT:
        printf("ERROR: Statement failed on a constraint. (%i)\nMake sure \"%s\" is not already used as a project name.\n", rc_step, p_project_name);
        break;

        case SQLITE_DONE:
        printf("Project %i name set to \"%s\".\n", id, p_project_name);
        break;

        default:
        printf("ERROR: Statement to edit project could not be executed. (%i)\n", rc_step);
    }

    //clean
    sqlite3_finalize(stmt);
    sqlite3_close(db);
}

void cmd_record(int32_t p_project_id)
{
    sqlite3* db;
    sqlite3_stmt* stmt;
    int32_t id;
    uint32_t last_record_id;
    uint8_t last_record_state;
    int32_t rc_prepare;
    int32_t rc_bind[2];
    int32_t rc_step;
    time_t record_begin;

    //connect
    if (database_connect(&db) != 0)
        return;
    
    //check validity of last work record
    if (check_last_work_record(db, &last_record_id, &last_record_state) != 0)
    {
        sqlite3_close(db);
        return;
    }

    if (last_record_state == 0)
    {
        //invalid record found, print
        printf("ERROR: Before starting a new work-record close the last one.\nlast work_record_id: %i\n", sqlite3_column_int(stmt, 0));
        sqlite3_close(db);
        return;
    }
    
    //add new work record
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
    
    if ((rc_prepare != SQLITE_OK) |
        (rc_bind[0] != SQLITE_OK) |
        (rc_bind[1] != SQLITE_OK))
    {
        printf("ERROR: Statement to add new work record could not be prepared. (%i, %i, %i)\n", rc_prepare, rc_bind[0], rc_bind[1]);
        sqlite3_finalize(stmt);
        sqlite3_close(db);
        return;
    }

    rc_step = sqlite3_step(stmt);

    if (rc_step != SQLITE_DONE)
        printf("ERROR: Statement to add new  work record could not be executed. (%i)\n", rc_step);
    else
        printf("Work record for project %i added.\n", id);

    sqlite3_finalize(stmt);
    sqlite3_close(db);
}

void cmd_stop(char* p_description)
{
    sqlite3* db;
    sqlite3_stmt* stmt;
    int32_t rc_prepare;
    int32_t rc_bind[2];
    int32_t rc_step;
    uint8_t last_record_state;
    uint32_t last_record_id;
    time_t record_end;

    //try connect to db
    if (database_connect(&db) != 0)
        return;

    //check validity of last work record
    if (check_last_work_record(db, &last_record_id, &last_record_state) != 0)
    {
        sqlite3_close(db);
        return;
    }

    //if last work record is closed, print and stop
    if (last_record_state == 1)
    {
        printf("ERROR: Before stopping a work-record, there should be an unfinished record.\nlast work_record_id: %i\n", last_record_id);
        sqlite3_close(db);
        return;
    }

    //update latest work record
    time(&record_end);

    rc_prepare = sqlite3_prepare_v2(db, SQL_FINISH_WORK_RECORD, -1, &stmt, 0);
    rc_bind[0] = sqlite3_bind_int(stmt, 1, record_end);
    rc_bind[1] = sqlite3_bind_text(stmt, 2, p_description, -1, NULL); 

    if ((rc_prepare != SQLITE_OK) |
        (rc_bind[0] != SQLITE_OK) |
        (rc_bind[1] != SQLITE_OK))
    {
        printf("ERROR: Statement to finish the work-record could not be prepared. (%i)\n", rc_prepare);
        sqlite3_close(db);
        return;
    }

    rc_step = sqlite3_step(stmt);

    if (rc_step != SQLITE_DONE)
        printf("ERROR: Statement to finish the work record could not be executed. (%i)\n", rc_step);
    else
        printf("Work record finished with following description:\n%s\n", p_description);

    sqlite3_finalize(stmt);
    sqlite3_close(db);
}

void cmd_edit_record_project(int32_t p_work_record_id, int32_t p_project_id)
{
    sqlite3* db;
    sqlite3_stmt* stmt;
    int32_t rec_id;
    int32_t pro_id;
    int32_t rc_prepare;
    int32_t rc_bind[2];
    int32_t rc_step;

    //connect
    if (database_connect(&db) != 0)
        return;

    //update record
    rc_prepare = sqlite3_prepare_v2(db, SQL_EDIT_RECORD_PROJECT, -1, &stmt, 0);

    if ((parse_id(db, p_project_id, true, &pro_id) != 0) |
        (parse_id(db, p_work_record_id, false, &rec_id) != 0))
    {
        sqlite3_finalize(stmt);
        sqlite3_close(db);
        return;
    }

    rc_bind[0] = sqlite3_bind_int(stmt, 1, pro_id);
    rc_bind[1] = sqlite3_bind_int(stmt, 2, rec_id);

    if ((rc_prepare != SQLITE_OK) |
        (rc_bind[0] != SQLITE_OK) |
        (rc_bind[1] != SQLITE_OK))
    {
        printf("Statement to edit the work-record could not be prepared. (%i, %i, %i)\n", rc_prepare, rc_bind[0], rc_bind[1]);
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
        printf("ERROR: Statement failed on a constraint. (%i)\nMake sure project %i exists.\n", rc_step, p_project_id);
        break;

        default:
        printf("ERROR: Statement to edit the work-record could not be executed. (%i)\n", rc_step);
    }

    sqlite3_finalize(stmt);
    sqlite3_close(db);
}

void cmd_edit_record_time(
    bool p_work_record_begin, int32_t p_work_record_id,
    int8_t p_hour, int8_t p_minute,
    int16_t p_year, int8_t p_month, int8_t p_day)
{
    sqlite3* db;
    sqlite3_stmt* stmt;
    time_t ts_now;
    struct tm* now;
    time_t record_time;
    struct tm dt;
    int32_t id;
    int32_t rc_prepare;
    int32_t rc_bind[2];
    int32_t rc_step;
    
    //connect
    if (database_connect(&db) != 0)
        return;
    
    //parse datetime arguments and convert to unixepoch
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

    //prepare sql
    if (parse_id(db, p_work_record_id, false, &id) != 0)
    {
        sqlite3_close(db);
        return;
    }

    rc_prepare = sqlite3_prepare_v2(db, (p_work_record_begin == true ? SQL_EDIT_RECORD_BEGIN : SQL_EDIT_RECORD_END), -1, &stmt, 0);
    rc_bind[0] = sqlite3_bind_int(stmt, 1, record_time);
    rc_bind[1] = sqlite3_bind_int(stmt, 2, id);

    if ((rc_prepare != SQLITE_OK) |
        (rc_bind[0] != SQLITE_OK) |
        (rc_bind[1] != SQLITE_OK))
    {
        printf("Statement to edit the work-record could not be prepared. (%i, %i, %i)\n", rc_prepare, rc_bind[0], rc_bind[1]);
        sqlite3_finalize(stmt);
        sqlite3_close(db);
        return;
    }

    //exec sql
    rc_step = sqlite3_step(stmt);

    if (rc_step != SQLITE_DONE)
        printf("Statement to edit work-record could not be executed. (%i)\n", rc_step);
    else
        printf("Record %i project %s set to %i-%i-%i %i:%i.\n",
            id, (p_work_record_begin == true ? "begin" : "end"), (1900 + dt.tm_year), (1 + dt.tm_mon), dt.tm_mday, p_hour, p_minute);

    sqlite3_finalize(stmt);
    sqlite3_close(db);
}

void cmd_edit_record_description(int32_t p_work_record_id, char* p_desc)
{
    sqlite3* db;
    sqlite3_stmt* stmt;
    int32_t id;
    int32_t rc_prep, rc_step, rc_bind[2];

    //connect
    if (database_connect(&db) != 0)
        return;

    //prepare
    if (parse_id(db, p_work_record_id, false, &id) != 0)
    {
        sqlite3_close(db);
        return;
    }

    rc_prep = sqlite3_prepare_v2(db, SQL_EDIT_RECORD_DESC, -1, &stmt, 0);
    rc_bind[0] = sqlite3_bind_text(stmt, 1, p_desc, -1, NULL);
    rc_bind[1] = sqlite3_bind_int(stmt, 2, id);

    if ((rc_prep != SQLITE_OK) |
        (rc_bind[0] != SQLITE_OK) |
        (rc_bind[1] != SQLITE_OK))
    {
        printf("Statement to edit record description could not be prepared. (%i, %i, %i)\n", rc_prep, rc_bind[0], rc_bind[1]);
        sqlite3_finalize(stmt);
        sqlite3_close(db);
        return;
    }

    //exec
    rc_step = sqlite3_step(stmt);

    if (rc_step != SQLITE_DONE)
        printf("Statement to edit record description could not be executed. (%i)\n", rc_step);
    else
        printf("Record %i description changed to:\n%s\n", id, p_desc);

    sqlite3_finalize(stmt);
    sqlite3_close(db);
}

void cmd_show_records_month(int8_t p_month, int16_t p_year)
{
    sqlite3* db;
    time_t begin, end;
    struct tm temp_begin, temp_end;
    struct tm* now;
    time_t ts_now;

    //connect to db
    if (database_connect(&db) != 0)
        return;

    //calculate begin and end in sec's since epoch
    time(&ts_now);
    now = localtime(&ts_now);

    if (p_year < 0)
    {
        temp_begin.tm_year = now->tm_year;
        temp_end.tm_year = now->tm_year;
    }
    else
    {
        temp_begin.tm_year = p_year - 1900;
        temp_end.tm_year = p_year -1900;
    }

    if (p_month < 0)
    {
        temp_begin.tm_mon = now->tm_mon;
        temp_end.tm_mon = now->tm_mon + 1;
    }
    else
    {
        temp_begin.tm_mon = p_month - 1;
        temp_end.tm_mon = p_month;
    }

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

    //show
    show_records(db, begin, end);
    
    //clean
    sqlite3_close(db);
}

void cmd_show_records_week()
{
    sqlite3* db;
    time_t begin, end;
    struct tm* temp;
    
    //connect
    if (database_connect(&db) != 0)
        return;

    //get today and time
    time(&begin);
    temp = localtime(&begin);

    temp->tm_hour = 0;
    temp->tm_min = 0;
    temp->tm_sec = 0;
    temp->tm_isdst = -1;
    begin = mktime(temp);

    temp->tm_hour = 23;
    temp->tm_min = 59;
    temp->tm_sec = 59;
    temp->tm_isdst = -1;
    end = mktime(temp);

    //find begin and end of week
    begin -= (temp->tm_wday) * (60 * 60 * 24);
    end += (6 - temp->tm_wday) * (60 * 60 * 24);

    //show
    show_records(db, begin, end);

    sqlite3_close(db);
}
