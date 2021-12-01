#    SchokiManager
#    Copyright (C) 2021  Andy Frank Schoknecht
#
#    This program is free software: you can redistribute it and/or modify
#    it under the terms of the GNU General Public License as published by
#    the Free Software Foundation, either version 3 of the License, or
#    (at your option) any later version.
#
#    This program is distributed in the hope that it will be useful,
#    but WITHOUT ANY WARRANTY; without even the implied warranty of
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#    GNU General Public License for more details.
#
#    You should have received a copy of the GNU General Public License
#    along with this program.  If not, see <https://www.gnu.org/licenses/>.

CC = cc
APP_NAME = smng
CFLAGS = -std=c99 -Wall -pedantic -O3
LIBS = -lsqlite3

options:
	@echo smng build options:
	@echo "CFLAGS   = ${CFLAGS}"
	@echo "CC       = ${CC}"

clean:
	rm -f ${APP_NAME} *.o

install:
	${CC} *.c ${CFLAGS} ${LIBS} -o ${APP_NAME}
	mkdir -p ${DESTDIR}${PREFIX}/bin
	mv -f ${APP_NAME} ${DESTDIR}${PREFIX}/bin
	chmod 755 ${DESTDIR}${PREFIX}/bin/${APP_NAME}

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/${APP_NAME}
