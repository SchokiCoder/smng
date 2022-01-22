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

#ifndef CONFIG_H
#define CONFIG_H

/*
    Disallows datetime values that may are incorrect.
*/
#define DISALLOW_WEIRD_DATETIME

/* 
    If weird datetime is disallowed.
    Defines what range of datetime values is accepted.
*/
#ifdef DISALLOW_WEIRD_DATETIME
    #define DT_YEAR_MAX     2100
    #define DT_YEAR_MIN     2000
    #define DT_MONTH_MAX    12
    #define DT_MONTH_MIN    1
    #define DT_DAY_MAX      31
    #define DT_DAY_MIN      1
    #define DT_HOUR_MAX     23
    #define DT_HOUR_MIN     0
    #define DT_MINUTE_MAX   59
    #define DT_MINUTE_MIN   0
#endif

/* database creation definitions (does not affect existing db) */
#define WORK_DESCRIPTION_MAXLENGTH "50"
#define PROJECT_NAME_MAXLENGTH "32"

/* 
    Uncomment the following definition to use a static path to the database.
    In that case do not forget to actually define the database path below.
    Otherwise path will be "/home/" + user_name + "/.smng/worktimes.db",
    which is constructed at runtime.
*/
// #define STATIC_DATABASE_PATH

#ifdef STATIC_DATABASE_PATH
    #define PATH_DATABASE ""
#endif

#endif
