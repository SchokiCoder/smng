/*
 * SchokiManager
 * Copyright (C) 2021 - 2022  Andy Frank Schoknecht
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not see
 * <https://www.gnu.org/licenses/old-licenses/gpl-2.0.html>.
 */

use smng_lib::{lang, cmdinfo};

fn main() {
	let lcl = lang::cur_locale();
	let cmd_name: &str = "smng-record";

	// gen manpage
	let cmd = cmdinfo::record(
		&lcl,
		cmd_name, env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
		
	let man = clap_mangen::Man::new(cmd);
			
	let mut buffer: Vec<u8> = Default::default();
	man.render(&mut buffer).unwrap();

	// write manpage to file
	use std::io::Write;
	let mut f = std::fs::File::create(format!("{}.1", cmd_name)).unwrap();
	f.write(&buffer).unwrap();
	
	// gen completion
	let cmd = cmdinfo::record(
		&lcl,
		cmd_name, env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
		
	let outdir = match std::env::var_os("OUT_DIR") {
		None => return Ok(()),
		Some(outdir) => outdir
	};

	let path = clap_complete::generate_to(
		clap_complete::Shells::Bash,
		&mut cmd,
		cmd_name,
		outdir).unwrap();
	
	println!("cargo:warning=completion file is generated: {:?}", path);
}
   
