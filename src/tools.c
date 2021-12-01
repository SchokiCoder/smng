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

#include "tools.h"
#include "config.h"
#include "sql.h"
#include <stdio.h>

int32_t database_connect(sqlite3** p_db)
{
    int32_t rc_connect;
    int32_t rc_empty;
    int32_t rc_activate_fkeys;
    int32_t rc_create_work_records;
    int32_t rc_create_projects;
    int32_t rc_create_indexes;

    //try connecting to db
    rc_connect = sqlite3_open(DATABASE_PATH, p_db);

    //if no connection possible, end
    if (rc_connect != SQLITE_OK)
    {
        printf("ERROR: The database is missing or access is not given. (%i)\n"
            "Check the path in the config and make sure permissions are not missing.\n", rc_connect);
        return 1;
    }

    //if db is empty, create tables etc.
    rc_empty = sqlite3_table_column_metadata(*p_db, NULL, "tbl_work_records", "work_record_id", NULL, NULL, NULL, NULL, NULL);

    if (rc_empty == 1)
    {
        rc_activate_fkeys = sqlite3_exec(*p_db, SQL_ACTIVATE_FKEYS, NULL, NULL, NULL);
        rc_create_work_records = sqlite3_exec(*p_db, SQL_CREATE_WORKRECORDS, NULL, NULL, NULL);
        rc_create_projects = sqlite3_exec(*p_db, SQL_CREATE_PROJECTS, NULL, NULL, NULL);
        rc_create_indexes = sqlite3_exec(*p_db, SQL_CREATE_INDEXES, NULL, NULL, NULL);
    }

    //else activate fkeys and end
    else
    {
        rc_activate_fkeys = sqlite3_exec(*p_db, SQL_ACTIVATE_FKEYS, NULL, NULL, NULL);

        if (rc_activate_fkeys != SQLITE_OK)
        {
            printf("ERROR: The activation of foreign keys in this database did not work. (%i)\n", rc_activate_fkeys);
            return 3;
        }

        return 0;
    }

    //if db creation fails, print error, end
    if ((rc_activate_fkeys != SQLITE_OK) |
        (rc_create_work_records != SQLITE_OK) |
        (rc_create_projects != SQLITE_OK) |
        (rc_create_indexes != SQLITE_OK))
    {
        printf("ERROR: The database was missing and an attempt to create it failed.\n");
        sqlite3_close(*p_db);
        *p_db = NULL;
        return 2;
    }

    //on success, print warning
    printf("WARNING: The database was missing and a new one was created.\n");
    return 0;
}

uint8_t check_last_work_record(sqlite3* p_db, uint32_t* p_work_record_id, uint8_t* p_work_record_state)
{
    sqlite3_stmt* stmt;
    int32_t rc_prepare;
    int32_t rc_step;
    
    //check if there is an open record left
    rc_prepare = sqlite3_prepare_v2(p_db, SQL_CHECK_PREVIOUS_RECORD, -1, &stmt, 0);

    if (rc_prepare != SQLITE_OK)
    {
        printf("ERROR: Statement to check validity of previous record could not prepared. (%i)\n", rc_prepare);
        return 1;
    }

    rc_step = sqlite3_step(stmt);

    if (rc_step != SQLITE_ROW)
    {
        printf("ERROR: Statement to check validity of previous record could not be executed. (%i)\n", rc_step);
        sqlite3_finalize(stmt);
        return 2;
    }

    //save values to output pointers
    *p_work_record_id = sqlite3_column_int(stmt, 0);
    *p_work_record_state = (uint8_t) (sqlite3_column_int(stmt, 1));
    
    sqlite3_finalize(stmt);
    return 0;
}

