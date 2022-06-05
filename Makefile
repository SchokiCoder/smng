#	SchokiManager
#	Copyright (C) 2021  Andy Frank Schoknecht
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

APP_NAME = smng
INSTALL_BIN_DIR = /usr/bin
USERNAME = ""

info:
	@echo "1. Set username in Makefile.\n2. make install"

clean:
	rm -f ${APP_NAME} *.o

build:
	cargo rustc --release

mkconfig:
	mkdir -p /etc/smng.d
	echo "/home/"${USERNAME}"/.smng/worktimes.db" >> "/etc/smng.d/db_path"

install: build mkconfig
	mkdir -p ${INSTALL_BIN_DIR}
	mv -f ./target/release/${APP_NAME} ${INSTALL_BIN_DIR}
	chmod 755 ${INSTALL_BIN_DIR}/${APP_NAME}

uninstall:
	rm -f ${INSTALL_BIN_DIR}/${APP_NAME}
