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
	uint32_t cmd_hash;

	// if no command given, print help and end
	if (argc < 2)
	{
		printf(
			"Usage: %s COMMAND [OPTIONS]\nTry '%s %s' for more information.\n",
			APP_NAME, APP_NAME, CMD_HELP_LONG);
		return 0;
	}

	// hash given command
	cmd_hash = djb2_hash(argv[1]);

	// execute command
	if ((djb2_hash(CMD_HELP) == cmd_hash) ||
		(djb2_hash(CMD_HELP_LONG) == cmd_hash))
	{
		// check max arg count
		if (argc > 2)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// exec
		cmd_help();
	}
	else if ((djb2_hash(CMD_ADD_PROJECT) == cmd_hash) ||
		(djb2_hash(CMD_ADD_PROJECT_LONG) == cmd_hash))
	{
		// check max arg count
		if (argc > 3)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			printf(MSG_HELP_ADD_PROJECT, CMD_ADD_PROJECT, CMD_ADD_PROJECT_LONG);
			printf("\n");
			return 0;
		}

		/*check min arg count
		else if (argc < 3)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
			return 0;
		}*/

		// exec
		cmd_add_project(argv[2]);
	}
	else if ((djb2_hash(CMD_SHOW_PROJECTS) == cmd_hash) ||
		(djb2_hash(CMD_SHOW_PROJECTS_LONG) == cmd_hash))
	{
		// check max arg count
		if (argc > 2)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// exec
		cmd_show_projects();
	}
	else if ((djb2_hash(CMD_EDIT_PROJECT) == cmd_hash) ||
		(djb2_hash(CMD_EDIT_PROJECT_LONG) == cmd_hash))
	{
		int32_t id;

		// check max arg count
		if (argc > 4)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			printf(MSG_HELP_EDIT_PROJECT, CMD_EDIT_PROJECT, CMD_EDIT_PROJECT_LONG);
			printf("\n");
			return 0;
		}

		// check min arg count
		else if (argc < 4)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
			return 0;
		}

		// parse args
		id = strtol(argv[2], NULL, 10);

		// exec
		cmd_edit_project(id, argv[3]);
	}
	else if(djb2_hash(CMD_DELETE_PROJECT_LONG) == cmd_hash)
	{
		int32_t id;
		bool force = false;

		// check max arg count
		if (argc > 4)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			printf(MSG_HELP_DELETE_PROJECT, CMD_DELETE_PROJECT_LONG);
			printf("\n");
			return 0;
		}

		/*check min arg count
		else if (argc < 3)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
			return 0;
		}*/

		// parse args
		id = strtol(argv[2], NULL, 10);

		if (argc > 3)
			force = strtoul(argv[3], NULL, 10);

		// exec
		cmd_delete_project(id, force);
	}
	else if ((djb2_hash(CMD_RECORD) == cmd_hash) ||
		(djb2_hash(CMD_RECORD_LONG) == cmd_hash))
	{
		int32_t id;

		// check max arg count
		if (argc > 3)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			printf(MSG_HELP_RECORD, CMD_RECORD, CMD_RECORD_LONG);
			printf("\n");
			return 0;
		}

		/*check min arg count
		else if (argc < 3)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
			return 0;
		}*/

		// parse args
		id = strtol(argv[2], NULL, 10);

		// exec
		cmd_record(id);
	}
	else if (djb2_hash(CMD_STATUS_LONG) == cmd_hash)
	{
		// check max arg count
		if (argc > 2)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// exec
		cmd_status();
	}
	else if((djb2_hash(CMD_STOP) == cmd_hash) ||
		(djb2_hash(CMD_STOP_LONG) == cmd_hash))
	{
		// check max arg count
		if (argc > 3)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			printf(MSG_HELP_STOP, CMD_STOP, CMD_STOP_LONG);
			printf("\n");
			return 0;
		}

		/*check min arg count
		else if (argc < 3)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
			return 0;
		}*/

		// exec
		cmd_stop(argv[2]);
	}
	else if((djb2_hash(CMD_EDIT_RECORD_PROJECT) == cmd_hash) ||
		(djb2_hash(CMD_EDIT_RECORD_PROJECT_LONG) == cmd_hash))
	{
		int32_t id[2];

		// check max arg count
		if (argc > 4)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			printf(MSG_HELP_EDIT_RECORD_PROJECT, CMD_EDIT_RECORD_PROJECT, CMD_EDIT_RECORD_PROJECT_LONG);
			printf("\n");
			return 0;
		}

		// check min arg count
		else if (argc < 4)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
			return 0;
		}

		// parse args
		id[0] = strtol(argv[2], NULL, 10);
		id[1] = strtol(argv[3], NULL, 10);

		// exec
		cmd_edit_record_project(id[0],id[1]);
	}
	else if((djb2_hash(CMD_EDIT_RECORD_BEGIN) == cmd_hash) ||
		(djb2_hash(CMD_EDIT_RECORD_BEGIN_LONG) == cmd_hash) ||
		(djb2_hash(CMD_EDIT_RECORD_END) == cmd_hash) ||
		(djb2_hash(CMD_EDIT_RECORD_END_LONG) == cmd_hash))
	{
		int32_t id;
		bool edit_begin = true;
		uint16_t year;
		uint8_t month;
		uint8_t day;
		uint8_t hour;
		uint8_t minute;

		// check if user edits begin or end, save in flag
		if ((djb2_hash(CMD_EDIT_RECORD_END) == cmd_hash) ||
			(djb2_hash(CMD_EDIT_RECORD_END_LONG) == cmd_hash))
		{
			edit_begin = false;
		}

		// check max arg count
		if (argc > 8)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			if (edit_begin == true)
				printf(MSG_HELP_EDIT_RECORD_BEGIN, CMD_EDIT_RECORD_BEGIN, CMD_EDIT_RECORD_BEGIN_LONG);
			else
				printf(MSG_HELP_EDIT_RECORD_END, CMD_EDIT_RECORD_END, CMD_EDIT_RECORD_END_LONG);

			printf("\n");
			return 0;
		}

		// check min arg count
		else if (argc < 8)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
			return 0;
		}

		// parse args
		id = strtoul(argv[2], NULL, 10);

