use core::mem::size_of;

use crate::timer::TimeSpec;

bitflags! {
    pub struct OpenFlags: u32 {
        const O_RDONLY      =   0o0; //只读
        const O_WRONLY      =   0o1; //只写
        const O_RDWR        =   0o2; //读写

        const O_CREAT       =   0o100;
        const O_EXCL        =   0o200;
        const O_NOCTTY      =   0o400;
        const O_TRUNC       =   0o1000;

        const O_APPEND      =   0o2000;
        const O_NONBLOCK    =   0o4000;
        const O_DSYNC       =   0o10000;
        const O_SYNC        =   0o4010000;
        const O_RSYNC       =   0o4010000;
        const O_DIRECTORY   =   0o200000;
        const O_NOFOLLOW    =   0o400000;
        const O_CLOEXEC     =   0o2000000;
        const O_ASYNC       =   0o20000;
        const O_DIRECT      =   0o40000;
        const O_LARGEFILE   =   0o100000;
        const O_NOATIME     =   0o1000000;
        const O_PATH        =   0o10000000;
        const O_TMPFILE     =   0o20200000;
    }
}

bitflags! {
    pub struct SeekWhence: u32 {
        const SEEK_SET  =   0; /* set to offset bytes.  */                            //将文件指针设置到偏移量为 offset 的位置处
        const SEEK_CUR  =   1; /* set to its current location plus offset bytes.  */  //将文件指针设置到当前位置加上偏移量 offset 的位置处
        const SEEK_END  =   2; /* set to the size of the file plus offset bytes.  */  //将文件指针设置到文件末尾加上偏移量 offset 的位置
    }
}

bitflags! {
    pub struct StatMode: u32 {
        /// 文件类型位掩码
        const S_IFMT    =   0o170000;
        /// 套接字
        const S_IFSOCK  =   0o140000;
        /// 符号链接
        const S_IFLNK   =   0o120000;
        /// 常规文件
        const S_IFREG   =   0o100000;
        /// 块设备
        const S_IFBLK   =   0o060000;
        /// 目录
        const S_IFDIR   =   0o040000;
        /// 字符设备
        const S_IFCHR   =   0o020000;
        /// 命名管道（FIFO）
        const S_IFIFO   =   0o010000;

        /// 设置用户 ID 位（参见 execve(2)）
        const S_ISUID   =   0o4000;
        /// 设置组 ID 位
        const S_ISGID   =   0o2000;
        /// 粘滞位（Sticky Bit）
        const S_ISVTX   =   0o1000;

        /// 拥有者读、写、执行权限
        const S_IRWXU   =   0o0700;
        /// 拥有者读权限
        const S_IRUSR   =   0o0400;
        /// 拥有者写权限
        const S_IWUSR   =   0o0200;
        /// 拥有者执行权限
        const S_IXUSR   =   0o0100;

        /// 组读、写、执行权限
        const S_IRWXG   =   0o0070;
        /// 组读权限
        const S_IRGRP   =   0o0040;
        /// 组写权限
        const S_IWGRP   =   0o0020;
        /// 组执行权限
        const S_IXGRP   =   0o0010;

        /// 其他用户读、写、执行权限
        const S_IRWXO   =   0o0007;
        /// 其他用户读权限
        const S_IROTH   =   0o0004;
        /// 其他用户写权限
        const S_IWOTH   =   0o0002;
        /// 其他用户执行权限
        const S_IXOTH   =   0o0001;
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
/// Store the file attributes from a supported file.
pub struct Stat {
    st_dev: u64,        /// 文件所在设备的ID
    st_ino: u64,        /// Inode编号
    st_mode: u32,       /// 文件类型和模式 
    st_nlink: u32,      /// 硬链接数量
    st_uid: u32,        /// 文件所有者的用户ID
    st_gid: u32,        /// 文件所属组的组ID
    st_rdev: u64,       /// 设备ID（如果是特殊文件）
    __pad: u64,
    st_size: i64,       ///文件大小，以字节为单位
    st_blksize: u32,    ///I/O的最佳块大小
    __pad2: i32,
    st_blocks: u64,     ///分配的512字节块数
    st_atime: TimeSpec, ///最后访问时间（向后兼容）
    st_mtime: TimeSpec, ///最后修改时间
    st_ctime: TimeSpec, ///最后状态更改时间
    __unused: u64,
}

#[allow(unused)]
impl Stat {
    /// Get the inode number described in the `Stat`
    pub fn get_ino(&self) -> usize {
        self.st_ino as usize
    }
    pub fn get_size(&self) -> usize {
        self.st_size as usize
    }

    pub fn new(
        st_dev: u64,
        st_ino: u64,
        st_mode: u32,
        st_nlink: u32,
        st_rdev: u64,
        st_size: i64,
        st_atime_sec: i64,
        st_mtime_sec: i64,
        st_ctime_sec: i64,
    ) -> Self {
        const BLK_SIZE: u32 = 512;
        Self {
            st_dev,
            st_ino,
            st_mode,
            st_nlink,
            st_uid: 0,
            st_gid: 0,
            st_rdev,
            __pad: 0,
            st_size,
            st_blksize: BLK_SIZE as u32,
            __pad2: 0,
            st_blocks: (st_size as u64 + BLK_SIZE as u64 - 1) / BLK_SIZE as u64,
            st_atime: TimeSpec {
                tv_sec: st_atime_sec as usize,
                tv_nsec: 0,
            },
            st_mtime: TimeSpec {
                tv_sec: st_mtime_sec as usize,
                tv_nsec: 0,
            },
            st_ctime: TimeSpec {
                tv_sec: st_ctime_sec as usize,
                tv_nsec: 0,
            },
            __unused: 0,
        }
    }
}

const NAME_LIMIT: usize = 128;
#[derive(Clone, Copy, Debug)]
#[repr(C)]
/// Native Linux directory entry structure.
/// # Note
/// In theory, the d_name may NOT have a fixed size and `d_name` may be arbitrarily lone.
pub struct Dirent {
    /// Inode number
    pub d_ino: usize,
    /// Offset to next `linux_dirent`
    pub d_off: isize,
    /// Length of this `linux_dirent`
    pub d_reclen: u16,
    /// Type of the file
    pub d_type: u8,
    /// The Filename (null-terminated)
    /// # Note
    /// We use fix-sized d_name array.
    pub d_name: [u8; NAME_LIMIT],
}

impl Dirent {
    /// Offset to next `linux_dirent`
    pub fn new(d_ino: usize, d_off: isize, d_type: u8, d_name: &str) -> Self {
        let mut dirent = Self {
            d_ino,
            d_off,
            d_reclen: size_of::<Self>() as u16,
            d_type,
            d_name: [0; NAME_LIMIT],
        };
        dirent.d_name[0..d_name.len()].copy_from_slice(d_name.as_bytes());
        dirent
    }
}