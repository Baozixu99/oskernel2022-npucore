#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{getpid, yield_};
use user_lib::{fork, wait, exit, sleep, get_time,print_tcb};

#[no_mangle]
pub fn main() -> i32 {
    let mut message = [0 as usize; 6];
    let mut message_ptr = &mut message as *mut usize;

    for _ in 0..2 {
        let pid = fork();
        if pid == 0 {
            for _ in 0..2 {
                println!("pid {} is running", getpid());
                sleep(100 as usize);
                //yield_();
                print_tcb(message_ptr);
                println!("pid of the TCB is: {} ",message[0]);
                println!("trap context physics page number of the TCB is: {} ",message[5]);
            }
            println!("pid {} exit!", getpid());
            exit(0);
        }
    }
    let mut exit_code: i32 = 0;
    for _ in 0..2 {
        assert!(wait(&mut exit_code) > 0);
        assert_eq!(exit_code, 0);
    }
    assert!(wait(&mut exit_code) < 0);
    println!("switch test pass.");
    0
}