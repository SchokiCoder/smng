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

const GNU_WEBSITE: &str = "https://www.gnu.org/licenses/old-licenses/gpl-2.0.html";

i18n_codegen::i18n!("../lang");

pub fn cur_locale() -> Locale {
	// get user language
	let lcl: Locale;
	let lang = std::env::var("LANG");
	
	// if lang env var could be read		
	if lang.is_ok() == false {
		return Locale::En;
	}
	
	let lang = lang.unwrap();
	let lang = lang.split('.').next();
	
	if lang.is_some() == false {
		return Locale::En;
	}
	
	let lang = lang.unwrap();
		
	// get strings	
	match lang {
		"de_DE" => {
			lcl = Locale::De;
		}
		
		_ => {
			lcl = Locale::En;
		}
	}
	
	return lcl;
}

