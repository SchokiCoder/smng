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

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include "constants.h"
#include "tools.h"
#include "commands.h"

int main(int argc, char *argv[])
{
    //if no command given, print help and end
    if (argc < 2)
    {
        printf("Usage: %s COMMAND [OPTIONS]\nTry '%s %s' for more information.\n", APP_NAME, APP_NAME, CMD_HELP_LONG);
        return 0;
    }

    //parse given command
    if ((strcmp(argv[1], CMD_HELP) == 0) |
        (strcmp(argv[1], CMD_HELP_LONG) == 0))
    {
        //check max arg count
        if (argc > 2)
        {
            printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
        }
        
        //exec
        cmd_help();
    }
    else if ((strcmp(argv[1], CMD_ADD_PROJECT) == 0) |
        (strcmp(argv[1], CMD_ADD_PROJECT_LONG) == 0))
    {        
        //check max arg count
        if (argc > 3)
        {
            printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
        }

        //check min arg count
        else if (argc < 3)
        {
            printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
            return 0;
        }
        
        //exec
        cmd_add_project(argv[2]);
    }
    else if ((strcmp(argv[1], CMD_SHOW_PROJECTS) == 0) |
        (strcmp(argv[1], CMD_SHOW_PROJECTS_LONG) == 0))
    {
        //check max arg count
        if (argc > 2)
        {
            printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
        }
        
        //exec
        cmd_show_projects();
    }
    else if ((strcmp(argv[1], CMD_EDIT_PROJECT) == 0) |
        (strcmp(argv[1], CMD_EDIT_PROJECT_LONG) == 0))
    {
        int32_t id;

        //check max arg count
        if (argc > 4)
        {
            printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
        }

        //check min arg count
        else if (argc < 4)
        {
            printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
            return 0;
        }
        
        //parse args
        id = strtol(argv[2], NULL, 10);

        //exec
        cmd_edit_project(id, argv[3]);
    }
    else if ((strcmp(argv[1], CMD_RECORD) == 0) |
        (strcmp(argv[1], CMD_RECORD_LONG) == 0))
    {
        int32_t id;

        //check max arg count
        if (argc > 3)
        {
            printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
        }

        //check min arg count
        else if (argc < 3)
        {
            printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
            return 0;
        }
        
        //parse args
        id = strtol(argv[2], NULL, 10);
        
        //exec
        cmd_record(id);
    }
    else if((strcmp(argv[1], CMD_STOP) == 0) |
        (strcmp(argv[1], CMD_STOP_LONG) == 0))
    {
        //check max arg count
        if (argc > 3)
        {
            printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
        }

        //check min arg count
        else if (argc < 3)
        {
            printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
            return 0;
        }
        
        //exec
        cmd_stop(argv[2]);
    }
    else if((strcmp(argv[1], CMD_EDIT_RECORD_PROJECT) == 0) |
        (strcmp(argv[1], CMD_EDIT_RECORD_PROJECT_LONG) == 0))
    {
        int32_t id[2];

        //check max arg count
        if (argc > 4)
        {
            printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
        }

        //check min arg count
        else if (argc < 4)
        {
            printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
            return 0;
        }
        
        //parse args
        id[0] = strtol(argv[2], NULL, 10);
        id[1] = strtol(argv[3], NULL, 10);

        //exec
        cmd_edit_record_project(id[0],id[1]);
    }
    else if((strcmp(argv[1], CMD_EDIT_RECORD_BEGIN) == 0) |
        (strcmp(argv[1], CMD_EDIT_RECORD_BEGIN_LONG) == 0) |
        (strcmp(argv[1], CMD_EDIT_RECORD_END) == 0) |
        (strcmp(argv[1], CMD_EDIT_RECORD_END_LONG) == 0))
    {
        int32_t id;
        bool edit_begin = true;
        uint16_t year;
        uint8_t month;
        uint8_t day;
        uint8_t hour;
        uint8_t minute;

        //check max arg count
        if (argc > 8)
        {
            printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
        }

        //check min arg count
        else if (argc < 8)
        {
            printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
            return 0;
        }

        //parse args
        id = strtoul(argv[2], NULL, 10);
        year = strtoul(argv[3], NULL, 10);
        month = strtoul(argv[4], NULL, 10);
        day = strtoul(argv[5], NULL, 10);
        hour = strtoul(argv[6], NULL, 10);
        minute = strtoul(argv[7], NULL, 10);

        
        if ((strcmp(argv[1], CMD_EDIT_RECORD_END) == 0) |
            (strcmp(argv[1], CMD_EDIT_RECORD_END_LONG) == 0))
        {
            edit_begin = false;
        }

        //exec
        cmd_edit_record_time(edit_begin, id, year, month, day, hour, minute);
    }
    else if((strcmp(argv[1], CMD_EDIT_RECORD_DESC) == 0) |
        (strcmp(argv[1], CMD_EDIT_RECORD_DESC_LONG) == 0))
    {
        int32_t id;
        
        //check max arg count
        if (argc > 4)
        {
            printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
        }

        //check min arg count
        else if (argc < 4)
        {
            printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
            return 0;
        }

        //parse args
        id = strtol(argv[2], NULL, 10);

        //exec
        cmd_edit_record_description(id, argv[3]);
    }
    else if((strcmp(argv[1], CMD_SHOW_WEEK) == 0) |
        (strcmp(argv[1], CMD_SHOW_WEEK_LONG) == 0))
    {
        //check max arg count
        if (argc > 2)
        {
            printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
        }
        
        //exec
        cmd_show_records_week();
    }
    else if((strcmp(argv[1], CMD_SHOW_MONTH) == 0) |
        (strcmp(argv[1], CMD_SHOW_MONTH_LONG) == 0))
    {
        int8_t month;
        int8_t year;

        //check max arg count
        if (argc > 4)
        {
            printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
        }
        
        //parse args
        if (argc > 2)
            month = strtol(argv[2], NULL, 10);
        else
            month = -1;

        if (argc > 3)
            year = strtol(argv[3], NULL, 10);
        else
            year = -1;
        
        //exec
        cmd_show_records_month(month, year);
    }
    else
    {
        //unknown command passed, print
        printf("Command not recognised.\nType '%s %s' for information on usage.\n", APP_NAME, CMD_HELP_LONG);
    }

    return 0;
}