uint8_t show_records(sqlite3* p_db, time_t p_begin, time_t p_end)
{
    sqlite3_stmt* stmt;
    int32_t rc_prepare;
    int32_t rc_bind[2];
    int32_t rc_step;
    char timespan[2][14];
    struct tm* temp;
    uint32_t hours, minutes, seconds;
    uint32_t sum_seconds = 0;
    uint32_t sum_hours, sum_minutes;
    
    //prepare sql
    rc_prepare = sqlite3_prepare_v2(p_db, SQL_SHOW_RECORDS, -1, &stmt, 0);
    rc_bind[0] = sqlite3_bind_int(stmt, 1, p_begin);
    rc_bind[1] = sqlite3_bind_int(stmt, 2, p_end);

    if ((rc_prepare != SQLITE_OK) |
        (rc_bind[0] != SQLITE_OK) |
        (rc_bind[1] != SQLITE_OK))
    {
        printf("Statement to show records could not be prepared. (%i)\n", rc_prepare);
        sqlite3_finalize(stmt);
        return 1;
    }

    //if results are incoming
    rc_step = sqlite3_step(stmt);

    if (rc_step == SQLITE_ROW)
    {
        //print header
        temp = localtime(&p_begin);
        strftime(timespan[0], sizeof(timespan[0]), "%Y-%m-%d", temp);

        temp = localtime(&p_end);
        strftime(timespan[1], sizeof(timespan[1]), "%Y-%m-%d", temp);

        printf("Summarize from %s to %s:\n\nrecord_id, begin, end, time, project_id, description\n", 
            timespan[0],
            timespan[1]);

        do
        {
            //convert seconds to hours and minutes
            seconds = sqlite3_column_int(stmt, 3);
            sum_seconds += seconds;
            minutes = seconds / 60;
            hours = minutes / 60;
            minutes = minutes % 60;

            //print results
            printf("%i, %s, %s, %i:%i, %i, %s\n",
                sqlite3_column_int(stmt, 0),
                sqlite3_column_text(stmt, 1),
                sqlite3_column_text(stmt, 2),
                hours, minutes,
                sqlite3_column_int(stmt, 4),
                sqlite3_column_text(stmt, 5));
        }
        while ((rc_step = sqlite3_step(stmt)) == SQLITE_ROW);

        //print sum time
        sum_minutes = sum_seconds / 60;
        sum_hours = sum_minutes / 60;
        sum_minutes = sum_minutes % 60;
        printf("\nSummarized work time: %i:%i\n", sum_hours, sum_minutes);
    }
    else if (rc_step == SQLITE_DONE)
        printf("No records available.\n");
    else
    {
        printf("Statement to show work-records could not be executed. (%i)\n", rc_step);
        sqlite3_finalize(stmt);
        return 2;
    }

    //clean
    sqlite3_finalize(stmt);
    return 0;
}

int32_t parse_id(sqlite3* p_db, int32_t p_raw, bool p_is_project, int32_t *p_result)
{
    sqlite3_stmt* stmt;
    int32_t rc_prep;
    int32_t rc_step;

    //if number is negative
    if (p_raw < 0)
    {
        //find real id
        if (p_is_project == true)
            rc_prep = sqlite3_prepare_v2(p_db, SQL_MAX_PROJECT_ID, -1, &stmt, 0);
        else
            rc_prep = sqlite3_prepare_v2(p_db, SQL_MAX_RECORD_ID, -1, &stmt, 0);

        if (rc_prep != SQLITE_OK)
        {
            printf("Statement to find real id could not be prepared. (%i)\n", rc_prep);
            sqlite3_finalize(stmt);
            return 1;
        }

        rc_step = sqlite3_step(stmt);

        if (rc_step != SQLITE_ROW)
        {
            printf("Statement to find real id could not be executed. (%i)\n", rc_step);
            sqlite3_finalize(stmt);
            return 2;
        }

        *p_result = sqlite3_column_int(stmt, 0) + 1 + p_raw;
        sqlite3_finalize(stmt);
    }
    else
    {
        *p_result = p_raw;
    }
    
    return  0;
}
