#/usr/bin/env bash

#	SchokiManager
#	Copyright (C) 2021	Andy Frank Schoknecht
#
#	This program is free software: you can redistribute it and/or modify
#	it under the terms of the GNU General Public License as published by
#	the Free Software Foundation, either version 3 of the License, or
#	(at your option) any later version.
#
#	This program is distributed in the hope that it will be useful,
#	but WITHOUT ANY WARRANTY; without even the implied warranty of
#	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#	GNU General Public License for more details.
#
#	You should have received a copy of the GNU General Public License
#	along with this program.  If not, see <https://www.gnu.org/licenses/>.

_smng_completions()
{
	COMPREPLY+=($(compgen -W "help about add-project show-projects edit-project delete-project record status stop add-record edit-record-project edit-record-begin edit-record-end edit-record-description delete-record transfer-project-records show-week show-month merge-db" "${COMP_WORDS[1]}"))
}

complete -F _smng_completions smng