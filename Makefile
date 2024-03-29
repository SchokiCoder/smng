APP_NAME = smng
INSTALL_BIN_DIR = /usr/bin

debug:
	cargo build

release:
	cargo build --release

mkconfig:
	echo "/home/"${USER}"/.smng/worktimes.db" >> "db_path"

prepare: release mkconfig

install-config:
	mkdir -p /etc/smng.d
	cp "db_path" "/etc/${APP_NAME}.d/db_path"

install-manpage:
	mkdir -p /usr/local/man/man1
	cp manpage.1 /usr/local/man/man1/${APP_NAME}.1
	gzip /usr/local/man/man1/${APP_NAME}.1
	mandb

install-completion:
	cp completion.bash /usr/share/bash-completion/completions/${APP_NAME}

install-utils: install-config install-manpage install-completion

install: install-utils
	mkdir -p ${INSTALL_BIN_DIR}
	mv -f ./target/release/${APP_NAME} ${INSTALL_BIN_DIR}
	chmod 755 ${INSTALL_BIN_DIR}/${APP_NAME}

uninstall:
	rm -f ${INSTALL_BIN_DIR}/${APP_NAME}
	
	rm -r -f /etc/smng.d
	
	rm -f /usr/share/bash-completion/completions/${APP_NAME}
	
	rm -f /usr/local/man/man1/${APP_NAME}.1
	rm -f /usr/local/man/man1/${APP_NAME}.1.gz
	mandb
