#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::getrusage;

#[no_mangle]
pub fn main() -> i32 {
    let mut rusage = [0 as usize; 18];
    let mut rusage_ptr = &mut rusage as *mut usize;
    getrusage(0, rusage_ptr);
    unsafe{
        let utime_sec = *(rusage_ptr.offset(0));
        let utime_nsec= *(rusage_ptr.offset(1));
        let stime_sec = *(rusage_ptr.offset(2));
        let stime_nsec= *(rusage_ptr.offset(3));
        println!("user cpu time:{}ns",utime_sec * 1000000 + utime_nsec);
        println!("system cpu time:{}ns",stime_sec * 1000000 + stime_nsec);
    }
    0
}
