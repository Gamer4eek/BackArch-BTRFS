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

use crate::helper;

use crate::constants::args::ARGS;
use crate::constants::config::{
    OPTS, COMMENTS, CONF_WARNING,
    EASTER_EGG, EASTER_EGG_MSG,
};
use crate::constants::validation::{
    CAN_SNAP, FORBIDDEN_SYMBOLS, BRACKETS,

    EMPTY_ERROR, DQUOTES_ERROR, LIST_ERROR,
    KEY_ERROR, CANT_SNAP_ERROR, PATH_ERROR,
    SYMBOL_ERROR,
};

use std::process::exit;
use std::env::args;
use std::fs::{File, metadata};
use std::io::{BufReader, BufRead};

#[derive(Debug)]
pub struct SnapshotInfo {
    pub name:        Option<String>,
    pub config_file: Option<String>,
    pub drive_uuid:  Option<String>,
    pub grub_file:   Option<String>,
    pub hooks_dir:   Option<String>,
    pub dir:         Option<String>,
    pub ro_dir:      Option<String>,
    pub fsroot_path: Option<String>,
    pub usr_path:    Option<String>,
    pub var_path:    Option<String>,
    pub boot_path:   Option<String>,
    pub home_path:   Option<String>,
    pub root_path:   Option<String>,
    pub mnt_path:    Option<String>,
    pub log_file:    Option<String>,
    pub to_snap:     Option<Vec<String>>,
}

impl SnapshotInfo {
    pub fn new() -> Self {
        Self {
            name:        None,
            config_file: None,
            drive_uuid:  None,
            grub_file:   None,
            hooks_dir:   None,
            dir:         None,
            ro_dir:      None,
            fsroot_path: None,
            usr_path:    None,
            var_path:    None,
            boot_path:   None,
            home_path:   None,
            root_path:   None,
            mnt_path:    None,
            log_file:    None,
            to_snap:     None,
        }
    }
    pub fn parse_args(&mut self) -> Result<(), &'static str> {
        let mut a: Vec<String> = Vec::with_capacity(16);

        a.extend(args().skip(1)); 
        if a.iter().any(|arg| arg == "--help") {
            helper::help()?; exit(0);
        }
        if a.iter().any(|arg| arg == "--tutorial") {
            helper::tutorial()?; exit(0);
        }
        
