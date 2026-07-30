//use std::fs;
//use btrfs_uapi;

pub struct SnapshotInfo {
    pub name:        Option<String>,
    pub cache_path:  Option<String>,
    pub path:        Option<String>,
    pub fsroot_path: Option<String>,
    pub usr_path:    Option<String>,
    pub var_path:    Option<String>,
    pub boot_path:   Option<String>,
    pub home_path:   Option<String>,
    pub root_path:   Option<String>,
}

macro_rules! insert_values {
    ($struct:ident; 
     $($field:ident = $value:expr),* $(,)?
     ) => {
        $(
            if $struct.$field == None {
                $struct.$field = Some($value.to_string());
            }
        )*
    }
}

impl SnapshotInfo {
    pub fn new() -> Self {
        Self {
            name:        None,
            cache_path:  None,
            path:        None,
            fsroot_path: None,
            usr_path:    None,
            var_path:    None,
            boot_path:   None,
            home_path:   None,
            root_path:   None,
        }
    }
    pub fn parse_args(&mut self, args: &Vec<String>) -> Result<(), String> {
        for arg in args {
            if let Some((key, value)) = arg.split_once('=') {
                match key {
                    "--snap-name"        => { insert_values!(self; name        = value); Ok(()) }
                    "--snap-cache-path"  => { insert_values!(self; cache_path  = value); Ok(()) }
                    "--snap-path"        => { insert_values!(self; path        = value); Ok(()) }
                    "--snap-fsroot-path" => { insert_values!(self; fsroot_path = value); Ok(()) }
                    "--snap-usr-path"    => { insert_values!(self; usr_path    = value); Ok(()) }
                    "--snap-var-path"    => { insert_values!(self; var_path    = value); Ok(()) }
                    "--snap-boot-path"   => { insert_values!(self; boot_path   = value); Ok(()) }
                    "--snap-home-path"   => { insert_values!(self; home_path   = value); Ok(()) }
                    "--snap-root-path"   => { insert_values!(self; root_path   = value); Ok(()) }

                    _ => Err(format!("Wrong argument: {}", arg))
                }?
            } else { Err(format!("Wrong argument: {}", arg))? }
        }
        Ok(())
    }
    pub fn parse_config(&mut self, reader: &mut std::io::BufReader<std::fs::File>) -> Result<(), String> {
        for line in std::io::BufRead::lines(reader) {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(format!("Error parsing config"))
            };
            let opt = match line.split("//").next() {
                Some(opt) => opt.trim(),
                None => ""
            };
            if let Some((key, value)) = opt.split_once('=') {
                match key {
                    "snap_name"        => { insert_values!(self; name        = value); Ok(()) }
                    "snap_cache_path"  => { insert_values!(self; cache_path  = value); Ok(()) }
                    "snap_path"        => { insert_values!(self; path        = value); Ok(()) }
                    "snap_fsroot_path" => { insert_values!(self; fsroot_path = value); Ok(()) }
                    "snap_usr_path"    => { insert_values!(self; usr_path    = value); Ok(()) }
                    "snap_var_path"    => { insert_values!(self; var_path    = value); Ok(()) }
                    "snap_boot_path"   => { insert_values!(self; boot_path   = value); Ok(()) }
                    "snap_home_path"   => { insert_values!(self; home_path   = value); Ok(()) }
                    "snap_root_path"   => { insert_values!(self; root_path   = value); Ok(()) }
                    
                    _ => Err(format!("Invalid option: {}", key))
                }?
            } else { Err(format!("Invalid option: {}", opt))? }
        }
        Ok(())
    }
    pub fn set_defaults(&mut self) {
        insert_values!(
            self;
            name        = "Archlinux",
            cache_path  = "/.snapshots_cache",
            path        = "/.snapshots",
            fsroot_path = "/",
            usr_path    = "/usr",
            var_path    = "/var",
            boot_path   = "/boot",
            home_path   = "/home",
            root_path   = "/root",
        );
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();    
    let mut snap_info = SnapshotInfo::new();

    snap_info.parse_args(&args)?;
    match std::fs::File::open("/etc/backarch/backarch.conf") {
        Ok(file) => {
            let mut reader = std::io::BufReader::new(file);
            if let Err(e) = snap_info.parse_config(&mut reader) {
                Err(format!("Error parsing config: {}", e))
            } else {
                Ok(())
            }
        }
        Err(_) => { println!("Warning: could not find config file"); Ok(()) }
    }?;
    snap_info.set_defaults();

    Ok(())
}
