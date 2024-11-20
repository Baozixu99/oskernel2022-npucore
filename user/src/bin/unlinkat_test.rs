#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{
    open, close, unlinkat, write, read, fstat,
    OpenFlags, UnlinkatFlags, Stat,
};

const AT_FDCWD: usize = 100usize.wrapping_neg();

#[no_mangle]
pub fn main() -> i32 {
    let file_path = "test_file.txt\0";
    let content = "Hello, unlinkat!\n";

    // Step 1: 创建测试文件
    let fd_file = open(file_path, OpenFlags::CREATE | OpenFlags::WRONLY);
    if fd_file < 0 {
        println!("Failed to create file: {}", file_path);
        return -1;
    }
    let fd_file = fd_file as usize;
    if write(fd_file, content.as_bytes()) < 0 {
        println!("Failed to write to file: {}", file_path);
        close(fd_file);
        return -1;
    }
    close(fd_file);
    println!("File created successfully: {}", file_path);

    // Step 2: 使用 unlinkat 删除文件
    let ret = unlinkat(AT_FDCWD, file_path, UnlinkatFlags::empty());
    if ret < 0 {
        println!("Failed to unlink file: {}", file_path);
        return -1;
    } else {
        println!("File unlinked successfully: {}", file_path);
    }
    println!("unlinkat_test passed!");
    0
}
