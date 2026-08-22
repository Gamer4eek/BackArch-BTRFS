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

use crate::constants::help::LIST_BRACKETS;
use crate::constants::validation::{
    FORBIDDEN_SYMBOLS, CAN_SNAP
};
use std::io::{stdin,}; //stdout};

pub fn tutorial() -> Result<(), &'static str> {
    loop {
        println!("===> BackArch TUTORIAL <===");
        println!("    1.  ");


        let mut input = String::new();
        stdin().read_line(&mut input).map_err(|_| "Input error")?;

    }
    Ok(())
}

pub fn help() -> Result<(), &'static str> {
    println!(" ");
    println!("Default configuration file is located at '/etc/backarch/backarch.conf'");
    println!(" ");
    println!("Usage(must have root access): backarch [--some-option='value']");
    println!("Configuration file(same options' names): [some_option = \"value\"]");
    println!(" ");
    println!("    --help:       Display this help message");
    println!(" ");
    println!("    --name:       Set name of the snapshot");
    println!(" ");
    println!("    --config:     Configuration file to use");
    println!(" ");
    println!("    --drive-uuid: UUID of the system drive");
    println!("    --grub-file:  Set the name of GRUB menuentry");
    println!(" ");
    println!("    --hooks:      Set hooks' directory");
    println!("    --dir:        Set the snapshot's directory");
    println!("    --ro-dir:     Set the readonly snaphot's directory");
    println!(" ");
    println!("    --fsroot:     Set path to the filesystem root, i.e., /");
    println!("    --usr:        Set path to the /usr directory");
    println!("    --var:        Set path to the /var directory");
    println!("    --boot:       Set path to the /boot directory");
    println!("    --home:       Set path to the /home directory");
    println!("    --root:       Set path to the /root directory");
    println!(" ");
    println!("    --log:        Set a file to log into");
    println!(" ");
    println!("List of forbidden symbols: {}", FORBIDDEN_SYMBOLS.join(", "));
    println!("List of list closing brackets: {}", LIST_BRACKETS.join(", "));
    println!("List of items that can be snapshotted: {}", CAN_SNAP.join(", "));
    println!(" ");

    Ok(())
}

