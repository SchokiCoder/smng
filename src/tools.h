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

#ifndef TOOLS_H
#define TOOLS_H

#include <stdint.h>
#include <stdbool.h>
#include <time.h>
#include <sqlite3.h>

int32_t database_connect(sqlite3 **p_db);

uint8_t is_prev_record_done(sqlite3 *p_db, uint32_t *p_work_record_id, bool *p_work_record_done);

uint8_t show_records(sqlite3 *p_db, time_t p_begin, time_t p_end);

int32_t parse_id(sqlite3 *p_db, int32_t p_raw, bool p_is_project, int32_t *p_result);

int32_t sanitize_datetime(int16_t p_year, int8_t p_month, int8_t p_day, int8_t p_hour, int8_t p_minute);

#endif /* TOOLS_H */
