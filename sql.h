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

#ifndef SQL_H
#define SQL_H

#include "constants.h"
#include "config.h"

#define SQL_ACTIVATE_FKEYS "PRAGMA foreign_keys = ON;"

#define SQL_CREATE_WORKRECORDS "CREATE TABLE tbl_work_records(" \
"work_record_id INTEGER PRIMARY KEY, " \
"project_id INTEGER NOT NULL REFERENCES tbl_projects(project_id), " \
"begin INTEGER NOT NULL, " \
"end INTEGER, " \
"description VARCHAR(" WORK_DESCRIPTION_MAXLENGTH ")" \
");"

#define SQL_CREATE_PROJECTS "CREATE TABLE tbl_projects(" \
"project_id INTEGER PRIMARY KEY, " \
"project_name VARCHAR(" PROJECT_NAME_MAXLENGTH ") NOT NULL UNIQUE" \
");"

#define SQL_CREATE_INDEXES "CREATE INDEX idx_work_record_id ON tbl_work_records(work_record_id);" \
"CREATE INDEX idx_project_id ON tbl_projects(project_id);" \
"CREATE INDEX idx_begin ON tbl_work_records(begin);" \
"CREATE INDEX idx_end ON tbl_work_records(end);"

#define SQL_ADD_PROJECT "INSERT INTO tbl_projects(project_name)\n" \
"VALUES (?);"

#define SQL_SHOW_PROJECTS "SELECT project_id, project_name\n" \
"FROM tbl_projects;"

#define SQL_EDIT_PROJECT "UPDATE tbl_projects\n" \
"SET project_name = ?\n" \
"WHERE project_id = ?;"

#define SQL_CHECK_PREVIOUS_RECORD "SELECT work_record_id, (CASE WHEN end IS NULL THEN 0 ELSE 1 END) as record_valid\n" \
"FROM tbl_work_records\n" \
"ORDER BY work_record_id DESC LIMIT 1;"

#define SQL_START_WORK_RECORD "INSERT INTO tbl_work_records (project_id, begin)\n" \
"VALUES (?, ?);"

#define SQL_FINISH_WORK_RECORD "UPDATE tbl_work_records\n" \
"SET end = ?, description = ?\n" \
"WHERE work_record_id = (SELECT MAX(work_record_id) FROM tbl_work_records);"

#define SQL_EDIT_RECORD_PROJECT "UPDATE tbl_work_records\n" \
"SET project_id = ?\n" \
"WHERE work_record_id = ?;"

#define SQL_EDIT_RECORD_BEGIN "UPDATE tbl_work_records\n" \
"SET begin = ?\n" \
"WHERE work_record_id = ?;"

#define SQL_EDIT_RECORD_END "UPDATE tbl_work_records\n" \
"SET end = ?\n" \
"WHERE work_record_id = ?;"

#define SQL_EDIT_RECORD_DESC "UPDATE tbl_work_records\n" \
"SET description = ?\n" \
"WHERE work_record_id = ?;"

#define SQL_SHOW_RECORDS "SELECT work_record_id, " \
"strftime('%d %H:%M', begin, 'unixepoch', 'localtime'), strftime('%d %H:%M', end, 'unixepoch', 'localtime'), " \
"(end - begin) AS seconds, " \
"project_id, description\n" \
"FROM tbl_work_records\n" \
"WHERE begin > ? AND end < ?;"

#define SQL_MAX_RECORD_ID "SELECT MAX(work_record_id)\n" \
"FROM tbl_work_records;"

#define SQL_MAX_PROJECT_ID "SELECT MAX(project_id)\n" \
"FROM tbl_projects;"

#endif
