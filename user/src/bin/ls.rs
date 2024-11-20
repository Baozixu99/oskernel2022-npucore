#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{open, close, getdents64, OpenFlags, Dirent};
const MAX_FILES: usize = 128; // 假设最多有 128 个文件
const MAX_NAME_LEN: usize = 256; // 假设文件名最长为 256 字节
#[no_mangle]
pub fn main() -> i32 {
    let path = ".\0";
    let fd = open(path, OpenFlags::RDONLY | OpenFlags::DIRECTORY);
    if fd < 0 {
        println!("Failed to open directory: {}", path);
        return -1;
    }
    let fd = fd as usize;
    let mut buf = [0u8; 1024];
    let mut names: [[u8; MAX_NAME_LEN]; MAX_FILES] = [[0; MAX_NAME_LEN]; MAX_FILES]; //二维数组，存储文件名
    let mut name_count = 0;
    loop { //单次读取可能无法获取所有目录项，因此需要循环读取直到读取完所有内容。
        let nread = getdents64(fd, &mut buf);
        if nread < 0 {
            println!("getdents64 failed");
            close(fd);
            return -1;
        }
        if nread == 0 {  //读取到文件末尾，退出循环。
            break;
        }

        let mut pos = 0;  //当前处理的缓冲区位置，初始值为 0。
        while pos < nread as usize {
            let dirent = unsafe { &*(buf.as_ptr().add(pos) as *const Dirent) };
            // println!("d_ino: {}, d_off: {}, d_reclen: {}, d_name: {}",
            //          dirent.d_ino, dirent.d_off, dirent.d_reclen, core::str::from_utf8(&dirent.d_name).unwrap_or(""));
            // let name = core::str::from_utf8(&dirent.d_name).unwrap_or("");
            // println!("{}", name);
            let name_len = dirent.d_name.iter().position(|&c| c == 0).unwrap_or(dirent.d_name.len());//计算文件名的长度
            names[name_count][..name_len].copy_from_slice(&dirent.d_name[..name_len]);
            name_count += 1;
            pos += dirent.d_reclen as usize; //移动到下一个目录项的位置。
        }
    }

    close(fd);
    // 按列对齐输出
    let max_len = names.iter().map(|name| name.iter().position(|&c| c == 0).unwrap_or(0)).max().unwrap_or(0);//计算最大文件名长度
    let cols = 80 / (max_len + 2); // 假设终端宽度为 80
    for i in 0..name_count {
        let name_len = names[i].iter().position(|&c| c == 0).unwrap_or(0);
        let name = core::str::from_utf8(&names[i][..name_len]).unwrap_or("");
        print!("{:<width$}", name, width = max_len + 2);
        if (i + 1) % cols == 0 {
            println!("");
        }
    }
    if name_count % cols != 0 {
        println!(""); // 输出换行
    }
    0
}
