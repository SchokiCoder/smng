# SchokiManager

Personal working time manager, to monitor the time you worked on each project.

## Dependencies

+ [sqlite](https://github.com/stainless-steel/sqlite)
+ [chrono](https://github.com/chronotope/chrono)
+ [term_size](https://github.com/clap-rs/term_size-rs)

## Installation

```
git clone https://github.com/SchokiCoder/smng.git
cd smng
make release
sudo make install
```

## Example

```
smng ap "MyNewProject"
smng r 1

* later *

smng s "created git repo, added main.c"
```

## License

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

[GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html)