        collect(&a, self, false)?;
        Ok(())
    }
    pub fn parse_config(&mut self) -> Result<(), &'static str> {
        let mut opts: Vec<String> = Vec::with_capacity(15);

        let Some(conf) = &self.config_file else { 
            eprintln!("{}", CONF_WARNING); return Ok(()); 
        };
        let file = match File::open(conf) {
            Ok(file) => file,
            _ => { eprintln!("{}", CONF_WARNING); return Ok(()); }
        };
        let mut reader = BufReader::new(file);
        for line in BufRead::lines(&mut reader) {
            let line = match line {
                Ok(line) => {
                    if line.trim().is_empty() { continue; } else { line }
                }
                Err(_) => { eprintln!("{}", CONF_WARNING); break; }
            };
            comments!(line, opts);
            if opts.iter().any(|s| EASTER_EGG.contains(s.trim())) {
                println!("{}", EASTER_EGG_MSG); exit(0);
            }
        }
        collect(&opts, self, true)?;
        Ok(())
    }
    pub fn set_defaults(&mut self) -> Result<(), &'static str> {
        insert!(
            self,
            name        = "ArchLinux",
            config_file = "/etc/backarch/backarch.conf",
            grub_file   = format!("40_backarch_{}", { if let Some(name) = &self.name { name } else { "ArchLinux" } }),
            hooks_dir   = "/etc/backarch/hooks",
            dir         = "/.snapshots",
            fsroot_path = "/",
            usr_path    = "/usr",
            var_path    = "/var",
            boot_path   = "/boot",
            home_path   = "/home",
            root_path   = "/root",
            mnt_path    = "/mnt",
        )?;
        insert!(vec: self,
            to_snap = vec![
                "fsroot".to_string(), "usr".to_string(), "var".to_string(), 
                "boot".to_string(), "home".to_string(), "root".to_string(), "mnt".to_string()
            ],
        )
    }
}
fn collect(objs: &Vec<String>, snap_info: &mut SnapshotInfo, conf: bool) -> Result<(), &'static str> {
    for obj in objs {
        let (key, mut value) = match obj.split_once('=') {
            Some((key,value)) => (key.trim(),value.trim()),
            None => {
                return Err(error!(obj, { helper::help()? } => KEY_ERROR)?);
            }
        };
        if !ARGS.iter().any(|&s| key == s) {
            if !OPTS.iter().any(|&s| key == s) {
                error!(obj, { helper::help()? } => KEY_ERROR)?;
            }
        }
        if value == "_" { continue; }
        if conf {
            if key == ARGS[15] || key == OPTS[14] {
                if !(
                    (value.starts_with('<') || value.ends_with('>')) ||
                    (value.starts_with('[') || value.ends_with(']')) ||
                    (value.starts_with('{') || value.ends_with('}')) ||
                    (value.starts_with('(') || value.ends_with(')')) ||
                    (value.starts_with('|') || value.ends_with('|'))
                ) {
                    dquotes!(value => error!(obj => DQUOTES_ERROR))?;
                    value = value.trim_matches('"');
                }
            } else {
                dquotes!(value => error!(obj => DQUOTES_ERROR))?;
                value = value.trim_matches('"');
            }
        }
        is_empty!(value => error!(obj => EMPTY_ERROR))?;
        if key == ARGS[15] || key == OPTS[14] {
            if !(
                (value.starts_with('<') || value.ends_with('>')) ||
                (value.starts_with('[') || value.ends_with(']')) ||
                (value.starts_with('{') || value.ends_with('}')) ||
                (value.starts_with('(') || value.ends_with(')')) ||
                (value.starts_with('|') || value.ends_with('|'))
            ) {
                forbidden!(FORBIDDEN_SYMBOLS, value =>
                    error!(
                        obj,
                        { helper::help()?; }
                        => SYMBOL_ERROR
                    )
                )?;
            }
        } else {
            forbidden!(FORBIDDEN_SYMBOLS, value =>
                error!(
                    obj,
                    { helper::help()?; }
                    => SYMBOL_ERROR
                )
            )?;
        }
        match key {
            v if v == ARGS[0]  => { insert!(snap_info, name = value) }
            v if v == ARGS[1]  => { 
                path!(value, is_file => error!(obj => PATH_ERROR))?;
                insert!(snap_info, config_file = value)
            }
            v if v == ARGS[2]  => { insert!(snap_info, drive_uuid = value) }
            v if v == ARGS[3]  => { 
                path!(filedir: value => error!(obj => PATH_ERROR))?;
                insert!(snap_info, grub_file = value)
            }
            v if v == ARGS[4]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, hooks_dir = value) 
            }
            v if v == ARGS[5]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, dir = value) 
            }
            v if v == ARGS[6]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, ro_dir = value) 
            }
            v if v == ARGS[7]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, fsroot_path = value) 
            }
            v if v == ARGS[8]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, usr_path = value) 
            }
            v if v == ARGS[9]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, var_path = value) 
            }
            v if v == ARGS[10]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, boot_path = value) 
            }
            v if v == ARGS[11]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, home_path = value) 
            }
            v if v == ARGS[12]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, root_path = value) 
            }
            v if v == ARGS[13]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, mnt_path = value) 
            }
            v if v == ARGS[14]  => { 
                path!(filedir: value => error!(obj => PATH_ERROR))?;
                insert!(snap_info, log_file = value)
            }
            v if v == ARGS[15]  => {
                let mut values: Vec<String> = Vec::with_capacity(6);
                list!(value; values => obj; error!(obj, { helper::help()?; } => LIST_ERROR))?;
                insert!(vec: snap_info, to_snap = values)
            }

            v if v == OPTS[0]  => { insert!(snap_info, name = value) }
            v if v == OPTS[1]  => { insert!(snap_info, drive_uuid = value) }
            v if v == OPTS[2]  => { 
                path!(filedir: value => error!(obj => PATH_ERROR))?;
                insert!(snap_info, grub_file = value)
            }
            v if v == OPTS[3]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, hooks_dir = value) 
            }
            v if v == OPTS[4]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, dir = value) 
            }
            v if v == OPTS[5]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, ro_dir = value) 
            }
            v if v == OPTS[6]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, fsroot_path = value) 
            }
            v if v == OPTS[7]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, usr_path = value) 
            }
            v if v == OPTS[8]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, var_path = value) 
            }
            v if v == OPTS[9]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, boot_path = value) 
            }
            v if v == OPTS[10]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, home_path = value) 
            }
            v if v == OPTS[11]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, root_path = value) 
            }
            v if v == OPTS[12]  => { 
                path!(value, is_dir => error!(obj => PATH_ERROR))?;
                insert!(snap_info, mnt_path = value) 
            }
            v if v == OPTS[13]  => { 
                path!(filedir: value => error!(obj => PATH_ERROR))?;
                insert!(snap_info, log_file = value)
            }
            v if v == OPTS[14]  => {
                let mut values: Vec<String> = Vec::with_capacity(6);
                list!(value; values => obj; error!(obj, { helper::help()?; } => LIST_ERROR))?;
                insert!(vec: snap_info, to_snap = values)
            }

            _ => { error!(obj, { helper::help()?; } => KEY_ERROR) }
        }?;
    }
    insert!(snap_info, config_file = "/etc/backarch/backarch.conf")?;
    Ok(())
}
