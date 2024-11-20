#![no_std]
#![no_main]

use user_lib::{exec, exit, fork, wait, yield_};
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    exit(main());
}

/* 程序⾸先调⽤了 exit 函数，该函数会在程序执⾏结束时将返回码返回给操作系统。然后程序定义了⼀个 main 函数，
该函数会调⽤ fork 函数创建⼦进程并返回⼦进程的 PID。如果当前进程是⼦进程，则调⽤exec 函数启动⼀个 /bin/bash 进程，
并使⽤ environ 数组来设置进程的环境变量。如果当前进程是⽗进程，则进⼊⼀个死循环，调⽤ wait 函数等待⼦进程的退出并处理僵⼫进程。*/

#[no_mangle]
fn main() -> i32 {
    let path = "/bin/bash\0";
    let environ = [
        "SHELL=/bash\0".as_ptr(),
        "PWD=/\0".as_ptr(),
        "LOGNAME=root\0".as_ptr(),
        "MOTD_SHOWN=pam\0".as_ptr(),
        "HOME=/root\0".as_ptr(),
        "LANG=C.UTF-8\0".as_ptr(),
        "TERM=vt220\0".as_ptr(),
        "USER=root\0".as_ptr(),
        "SHLVL=0\0".as_ptr(),
        "OLDPWD=/root\0".as_ptr(),
        "PS1=\x1b[1m\x1b[32mNPUCore\x1b[0m:\x1b[1m\x1b[34m\\w\x1b[0m\\$ \0".as_ptr(),
        "_=/bin/bash\0".as_ptr(),
        "PATH=/:/bin\0".as_ptr(),
        "LD_LIBRARY_PATH=/\0".as_ptr(),
        core::ptr::null(),
    ];
    if fork() == 0 {
        exec(
            path,
            &[path.as_ptr() as *const u8, core::ptr::null()],
            &environ,
        );
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            // ECHLD is -10
            if pid == -10 {
                yield_();
                continue;
            }
            user_lib::println!(
                "[initproc] Released a zombie process, pid={}, exit_code={}",
                pid,
                exit_code,
            );
        }
    }
    0
}
