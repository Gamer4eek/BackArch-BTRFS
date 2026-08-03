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

    pub log_file:    Option<String>,
}

const FORBIDDEN_SYMBOLS: [&str; 24] = [
    "'", "\"", "//", ":", ";", "..", "@", "$", "#",
    "№", "*", "`", "~", "[", "]", "{", "}", "?", "<", ">",
    ",", "(", ")", "="
];

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
macro_rules! validate_value {
    ($obj:expr, $value:expr, $have_quotes:expr) => {
        if !$value.trim_matches('"').is_empty() {
            if $have_quotes == true {
                if !$value.starts_with('"') || !$value.ends_with('"') {
                    eprintln!("{}", $obj);
                    Err("Value must be put in double quotes")
                } else {
                    let no_quotes = &$value[1..$value.len()-1];
                    if FORBIDDEN_SYMBOLS.iter().any(|&s| no_quotes.contains(s)) {
                        eprintln!("{}", $obj);
                        eprintln!("List of forbidden symbols: {}", FORBIDDEN_SYMBOLS.join(", "));
                        Err("Value contains forbidden symbol(s)")
                    } else { Ok(no_quotes) }
                }
            } else {
                if FORBIDDEN_SYMBOLS.iter().any(|&s| $value.contains(s)) {
                    eprintln!("{}", $obj);
                    eprintln!("List of forbidden symbols: {}", FORBIDDEN_SYMBOLS.join(", "));
                    Err("Value contains forbidden symbol(s)")
                } else { Ok($value) }
            }
        } else { eprintln!("{}", $obj); Err("Empty value") }
    }
}

fn help(tutorial: bool) -> Result<(), &'static str> {
    if tutorial == true {
        return Ok(());
    } else {
        println!("Usage(must have root access): backarch [--option=value]");
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
        return Ok(());
    }
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
            log_file:    None,
        }
    }
    pub fn parse_args(&mut self) -> Result<(), &'static str> {
        let args: Vec<String> = std::env::args().skip(1).collect();    
        for arg in args {
            if let Some((key, value)) = arg.split_once('=') {
                let data = { 
                    if value.starts_with('"') && value.len() < 2 {
                        eprintln!("{}", arg); Err("Empty value")?
                    } else { validate_value!(arg, value, false)? }
                };
                match key {
                    "--name"        => { insert_values!(self; name        = data); Ok(()) }

                    "--config"      => { insert_values!(self; config_file = data); Ok(()) }

                    "--drive-uuid"  => { insert_values!(self; drive_uuid  = data); Ok(()) }
                    "--grub-file"   => { insert_values!(self; grub_file   = data); Ok(()) }

                    "--hooks-dir"   => { insert_values!(self; hooks_dir   = data); Ok(()) }
                    "--dir"         => { insert_values!(self; dir         = data); Ok(()) }
                    "--ro-dir"      => { insert_values!(self; ro_dir      = data); Ok(()) }

                    "--fsroot-path" => { insert_values!(self; fsroot_path = data); Ok(()) }
                    "--usr-path"    => { insert_values!(self; usr_path    = data); Ok(()) }
                    "--var-path"    => { insert_values!(self; var_path    = data); Ok(()) }
                    "--boot-path"   => { insert_values!(self; boot_path   = data); Ok(()) }
                    "--home-path"   => { insert_values!(self; home_path   = data); Ok(()) }
                    "--root-path"   => { insert_values!(self; root_path   = data); Ok(()) }

                    "--log"         => { insert_values!(self; log_file    = data); Ok(()) }

                    _ => { eprintln!("{}", arg); Err("Wrong argument") }
                }?
            } else {
                if arg == "--help" {
                    help(false)?; std::process::exit(0);
                } else if arg == "--tutorial" {
                    help(true)?;  std::process::exit(0);
                } else {
                    eprintln!("{}", arg);
                    Err("Wrong argument")? 
                }
            }
            
        }
        insert_values!(self; config_file = "/etc/backarch/backarch.conf");
        Ok(())
    }
    pub fn parse_config(&mut self) -> Result<(), &'static str> {
        if let Some(conf) = &self.config_file {
            match std::fs::File::open(&conf) {
                Ok(file) => {
                    let mut reader = std::io::BufReader::new(file);
                    for line in std::io::BufRead::lines(&mut reader) {
                        let line = match line {
                            Ok(line)  => line,
                            Err(_)    => { eprintln!("Warning: could not find or open the configuration file"); " ".to_string() }
                        };
                        if line.is_empty() { 
                            continue;
                        }
                        let opt = match line.split(";;").next() {
                            Some(opt) => opt.trim(),
                            None      => ""
                        };
                        if let Some((key, value)) = opt.split_once('=') { 
                            let data = { 
                                if value.starts_with('"') && value.len() < 2 {
                                    eprintln!("{}", opt); Err("Empty value")?
                                } else { validate_value!(opt, value, true)? }
                            };
                            match key {
                                "name"        => { insert_values!(self; name        = data); Ok(()) }

                                "drive_uuid"  => { insert_values!(self; drive_uuid  = data); Ok(()) }
                                "grub_file"   => { insert_values!(self; grub_file   = data); Ok(()) }

                                "hooks_dir"   => { insert_values!(self; hooks_dir   = data); Ok(()) }
                                "dir"         => { insert_values!(self; dir         = data); Ok(()) }
                                "ro_dir"      => { insert_values!(self; ro_dir      = data); Ok(()) }

                                "fsroot_path" => { insert_values!(self; fsroot_path = data); Ok(()) }
                                "usr_path"    => { insert_values!(self; usr_path    = data); Ok(()) }
                                "var_path"    => { insert_values!(self; var_path    = data); Ok(()) }
                                "boot_path"   => { insert_values!(self; boot_path   = data); Ok(()) }
                                "home_path"   => { insert_values!(self; home_path   = data); Ok(()) }
                                "root_path"   => { insert_values!(self; root_path   = data); Ok(()) }

                                "log"         => { insert_values!(self; log_file    = data); Ok(()) }
                                
                                _ => { eprintln!("{}", key); Err("Invalid option") } 
                            }?;
                        } else { 
                            if opt.is_empty() {
                                return Ok(());
                            } else {
                                eprintln!("{}", opt); Err("Invalid option")?
                            }
                        }
                    }
                    return Ok(());
                }
                Err(_) => { eprintln!("Warning: could not find or open the configuration file"); return Ok(()); }
            }
        } else {
            eprintln!("Warning: could not find config file");
            Ok(())
        }
    }
    pub fn set_defaults(&mut self) {
        insert_values!(
            self;
            name        = "Archlinux",
            config_file = "/etc/backarch/backarch.conf",
            grub_file   = format!("40_backarch_{}", { if let Some(name) = &self.name { name } else { "ArchLinux" } }),
            hooks_dir   = "/etc/backarch/hooks",
            dir         = "/.snapshots",
            ro_dir      = "/.snapshots_ro",
            fsroot_path = "/",
            usr_path    = "/usr",
            var_path    = "/var",
            boot_path   = "/boot",
            home_path   = "/home",
            root_path   = "/root",
            log_file    = "/var/log/backarch/backarch.log",
        );
    }
}

fn main() -> Result<(), &'static str> {
    let mut snap_info = SnapshotInfo::new();

    snap_info.parse_args()?;
    snap_info.parse_config()?;
    snap_info.set_defaults();

    println!("{:#?}", snap_info);

    Ok(())
}
