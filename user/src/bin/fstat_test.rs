#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{
    open,
    close,
    fstat,
    OpenFlags,
    Stat,
};
use user_lib::write;


#[no_mangle]
pub fn main() -> i32 {
    let test_str = "Hello, world!";
    let filea = "filea\0";
    
    // 打开文件进行写入
    let fd = open(filea, OpenFlags::CREATE | OpenFlags::WRONLY);
    assert!(fd > 0);
    let fd = fd as usize;
    let bytes_written = write(fd, test_str.as_bytes());
    println!("bytes_written: {}, write content: {:?}", bytes_written, test_str);
    assert!(bytes_written as usize == test_str.len());
    close(fd);

    // 打开文件进行读取
    let fd = open(filea, OpenFlags::RDONLY);
    assert!(fd > 0);
    let fd = fd as usize;

    // 初始化,调用 fstat 获取文件状态
    let mut stat = Stat::new(1, 1, 1, 1, 1, 1 , 1, 1, 1);
    let ret = fstat(fd, &mut stat);
    assert!(ret == 0);  // 假设 fstat 成功返回 0

    // 输出文件大小
    println!("File size: {}", &stat.get_size());
    // 关闭文件描述符
    close(fd);

    println!("fstat_test passed!");
    0
}
