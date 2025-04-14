use nix::libc::{self, siginfo_t};
use nix::sys::signal::{self, sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal};
use std::error::Error;
use std::os::raw::{c_int, c_void};
use std::thread;
use std::time::Duration;

extern "C" fn sigsegv_handler(_signal: c_int, siginfo: *mut siginfo_t, _extra: *mut c_void) {
    let address = unsafe { (*siginfo).si_addr() } as usize;
    // TODO 3: Print the address and exit the process
}
const data: usize = 10;

fn main() {
    unsafe {
        let handler = SigHandler::SigAction(sigsegv_handler);
        let sigaction_data = SigAction::new(handler, SaFlags::empty(), SigSet::all());

        // TODO 1: register the SIGSEGV handler

        let p = 0 as *mut usize; // This is the null pointer

        // TODO 2: Write a value to the address indicated by the pointer

        let p = &data as *const usize as *mut usize; // This is the pointer to a constant
    }
}
