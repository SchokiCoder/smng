#/usr/bin/env bash

_smng_completions()
{
	COMPREPLY+=($(compgen -W "help about add-project show-projects edit-project archive-project delete-project record status stop add-record edit-record-project edit-record-begin edit-record-end edit-record-description delete-record transfer-project-records swap-project-records show-week show-month show-project-records merge-db show-config-path show-db-path" "${COMP_WORDS[1]}"))
}

complete -F _smng_completions smng