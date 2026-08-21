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

pub mod args {
    pub const ARGS: [&str; 16] = [
        "--name", "--config", "--drive-uuid", "--grub-file", "--hooks",
        "--dir", "--ro-dir", "--fsroot", "--usr", "--var", "--boot", "--home",
        "--root", "--mnt", "--log", "--to-snap",
    ];
    pub const FORBIDDEN_SYMBOLS: [&str; 22] = [
        "'","\"","//",":",";","..","@","$","#","`","\\",
        "*","[","]","{", "}","?","<",">",",","(",")",
    ];
    pub const EMPTY_ERROR:  &str = "Empty value";
    pub const PATH_ERROR:   &str = "Invalid path";
    pub const KEY_ERROR:    &str = "Invalid argument/option";
    pub const SYMBOL_ERROR: &str = "Value contains forbidden symbol(s)";
}

pub mod opts {
    pub const OPTS: [&str; 15] = [
        "name", "drive_uuid", "grub_file", "hooks", "dir", "ro_dir",
        "fsroot", "usr", "var", "boot", "home", "root", "mnt", "log", 
        "to_snap",
    ];
    pub const FORBIDDEN_SYMBOLS: [&str; 22] = [
        "'","\"","//",":",";","..","@","$","#","`","\\",
        "*","[","]","{", "}","?","<",">",",","(",")",
    ];
    pub const LIST_BRACKETS: [&str; 3] = ["<>", "[]", "{}"];
    pub const CAN_SNAP:      [&str; 7] = ["fsroot", "usr", "var", "boot", "home", "root", "mnt"];

    pub const COMMENTS: (char, &str, &str, &str, &str, &str) = ('#', ";;", "==>", "-->", "//", "/*");

    pub const PATH_ERROR:      &str = "Invalid path";
    pub const KEY_ERROR:       &str = "Invalid argument/option";
    pub const EMPTY_ERROR:     &str = "Empty option value";
    pub const DQUOTES_ERROR:   &str = "Value must be put in double quotes";
    pub const SYMBOL_ERROR:    &str = "Value contains forbidden symbol(s)";
    pub const LIST_ERROR:      &str = "Unclosed list brackets";
    pub const CANT_SNAP_ERROR: &str = "Can't snapshot item(s)";
    pub const CONF_WARNING:    &str = "Warning: could not find or open the configuration file";

    pub const EASTER_EGG:     &str = "btw";
    pub const EASTER_EGG_MSG: &str = "I use Arch, btw";
}

pub mod btrfs {
    pub const BTRFS_MAGIC:       u64 = 0x94;
    pub const SNAP_CREATE_V2_NR: u64 = 23;
}

pub mod ioctl {
    pub const IOCTL: u64 = 16;
    
    pub const IOC_DIRSHIFT:  u64 = 30;
    pub const IOC_WRITE:     u64 = 1;
    pub const IOC_TYPESHIFT: u64 = 8;
    pub const IOC_NRSHIFT:   u64 = 0;
    pub const IOC_SIZESHIFT: u64 = 16;
}
