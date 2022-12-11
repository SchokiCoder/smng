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
make prepare        # DO NOT USE SUDO HERE
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

SchokiManager
Copyright (C) 2021 - 2022  Andy Frank Schoknecht

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program; if not see
[GPLv2](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html)

