use std::os::fd::AsRawFd;

const BTRFS_IOCTL_MAGIC: u64 = 0x94;
const IOCTL: u64 = 16;
const SNAPSHOT_READONLY: u64 = 0x02;
const SNAPSHOT_ASYNC: u64 = 0x04;
const BTRFS_IOC_SNAP_CREATE_V2: u64 = 0x50009417;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct btrfs_qgroup_limit {
    flags:    u64,
    max_rfer: u64,
    max_excl: u64,
    rsv_rfer: u64,
    rsv_excl: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct btrfs_qgroup_inherit {
    flags:           u64,
    num_qgroups:     u64,
    num_ref_copies:  u64,
    num_excl_copies: u64,
    limit:           btrfs_qgroup_limit,
    qgroups:         [u64; 20],
}

#[repr(C)]
//#[derive(Debug)]
union btrfs_ioctl_vol_args_v2_union1 {
    qgroup: btrfs_qgroup_inherit,
    unused: [u64; 4],
}

#[repr(C)]
//#[derive(Debug)]
union btrfs_ioctl_vol_args_v2_union2 {
    name:     [u8; 4040],
    devid:    u64,
    subvolid: u64,
}

#[repr(C)]
//#[derive(Debug)]
struct btrfs_ioctl_vol_args_v2 {
    fd:      i64,
    transid: u64,
    flags:   u64,
    qgroups: [u64; 4],
    name:    [u8; 4040],
}

pub fn make_syscall() -> Result<(), &'static str> {
    let file = match std::fs::File::open("/") {
        Ok(file) => Ok(file),
        Err(_)   => Err("File error")
    }?;
    let dir = match std::fs::File::open("/.snapshots") {
        Ok(dir) => Ok(dir),
        Err(_)  => Err("Dir error")
    }?;

    let filefd = file.as_raw_fd();
    let dirfd = dir.as_raw_fd();

    let mut args: btrfs_ioctl_vol_args_v2 = unsafe { std::mem::zeroed() };
    
    args.fd = 3;
    args.flags = SNAPSHOT_READONLY;
    args.transid = 0;

    let name = *b"arch\0";
    unsafe { args.name[..name.len()].copy_from_slice(&name); }

   // unsafe {
   //     args.union2.name[..name.len()].copy_from_slice(name);
// //       args.union2.name[len] = 0;
   // }
    unsafe {
        let result: i64;

        std::arch::asm!(
            "syscall",
            in("rax") IOCTL,
            in("rdi") 4,
            in("rsi") BTRFS_IOC_SNAP_CREATE_V2,
            in("rdx") &mut args as *mut _,
            lateout("rax") result,
            options(nostack, preserves_flags, readonly)
        );
        if result == -1 {
            println!("Провал! {} {} {}", args.fd, args.flags, args.transid );
            println!("{:?}", args.name);
            println!("returned: {}", result);
            return Err("Провал");
        } else {
            println!("Ура, победа! {} {} {}", args.fd, args.flags, args.transid );
            println!("returned: {}", result);
            return Ok(());
        }
    }
}
