use nix::libc;
use nix::sys::signal::{self, SaFlags, SigAction, SigHandler, SigSet, Signal};
use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::unistd::{fork, ForkResult};
use std::thread;
use std::time::Duration;

extern "C" fn handle_signal(signal: libc::c_int) {
    let signal = Signal::try_from(signal).unwrap();
    thread::sleep(Duration::from_millis(100));

    // TODO 4: Wait for child process to exit
    println!("signal handled");
}

fn main() {
    let handler = SigHandler::Handler(handle_signal);
    let mut mask = SigSet::empty();
    mask.add(Signal::SIGCHLD);

    // TODO 1: Register the handler in sigaction

    let mut processes = 10;

    while processes > 0 {
        processes -= 1;

        // TODO 2: Create the child process

        /* match pid {
            Ok(pid) => match pid {
                ForkResult::Parent { child } => {}
                ForkResult::Child => {

                    // TODO 3: close the process
                }
            },
            Err(error) => eprintln!("For failed {}", error),
        } */
    }

    loop {
        thread::sleep(Duration::from_millis(10));
    }
}
