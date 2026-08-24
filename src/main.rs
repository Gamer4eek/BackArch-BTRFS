//    BackArch is a CLI tool for managing BTRFS snapshots
//
//    Copyright (C) 2026  Gamer4eek <gamer4eek1@gmail.com>
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[macro_use]
mod macros;

mod constants;
mod helper;
mod parser;
mod syscalls;
mod file_parser;


fn main() -> Result<(), &'static str> {
    let mut snap_info = parser::SnapshotInfo::new();

    snap_info.parse_args()?;
    snap_info.parse_config()?;
    snap_info.set_defaults()?;

    println!("{:#?}", snap_info);

    let Some(name) = &snap_info.name else {
        return Err("No name");
    };
    let Some(dir) = &snap_info.dir else {
        return Err("No dir");
    };
    let Some(fsroot) = &snap_info.fsroot_path else {
        return Err("No fsroot path");
    };
    let Some(fstab) = &snap_info.fstab_path else {
        return Err("no fstab");
    };
    let Some(to_snap) = &snap_info.to_snap else {
        return Err("nothing to snap");
    };

    snap_info.parse_fstab(fstab, name, to_snap)?;
//    syscalls::make_syscall(&name, &dir, &fsroot)?;
    Ok(())
}
