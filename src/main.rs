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

pub mod app;
pub mod commands;
//use std::env;

fn main() {
	//let args: Vec<String> = env::args().collect();
	
	commands::show_month(2022, 02);
	//commands::show_week();
}


/*
	u32_t cmd_hash;
	bool_t edit_begin = FALSE;
	bool_t force = FALSE;
	sl32_t record_id;
	sl32_t project_id;
	sl32_t dest_project_id;
	sl32_t src_project_id;
	ul16_t year = -1;
	ul8_t month = -1;
	ul8_t day = -1;
	ul8_t hour = -1;
	ul8_t minute = -1;

	// if no command given, print help and end
	if (argc < 2)
	{
		printf(
			"Usage: %s COMMAND [OPTIONS]\nTry '%s %s' for more information.\n",
			APP_NAME, APP_NAME, DATA_COMMANDS[CMD_HELP].name);
		return 0;
	}

	// hash given command
	cmd_hash = SM_djb2_encode(argv[1]);

	// execute command
	switch (cmd_hash)
	{
	case DJB2_HELP:
	case DJB2_HELP_ABBR:
		// check max arg count
		if (argc > 2)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// exec
		cmd_help();
		break;

	case DJB2_ADD_PROJECT:
	case DJB2_ADD_PROJECT_ABBR:
		// check max arg count
		if (argc > 3)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			print_cmd_help(CMD_ADD_PROJECT);
			return 0;
		}

		// exec
		cmd_add_project(argv[2]);
		break;

	case DJB2_SHOW_PROJECTS:
	case DJB2_SHOW_PROJECTS_ABBR:
		// check max arg count
		if (argc > 2)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// exec
		cmd_show_projects();
		break;

	case DJB2_EDIT_PROJECT:
	case DJB2_EDIT_PROJECT_ABBR:
		// check max arg count
		if (argc > 4)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			print_cmd_help(CMD_EDIT_PROJECT);
			return 0;
		}

		// check min arg count
		else if (argc < 4)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, DATA_COMMANDS[CMD_HELP].name);
			return 0;
		}

		// parse args
		project_id = strtol(argv[2], NULL, 10);

		// exec
		cmd_edit_project(project_id, argv[3]);
		break;

	case DJB2_DELETE_PROJECT:
		// check max arg count
		if (argc > 4)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			print_cmd_help(CMD_DELETE_PROJECT);
			return 0;
		}

		// parse args
		project_id = strtol(argv[2], NULL, 10);

		if (argc > 3)
			force = strtoul(argv[3], NULL, 10);

		// exec
		cmd_delete_project(project_id, force);
		break;

	case DJB2_RECORD:
	case DJB2_RECORD_ABBR:
		// check max arg count
		if (argc > 3)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			print_cmd_help(CMD_RECORD);
			return 0;
		}

		// parse args
		project_id = strtol(argv[2], NULL, 10);

		// exec
		cmd_record(project_id);
		break;

	case DJB2_STATUS:
		// check max arg count
		if (argc > 2)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// exec
		cmd_status();
		break;

	case DJB2_STOP:
	case DJB2_STOP_ABBR:
		// check max arg count
		if (argc > 3)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			print_cmd_help(CMD_STOP);
			return 0;
		}

		// exec
		cmd_stop(argv[2]);
		break;

	case DJB2_EDIT_RECORD_PROJECT:
	case DJB2_EDIT_RECORD_PROJECT_ABBR:
		// check max arg count
		if (argc > 4)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			print_cmd_help(CMD_EDIT_RECORD_PROJECT);
			return 0;
		}

		// check min arg count
		else if (argc < 4)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, DATA_COMMANDS[CMD_HELP].name);
			return 0;
		}

		// parse args
		record_id = strtol(argv[2], NULL, 10);
		project_id = strtol(argv[3], NULL, 10);

		// exec
		cmd_edit_record_project(record_id, project_id);
		break;

	case DJB2_EDIT_RECORD_BEGIN:
	case DJB2_EDIT_RECORD_BEGIN_ABBR:
		edit_begin = TRUE;
		goto EDIT_RECORD_CASE;

	case DJB2_EDIT_RECORD_END:
	case DJB2_EDIT_RECORD_END_ABBR:
		EDIT_RECORD_CASE:
		// check max arg count
		if (argc > 8)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			if (edit_begin == TRUE)
				print_cmd_help(CMD_EDIT_RECORD_BEGIN);
			else
				print_cmd_help(CMD_EDIT_RECORD_END);

			return 0;
		}

		// check min arg count
		else if (argc < 8)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, DATA_COMMANDS[CMD_HELP].name);
			return 0;
		}

		// parse args
		record_id = strtoul(argv[2], NULL, 10);

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
		cmd_edit_record_time(edit_begin, record_id, year, month, day, hour, minute);
		break;

    case DJB2_EDIT_RECORD_DESCRIPTION:
    case DJB2_EDIT_RECORD_DESCRIPTION_ABBR:
		// check max arg count
		if (argc > 4)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			print_cmd_help(CMD_EDIT_RECORD_DESCRIPTION);
			return 0;
		}

		// check min arg count
		else if (argc < 4)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, DATA_COMMANDS[CMD_HELP].name);
			return 0;
		}

		// parse args
		record_id = strtol(argv[2], NULL, 10);

		// exec
		cmd_edit_record_description(record_id, argv[3]);
		break;

	case DJB2_DELETE_RECORD:
		// check max arg count
		if (argc > 3)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			print_cmd_help(CMD_DELETE_RECORD);
			return 0;
		}

		// parse args
		record_id = strtol(argv[2], NULL, 10);

		// exec
		cmd_delete_record(record_id);
		break;

	case DJB2_TRANSFER_PROJECT_RECORDS:
		// check max arg count
		if (argc > 4)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check if no args
		else if (argc == 2)
		{
			// print help, stop
			print_cmd_help(CMD_TRANSFER_PROJECT_RECORDS);
			return 0;
		}

		// check min arg count
		else if (argc < 4)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, DATA_COMMANDS[CMD_HELP].name);
			return 0;
		}

		// parse
		src_project_id = strtol(argv[2], NULL, 10);
		dest_project_id = strtol(argv[3], NULL, 10);

		// execute
		cmd_transfer_project_records(src_project_id, dest_project_id);
		break;

	case DJB2_SHOW_WEEK:
	case DJB2_SHOW_WEEK_ABBR:
		// check max arg count
		if (argc > 5)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check min arg count
		else if (argc > 2 && argc < 5)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, DATA_COMMANDS[CMD_HELP].name);
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
		break;

	case DJB2_SHOW_MONTH:
	case DJB2_SHOW_MONTH_ABBR:
		// check max arg count
		if (argc > 4)
		{
			printf("WARNING: Too many arguments were passed.\nAdditional arguments will be ignored.\n");
		}

		// check min arg count
		else if (argc > 2 && argc < 4)
		{
			printf("ERROR: Not enough arguments were passed.\nType %s %s for help.\n", APP_NAME, DATA_COMMANDS[CMD_HELP].name);
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
		break;

	default:
		// unknown command passed, print
		printf("Command not recognised.\nType '%s %s' for information on usage.\n", APP_NAME, DATA_COMMANDS[CMD_HELP].name);
		break;
	}

	return 0;
}
*/
