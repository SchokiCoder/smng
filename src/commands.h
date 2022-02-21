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

void cmd_transfer_project_records(int32_t p_old_project_id, int32_t p_new_project_id);

void cmd_show_records_month(int16_t p_year, int8_t p_month);

void cmd_show_records_week(int16_t p_year, int8_t p_month, int8_t p_day);

#endif /* COMMANDS_H */
