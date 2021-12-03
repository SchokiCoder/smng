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

#ifndef COMMANDS_H
#define COMMANDS_H

#include <stdint.h>
#include <stdbool.h>

void cmd_help();

void cmd_add_project(char* p_project_name);

void cmd_show_projects();

void cmd_edit_project(int32_t p_project_id, char* p_project_name);

void cmd_record(int32_t p_project_id);

void cmd_stop(char* p_description);

void cmd_edit_record_project(int32_t p_work_record_id, int32_t p_project_id);

void cmd_edit_record_time(
    bool p_work_record_begin, int32_t p_work_record_id,
    int8_t p_hour, int8_t p_minute,
    int16_t p_year, int8_t p_month, int8_t p_day);

void cmd_edit_record_description(int32_t p_work_record_id, char* p_desc);

void cmd_show_records_month(int8_t p_month, int16_t p_year);

void cmd_show_records_week();

#endif
