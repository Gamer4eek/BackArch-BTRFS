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

use crate::constants::btrfs::{BTRFS_MAGIC, SNAP_CREATE_V2_NR,};
use crate::constants::ioctl::{
    IOCTL, IOC_WRITE, 
    IOC_TYPESHIFT, IOC_DIRSHIFT,
    IOC_NRSHIFT, IOC_SIZESHIFT,
};

use std::os::fd::AsRawFd;
use std::mem::size_of as sizeof;

#[repr(C)]
struct vol_args {
    fd:      i64,
    transid: u64,
    flags:   u64,
    qgroups: [u64; 4],
//    name:    &'a Vec<u8>,
    name:    [u8; 4040],
}

const SNAP_CREATE_V2: u64 = ioc!(
    IOC_WRITE, BTRFS_MAGIC, 
    SNAP_CREATE_V2_NR, sizeof::<vol_args>()
);

pub fn make_syscall(
    snap_name: String,
    snap_dir:  &str,
    fsroot:    &str,
) -> Result<(), &'static str> {

    let file = match std::fs::File::open(fsroot) {
        Ok(file) => Ok(file),
        Err(_)   => Err("File error")
    }?;
    let dir = match std::fs::File::open(snap_dir) {
        Ok(dir) => Ok(dir),
        Err(_)  => Err("Dir error")
    }?;

    let filefd = file.as_raw_fd();
    let dirfd = dir.as_raw_fd();

    let mut name = [0u8; 4040];
    let mut bytes = snap_name.as_bytes().to_vec();
    bytes.push(0);
    name[..bytes.len()].copy_from_slice(&bytes);

    let mut args = vol_args {
        fd: filefd as i64,
        flags: 0,
        transid: 0,
        qgroups: [0u64; 4],
        name: name,
    };

    unsafe {
        let result: i64;
        syscall!(
            nostack, preserves_flags, readonly;

            IOCTL, dirfd, 
            SNAP_CREATE_V2, &mut args

            => result
        );

        if result == -1 {
            println!("Провал! {} {} {}", args.fd, args.flags, args.transid );
            println!("returned: {}", result);
            return Ok(());
        } else {
            println!("Ура, победа! {} {} {}", args.fd, args.flags, args.transid );
            println!("returned: {}", result);
            return Ok(());
        }
    }
}
