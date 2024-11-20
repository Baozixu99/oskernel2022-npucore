#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{
    open,
    close,
    read,
    write,
    OpenFlags,
};

#[no_mangle]
pub fn main() -> i32 {
    let test_str = "Hello, world!";
    let filea = "filea.txt\0";
    let fd = open(filea, OpenFlags::CREATE | OpenFlags::WRONLY);
    println!("Create file {}, fd: {}", filea,fd);
    assert!(fd > 0);
    let fd = fd as usize;
    write(fd, test_str.as_bytes());
    println!("Content written to filea.txt: {}", test_str);
    close(fd);
    let fd = open(filea, OpenFlags::RDONLY);
    println!("Open file {}, fd: {}", filea,fd);
    assert!(fd > 0);
    let fd = fd as usize;
    let mut buffer = [0u8; 100];
    let read_len = read(fd, &mut buffer) as usize;
    close(fd);
    println!("Content read from filea.txt: {}", core::str::from_utf8(&buffer[..read_len]).unwrap());
    assert_eq!(
        test_str,
        core::str::from_utf8(&buffer[..read_len]).unwrap(),
    );
    println!("file_test passed!");
    0
}
