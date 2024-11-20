#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{
    open,
    close,
    fstatat,
    OpenFlags,
    Stat,
};
use user_lib::write;

const AT_EMPTY_PATH:u32 = 0x1000;
const AT_NO_AUTOMOUNT:u32 = 0x800;
const AT_SYMLINK_NOFOLLOW:u32 = 0x100;
const AT_FDCWD: usize = 100usize.wrapping_neg();
#[no_mangle]
pub fn main() -> i32 {
    let test_str = "Hello, world!hhhhh";
    let filea = "filea\0";
    let dirfd = AT_FDCWD;
    let flags = AT_EMPTY_PATH;
    // 打开/创建文件进行写入
    let fd = open(filea, OpenFlags::CREATE | OpenFlags::WRONLY);
    assert!(fd > 0);
    let fd = fd as usize;
    let bytes_written = write(fd, test_str.as_bytes());
    println!("bytes_written: {}, write content: {:?}", bytes_written, test_str);
    assert!(bytes_written as usize == test_str.len());
    close(fd);

    // 初始化Stat,调用 fstatat 获取文件状态
    let mut stat = Stat::new(1, 1, 1, 1, 1, 1 , 1, 1, 1);
    //println!("File size: {},{}", stat_ptr.get_size(),stat_ptr.get_ino());
    let ret = fstatat(dirfd, filea,&mut stat,flags);
    assert!(ret == 0);  // 假设 fstat 成功返回 0

    // 输出文件大小
    println!("File size: {},", &stat.get_size());
    // 关闭文件描述符
    close(fd);

    println!("fstatat_test passed!");
    0
}
