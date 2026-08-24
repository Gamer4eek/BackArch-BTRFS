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

use std::io::{BufRead, BufReader, Write, BufWriter};
use std::fs::{File, OpenOptions, metadata, self};
use std::path::Path;
use crate::parser;
use crate::constants::validation::{FSTAB_ERROR, CAN_SNAP};

impl parser::SnapshotInfo {
    pub fn parse_fstab(
        &self, 
        fstab_file: &str,
        name: &str,
        to_snap: &Vec<String>,
    ) -> Result<(), &'static str> {
        let file = File::open(fstab_file).map_err(|_| FSTAB_ERROR)?;

        let fstab_file = Path::new(fstab_file);
        let metadata = metadata(fstab_file).map_err(|_| FSTAB_ERROR)?;

        let tmp = fstab_file.with_extension("tmp");
        let tmp_w = File::create(&tmp).map_err(|_| FSTAB_ERROR)?;

        let mut reader = BufReader::new(file);
        let mut writer = BufWriter::new(tmp_w);

        for line in BufRead::lines(&mut reader) {
            let mut line = line.map_err(|_| FSTAB_ERROR)?;
            for snap in to_snap {
                if CAN_SNAP.contains(&snap.as_str()) {
                    line = line.replace(
                        &format!("subvol={}{}", snap), 
                        &format!("subvol=/@snaps/{}/{}", name, snap)
                    );
                }
            }
            writeln!(writer, "{}", line).map_err(|_| FSTAB_ERROR)?;
        }
        writer.flush().map_err(|_| FSTAB_ERROR)?;
        drop(writer);
        std::fs::rename(&tmp, fstab_file).map_err(|_| FSTAB_ERROR)?;

        Ok(())
    }
}
