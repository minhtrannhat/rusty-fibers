/*
*  Each Thread got its own stack, threadcontext (for the registers that CPU needs to resume) and
*  state of the thread
*/

use crate::runtime::DEFAULT_STACK_SIZE;

pub struct Thread {
    pub stack: Vec<u8>,
    pub ctx: ThreadContext,
    pub state: ThreadState,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ThreadState {
    Available, // Thread ready to be assigned to a task
    Running,   // Thread running
    Ready, // Thread ready to move forward and resume execution (i.e after a blocking network syscall)
}
/* ThreadContext struct
*
* - Assuming the machine's arch is x86_64
*/

/*
* Preserving callee-saved registers:
* The callee must preserve the values of specific registers
* (like rbx, rbp, r12-r15) if it modifies them.
* It must restore these registers
* to their original values before returning.
*/
#[derive(Debug, Default)]
#[repr(C)]
pub struct ThreadContext {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
}

/*
* Once a stack is allocated, it must not move otherwise all pointers are invalidated
*/
impl Thread {
    pub fn new() -> Self {
        Thread {
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: ThreadState::Available,
        }
    }
}
