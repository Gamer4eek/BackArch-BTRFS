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

macro_rules! error {
    (
        $obj:expr $(, $custom:block)? => $err:expr
    ) =>
    {{
        eprintln!("Error: {}", $obj); 
        $($custom;)?
        Err($err)
    }};
}
macro_rules! forbidden {
    (
        $forbidden:expr, $value:expr => $result:expr
    ) => 
    {
        if !$forbidden.iter().any(|&s| $value.contains(s)) {
            Ok(())
        } else { $result }
    }
}
macro_rules! is_empty {
    (
        $value:expr => $result:expr
    ) =>
    {{
        if !$value.trim_matches('"').is_empty() {
            Ok(())
        } else { $result }
    }};
}
macro_rules! insert {
    (
        $struct:ident, $($field:ident = $value:expr),* $(,)?
    ) =>
    {{
        $(
            if $struct.$field == None {
                $struct.$field = Some($value.to_string());
            }
        )*
        Ok(())
    }};
    (vec:
        $struct:ident, $($field:ident = $value:expr),* $(,)?
    ) =>
    {{
        $(
            if $struct.$field == None {
                $struct.$field = Some($value);
            }
        )*
        Ok(())
    }};
}
macro_rules! dquotes {
    (
        $value:ident => $result:expr
    ) =>
    {
        if $value.starts_with('"') && $value.ends_with('"') {
            Ok(())
        } else { $result }
    }
}
macro_rules! list {
    (
        $value:expr; $values:ident => $obj:expr; $result:expr
    ) =>
    {{
        if $value.starts_with('<') && $value.ends_with('>') {
            $values = $value.trim_matches(|c| c == '<' || c == '>')
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            for value in $values.iter_mut() {
                is_empty!(value => error!($obj => EMPTY_ERROR))?;
                dquotes!(value => error!($obj => DQUOTES_ERROR))?;
                *value = value.trim_matches('"').to_string();
                is_empty!(value => error!($obj => EMPTY_ERROR))?;
                if !CAN_SNAP.iter().any(|&s| value == s) {
                    error!($obj, { crate::helper::help()?; } => CANT_SNAP_ERROR)?;
                }
            }
            Ok(())
        } else if $value.starts_with('[') && $value.ends_with(']') { 
            $values = $value.trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            for value in $values.iter_mut() {
                is_empty!(value => error!($obj => EMPTY_ERROR))?;
                dquotes!(value => error!($obj => DQUOTES_ERROR))?;
                *value = value.trim_matches('"').to_string();
                is_empty!(value => error!($obj => EMPTY_ERROR))?;
                if !CAN_SNAP.iter().any(|&s| value == s) {
                    error!($obj, { crate::helper::help()?; } => CANT_SNAP_ERROR)?;
                }
            }
            Ok(())
        } else if $value.starts_with('{') && $value.ends_with('}') {
            $values = $value.trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            for value in $values.iter_mut() {
                is_empty!(value => error!($obj => EMPTY_ERROR))?;
                dquotes!(value => error!($obj => DQUOTES_ERROR))?;
                *value = value.trim_matches('"').to_string();
                is_empty!(value => error!($obj => EMPTY_ERROR))?;
                if !CAN_SNAP.iter().any(|&s| value == s) {
                    error!($obj, { crate::helper::help()?; } => CANT_SNAP_ERROR)?;
                }
            }
            Ok(())
        } else { $result }
    }};
}
macro_rules! path {
    (
        $value:expr, $is:ident => $result:expr
    ) =>
    {
        if let Ok(metadata) = std::fs::metadata($value) {
            if metadata.$is() {
                Ok(())
            } else { $result }
        } else { $result }
    };

    (filedir:
        $value:expr => $result:expr
    ) =>
    {
        match $value.rsplit_once('/') {
            Some((dir, _)) => {
                path!(dir, is_dir => $result)
            }
            None => { $result }
        }
    }
}
macro_rules! syscall {
    (
        $($opts:ident),*;
        $rax:expr, $rdi:expr, 
        $rsi:expr, $rdx:expr $(=> $result:expr)?
    ) =>
    {
        core::arch::asm!(
            "syscall",
            in("rax") $rax,
            in("rdi") $rdi,
            in("rsi") $rsi,
            in("rdx") $rdx,
            $(lateout("rax") $result,)?
            options($($opts),*)
        );
    }
}
macro_rules! ioc {
    (
        $dir:expr, $type:expr, $nr:expr, $size:expr
    ) =>
    {
        {
            (($dir as u64)  << IOC_DIRSHIFT)  |
            (($type as u64) << IOC_TYPESHIFT) |
            (($nr as u64)   << IOC_NRSHIFT)   |
            (($size as u64) << IOC_SIZESHIFT)
        }
    }
}
macro_rules! comments {
    (
        $line:ident, $opts:ident
    ) =>
    {
        let mut opt = $line.clone();
        let chars: Vec<char> = $line.chars().collect();
        let len = chars.len();
        for i in 0..len {
            if chars[i] == COMMENTS.0 {
                if let Some((o, _)) = $line.split_once(COMMENTS.0) {
                    opt = o.to_string();
                }
                break;
            }
            if i+1 < len && chars[i..=i+1].iter().collect::<String>() == COMMENTS.1 {
                if let Some((o, _)) = $line.split_once(COMMENTS.1) {
                    opt = o.to_string();
                }
                break;
            }
            if i+2 < len && chars[i..=i+2].iter().collect::<String>() == COMMENTS.2 {
                if let Some((o, _)) = $line.split_once(COMMENTS.2) {
                    opt = o.to_string();
                }
                break;
            }
            if i+2 < len && chars[i..=i+2].iter().collect::<String>() == COMMENTS.3 {
                if let Some((o, _)) = $line.split_once(COMMENTS.3) {
                    opt = o.to_string();
                }
                break;
            }
            if i+1 < len && chars[i..=i+1].iter().collect::<String>() == COMMENTS.4 {
                if let Some((o, _)) = $line.split_once(COMMENTS.4) {
                    opt = o.to_string();
                }
                break;
            }
        }
        if opt.trim().is_empty() {
            continue;
        } else {
            $opts.push(opt);
        }
    }
}
