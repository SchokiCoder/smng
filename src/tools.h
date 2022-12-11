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

#ifndef TOOLS_H
#define TOOLS_H

#include <stdint.h>
#include <stdbool.h>
#include <time.h>
#include "config.h"

typedef struct sqlite3 sqlite3;
typedef enum Command Command;

void print_cmd_help(const Command cmd);

int32_t database_connect(sqlite3 ** db);

uint8_t is_prev_record_done(sqlite3 * db, uint32_t * record_id,
			    bool *record_done);

uint8_t show_records(sqlite3 * db, const time_t begin, const time_t end);

int32_t parse_id(sqlite3 * db, const int32_t raw, const bool is_project,
		 int32_t * result);

#ifdef SANITIZE_DATETIME
int32_t sanitize_datetime(const int16_t year, const int8_t month,
			  const int8_t day, const int8_t hour,
			  const int8_t minute);
#endif

#endif				/* TOOLS_H */