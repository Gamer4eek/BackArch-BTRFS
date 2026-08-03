mod parser;
//mod syscalls;

fn main() -> Result<(), &'static str> {
    let mut snap_info = parser::SnapshotInfo::new();

    snap_info.parse_args()?;
    snap_info.parse_config()?;
    snap_info.set_defaults();

    println!("{:#?}", snap_info);

//    syscalls::make_syscall()?;

    Ok(())
}
