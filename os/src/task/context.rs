use crate::trap::trap_return;

#[repr(C)]
pub struct TaskContext {
    ra: usize,   //存储返回地址return address ( e.g. __restore ) of __switch ASM function
    sp: usize,  //存储栈指针
    s: [usize; 12],  //12个usize类型的数组，用于存储s0-s11寄存器状态
}

impl TaskContext {
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
    pub fn goto_trap_return(kstack_ptr: usize) -> Self {
        Self {
            ra: trap_return as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
