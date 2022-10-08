/*
	"SchokiManager" in short "smng"
	Copyright (C) 2021 - 2022	Andy Frank Schoknecht

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

const GNU_WEBSITE: &str = "https://www.gnu.org/licenses/";

i18n_codegen::i18n!("lang");

pub fn get_locale() -> Locale {
	// get user language
	let lcl: Locale;
	let lang = env!("LANG").split('.').next();
	
	// if lang env var could be read, get strings
	if lang.is_some() {
		match lang.unwrap() {
			"de_DE" => {
				lcl = Locale::De;
			}
			
			_ => {
				lcl = Locale::En;
			}
		}
	}
	else {
		lcl = Locale::En;
	}
	
	return lcl;
}
