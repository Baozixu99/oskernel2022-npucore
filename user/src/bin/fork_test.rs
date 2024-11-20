#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{exit, fork, wait,sleep};
const MAX_CHILD: usize = 30;

#[no_mangle]
pub fn main() -> i32 {
    for i in 0..MAX_CHILD {
        let pid = fork();
        if pid == 0 {
            println!("I am child {}", i);
            exit(0);
        } else {
            println!("forked child pid = {}", pid);
        }
    
        assert!(pid > 0);
    }
    let mut exit_code: i32 = 0;
    for _ in 0..MAX_CHILD {
        if wait(&mut exit_code) <= 0 {  //如果 wait 返回一个正数，表示成功等待到了一个子进程的终止。
            panic!("wait stopped early");
        }
    }
    if wait(&mut exit_code) > 0 {   //检查是否所有子进程是否都已经终止
        panic!("wait got too many"); //如果这次 wait 返回一个正数，表示还有更多的子进程可等待，这显然是不正常的，因为应该已经没有更多的子进程了。
    }
    println!("forktest pass.");
    0
}
