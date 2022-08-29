APP_NAME = smng
INSTALL_BIN_DIR = /usr/bin

debug:
	cargo rustc

release:
	cargo rustc --release

mkconfig:
	mkdir -p /etc/smng.d
	echo "/home/"${USER}"/.smng/worktimes.db" >> "/etc/smng.d/db_path"

install-manpage:
	mkdir -p /usr/local/man/man1
	cp manpage.1 /usr/local/man/man1/${APP_NAME}.1
	gzip /usr/local/man/man1/${APP_NAME}.1
	mandb

install-completion:
	cp completion.bash /usr/share/bash-completion/completions/${APP_NAME}

install-utils: mkconfig install-manpage install-completion

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
