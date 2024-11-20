#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{exit, fork, wait,sleep};
const MAX_CHILD: usize = 30;

#[no_mangle]
pub fn main() -> i32 {
    
        let pid = fork();
        if pid != 0{
            println!("Creating father processes: {}",pid);
        }else if pid == 0 {
            println!("Creating child processes: {}",pid);
            println!("Child process {} is running", pid);
            sleep(2);  // 模拟阻塞
            println!("The child process {} is blocked", pid);
            exit(0);
        } 
    let mut exit_code: i32 = 0;
    if wait(&mut exit_code) <= 0 {
            panic!("wait stopped early");
        }
    if wait(&mut exit_code) > 0 {
        panic!("wait got too many");
    }
    0
    // for i in 0..MAX_CHILD {
    //     let pid = fork();
    //     if pid == 0 {
    //         println!("I am child {}", i);
    //         exit(0);
    //     } else {
    //         println!("forked child pid = {}", pid);
    //     }
    
    //     assert!(pid > 0);
    // }
    // let mut exit_code: i32 = 0;
    // for _ in 0..MAX_CHILD {
    //     if wait(&mut exit_code) <= 0 {
    //         panic!("wait stopped early");
    //     }
    // }
    // if wait(&mut exit_code) > 0 {
    //     panic!("wait got too many");
    // }
    // println!("forktest pass.");
    // 0
}
