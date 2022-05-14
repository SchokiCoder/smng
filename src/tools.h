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

#include <time.h>
#include "config.h"

typedef struct sqlite3 sqlite3;
typedef enum Command Command;

void print_cmd_help( const Command cmd );

sl32_t database_connect( sqlite3 **db );

ul8_t is_prev_record_done( sqlite3 *db, sl32_t *record_id, bool_t *record_done );

ul8_t show_records( sqlite3 *db, const time_t begin, const time_t end );

sl32_t parse_id( sqlite3 *db, const sl32_t raw, const bool_t is_project, sl32_t *result );

#ifdef SANITIZE_DATETIME
sl32_t sanitize_datetime(
	const sl16_t year, const sl8_t month, const sl8_t day,
	const sl8_t hour, const sl8_t minute );
#endif

#endif /* TOOLS_H */
