#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{
    open, close, write, read, linkat, fstat,
    OpenFlags, Stat,
};
const AT_FDCWD: usize = 100usize.wrapping_neg();
#[no_mangle]
pub fn main() -> i32 {
    let original_file = "original.txt\0";
    let link_file = "link.txt\0";
    let content = "Hello, I'm original.txt!\n";

    // 创建并写入原始文件
    let fd_orig = open(original_file, OpenFlags::CREATE | OpenFlags::WRONLY);
    if fd_orig < 0 {
        println!("Failed to create file: {}", original_file);
        return -1;
    }
    let fd_orig = fd_orig as usize;
    let bytes_written = write(fd_orig, content.as_bytes());
    println!("Content written to original.txt: {}", content);
    if bytes_written < 0 {
        println!("Failed to write to file: {}", original_file);
        close(fd_orig);
        return -1;
    }
    close(fd_orig);

    // 使用 linkat 创建硬链接
    let ret = linkat(
        AT_FDCWD,
        original_file,
        AT_FDCWD,
        link_file,
    );
    if ret < 0 {
        println!("linkat failed");
        return -1;
    }else {
        println!("Hard link created successfully.");
    }

    // 打开原始文件和硬链接文件
    let fd_orig = open(original_file, OpenFlags::RDONLY);
    let fd_link = open(link_file, OpenFlags::RDONLY);
    if fd_orig < 0 || fd_link < 0 {
        println!("Failed to open files");
        return -1;
    }

    // 获取文件状态
    let mut stat_orig = Stat::new(0, 0, 0, 0, 0, 0 , 0, 0, 0);
    let mut stat_link = Stat::new(0, 0, 0, 0, 0, 0 , 0, 0, 0);
    let ret_orig = fstat(fd_orig as usize, &mut stat_orig);
    let ret_link = fstat(fd_link as usize, &mut stat_link);
    if ret_orig < 0 || ret_link < 0 {
        println!("fstat failed");
        close(fd_orig as usize);
        close(fd_link as usize);
        return -1;
    }

    // 比较 inode 号，因为创建硬链接不会复制文件数据，而是增加对同一个 inode 的引用计数。
    if stat_orig.get_ino()== stat_link.get_ino(){
        println!(" Inode[original.txt]: {}，Inode[link.txt]: {}", stat_orig.get_ino(),stat_link.get_ino());
    } else {
        println!("Hard link creation failed. Inodes are different.");
    }

    // 读取硬链接文件的内容
    let mut buffer = [0u8; 64];
    let bytes_read = read(fd_link as usize, &mut buffer) as usize;
    if bytes_read == 0 {
        println!("Failed to read from link file");
        close(fd_orig as usize);
        close(fd_link as usize);
        return -1;
    }
    let content_read = core::str::from_utf8(&buffer[..bytes_read]).unwrap();
    println!("Content read from link file[link.txt]: {}", content_read);

    // 关闭文件描述符
    close(fd_orig as usize);
    close(fd_link as usize);

    println!("linkat_test passed!");
    0
}