#ifdef MAJOR_DATEFORMAT
		hour = strtoul(argv[3], NULL, 10);
		minute = strtoul(argv[4], NULL, 10);
		day = strtoul(argv[5], NULL, 10);
		month = strtoul(argv[6], NULL, 10);
		year = strtoul(argv[7], NULL, 10);
#else
		year = strtoul(argv[3], NULL, 10);
		month = strtoul(argv[4], NULL, 10);
		day = strtoul(argv[5], NULL, 10);
		hour = strtoul(argv[6], NULL, 10);
		minute = strtoul(argv[7], NULL, 10);
#endif

		// exec
		cmd_edit_record_time(edit_begin, id, year, month, day, hour, minute);
	}
	else if((djb2_hash(CMD_EDIT_RECORD_DESC) == cmd_hash) ||
		(djb2_hash(CMD_EDIT_RECORD_DESC_LONG) == cmd_hash))
	{
		int32_t id;

		// check max arg count
		if (argc > 4)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			printf(MSG_HELP_EDIT_RECORD_DESC, CMD_EDIT_RECORD_DESC, CMD_EDIT_RECORD_DESC_LONG);
			printf("\n");
			return 0;
		}

		// check min arg count
		else if (argc < 4)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
			return 0;
		}

		// parse args
		id = strtol(argv[2], NULL, 10);

		// exec
		cmd_edit_record_description(id, argv[3]);
	}
	else if(djb2_hash(CMD_DELETE_RECORD_LONG) == cmd_hash)
	{
		int32_t id;

		// check max arg count
		if (argc > 3)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			printf(MSG_HELP_DELETE_RECORD, CMD_DELETE_RECORD_LONG);
			printf("\n");
			return 0;
		}

		/*check min arg count
		else if (argc < 3)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
			return 0;
		}*/

		// parse args
		id = strtol(argv[2], NULL, 10);

		// exec
		cmd_delete_record(id);
	}
	else if (djb2_hash(CMD_TRANSFER_PROJECT_RECORDS_LONG) == cmd_hash)
	{
		int32_t old_id, new_id;

		// check max arg count
		if (argc > 4)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			printf(MSG_HELP_TRANSFER_PROJECT_RECORDS, CMD_TRANSFER_PROJECT_RECORDS_LONG);
			printf("\n");
			return 0;
		}

		// check min arg count
		else if (argc < 4)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
			return 0;
		}

		// parse
		old_id = strtol(argv[2], NULL, 10);
		new_id = strtol(argv[3], NULL, 10);

		// execute
		cmd_transfer_project_records(old_id, new_id);
	}
	else if((djb2_hash(CMD_SHOW_WEEK) == cmd_hash) ||
		(djb2_hash(CMD_SHOW_WEEK_LONG) == cmd_hash))
	{
		int8_t day = -1;
		int8_t month = -1;
		int16_t year = -1;

		// check max arg count
		if (argc > 5)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check min arg count
		else if (argc > 2 && argc < 5)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
			return 0;
		}

		// parse args
		if (argc != 2)
		{
#ifdef MAJOR_DATEFORMAT
			day = strtol(argv[2], NULL, 10);
			month = strtol(argv[3], NULL, 10);
			year = strtol(argv[4], NULL, 10);
#else
			year = strtol(argv[2], NULL, 10);
			month = strtol(argv[3], NULL, 10);
			day = strtol(argv[4], NULL, 10);
#endif
		}

		// exec
		cmd_show_records_week(year, month, day);
	}
	else if((djb2_hash(CMD_SHOW_MONTH) == cmd_hash) ||
		(djb2_hash(CMD_SHOW_MONTH_LONG) == cmd_hash))
	{
		int8_t month = -1;
		int16_t year = -1;

		// check max arg count
		if (argc > 4)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check min arg count
		else if (argc > 2 && argc < 4)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, CMD_HELP_LONG);
			return 0;
		}

		// parse args
		if (argc != 2)
		{
#ifdef MAJOR_DATEFORMAT
			month = strtol(argv[2], NULL, 10);
			year = strtol(argv[3], NULL, 10);
#else
			year = strtol(argv[2], NULL, 10);
			month = strtol(argv[3], NULL, 10);
#endif
		}

		// exec
		cmd_show_records_month(year, month);
	}
	else
	{
		// unknown command passed, print
		printf("Command not recognised.\nType '%s %s' for information on usage.\n", APP_NAME, CMD_HELP_LONG);
	}

	return 0;
}
