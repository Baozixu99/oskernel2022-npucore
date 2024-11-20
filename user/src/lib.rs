#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#[allow(dead_code)]
#[macro_use]
pub mod console;
mod lang_items;
mod syscall;
mod usr_call;

extern crate alloc;
#[macro_use]
extern crate bitflags;

use core::arch::asm;

use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;
pub use usr_call::*;

const USER_HEAP_SIZE: usize = 32768;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap<32> = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[linkage = "weak"]
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    let argc: usize;
    let argv: usize;
    unsafe {
        asm!(
            "ld a0, 0(sp)",
            "add a1, sp, 8",
            out("a0") argc,
            out("a1") argv
        );
    }
    _parameter(argc, argv);
}

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn _parameter(argc: usize, argv: usize) -> ! {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    let mut v: Vec<&'static str> = Vec::new();
    for i in 0..argc {
        let str_start =
            unsafe { ((argv + i * core::mem::size_of::<usize>()) as *const usize).read_volatile() };
        let len = (0usize..)
            .find(|i| unsafe { ((str_start + *i) as *const u8).read_volatile() == 0 })
            .unwrap();
        v.push(
            core::str::from_utf8(unsafe {
                core::slice::from_raw_parts(str_start as *const u8, len)
            })
            .unwrap(),
        );
    }
    exit(main(argc, v.as_slice()));
}

#[linkage = "weak"]
#[no_mangle]
fn main(_argc: usize, _argv: &[&str]) -> i32 {
    panic!("Cannot find main!");
}

bitflags! {
    pub struct OpenFlags: u32 {
        // const RDONLY = 0;
        // const WRONLY = 1 << 0; //0b1 ->0o1
        // const RDWR = 1 << 1;
        // const CREATE = 1 << 9; //0b1 000 000 000错误    ->0o1000
        // const TRUNC = 1 << 10; //0b10 000 000 000       ->0o2000
        //修复CREATE的计算错误bug 
        const RDONLY = 0o0;
        const WRONLY = 0o1;
        const RDWR   = 0o2;
        const CREATE = 0o100;  //0b1 000 000 == 1<<6
        const TRUNC  = 0o1000; 
        const DIRECTORY   = 0o200000; // 要求路径必须是一个目录
    }
}
#[repr(C)]
pub struct TimeSpec {
    /// The tv_sec member represents the elapsed time, in whole seconds.
    pub tv_sec: usize,
    /// The tv_usec member captures rest of the elapsed time, represented as the number of microseconds.
    pub tv_nsec: usize,
}
#[repr(C)]
//如果 Stat 和 TimeSpec 结构体未使用 #[repr(C)] 注解，编译器可能会对字段进行重新排列，导致内存布局与预期不符。
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
    pub fn get_blocks(&self) -> usize {
        self.st_blocks as usize
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

bitflags! {
    pub struct UnlinkatFlags: u32 {
        const AT_REMOVEDIR = 0x200;
    }
}

const NAME_LIMIT: usize = 128;
#[repr(C)]  
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