use std::arch::asm;

use crate::thread::Thread;
use crate::thread::ThreadContext;
use crate::thread::ThreadState;

pub const DEFAULT_STACK_SIZE: usize = 1024 * 1024 * 2; // 2MB
const MAX_THREADS: usize = 4;

// global mutable pointer to our runtime
pub static mut RUNTIME: usize = 0;

pub struct Runtime {
    threads: Vec<Thread>,
    current_thread: usize, // pointer to which thread we're running
}

/*
* Initialize the Runtime to the base state
*/
impl Runtime {
    pub fn new() -> Self {
        let base_thread = Thread {
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: ThreadState::Running,
        };

        let mut threads = vec![base_thread];

        let mut available_threads: Vec<Thread> = (1..MAX_THREADS).map(|_| Thread::new()).collect();

        threads.append(&mut available_threads);

        Runtime {
            threads,
            current_thread: 0,
        }
    }

    /*
     * We want to initialize the global mutable runtime ptr
     */
    pub fn init(&self) {
        unsafe {
            let runtime_ptr: *const Runtime = self;
            RUNTIME = runtime_ptr as usize;
        }
    }

    pub fn run(&mut self) -> ! {
        // runs till t_yield() if false then we exit
        while self.t_yield() {}
        std::process::exit(0);
    }

    // return function for when a thread is finished
    fn t_return(&mut self) {
        if self.current_thread != 0 {
            self.threads[self.current_thread].state = ThreadState::Available;
            self.t_yield(); // yield control to other tasks
        }
    }

    /*
     * t_yield is our runtime's scheduler
     *
     * It will go through all the threads and
     * see if any of them are in the /READY/ state,
     * (i.e a database call is finished)
     *
     * If no threads are /READY/, we're finished
     *
     * This is only a simple Round-Robin scheduler
     */
    #[inline(never)] // rustc plz dont optimize
    fn t_yield(&mut self) -> bool {
        let mut pos = self.current_thread;

        // going through all the threads that we have
        while self.threads[pos].state != ThreadState::Ready {
            pos += 1;
            if pos == self.threads.len() {
                pos = 0;
            }

            if pos == self.current_thread {
                return false;
            }
        }

        // we found at least one thread in READY state
        if self.threads[self.current_thread].state != ThreadState::Available {
            self.threads[self.current_thread].state = ThreadState::Ready;
        };

        self.threads[pos].state = ThreadState::Running;
        let old_pos = self.current_thread;
        self.current_thread = pos;

        unsafe {
            let old: *mut ThreadContext = &mut self.threads[old_pos].ctx;
            let new: *const ThreadContext = &self.threads[pos].ctx;

            asm!("call switch", in("rdi") old, in("rsi") new, clobber_abi("C"))
        }

        !self.threads.is_empty()
    }
}
