use nix::errno::Errno;
use nix::sys::wait::waitpid;
use nix::unistd::dup2;
use nix::unistd::execvp;
use nix::unistd::{close, fork, pipe, ForkResult};
use std::ffi::CString;

fn main() -> Result<(), Errno> {
    // TODO 1 create a pipe (use the pipe function)
    // Hint: search https://docs.rs/nix/latest/nix/

    // TODO 2 fork two times
    // Parent - connects the two children using a pipe
    // Child1 - executes ls -l
    // Child2 - executes grep src
    // This is how ls -l | grep src is implemented
    // As the main function returns Result<(), Errno>, use the ? operator to unwrap the fork
    let _pid = todo!();
    match _pid {
        ForkResult::Parent { child } => {
            // TODO 6 fork

            match _pid {
                ForkResult::Parent { child } => {
                    // TODO 11 close both ends of the pipe

                    // TODO 12 wait for both children
                    // Hint: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-to-break-apart-values
                }
                ForkResult::Child => {
                    // TODO 7 close the unused end of the pipe

                    // TODO 8 duplicate the pipe read end to use stdin

                    // TODO 9 close the reading end of the pipe

                    // TODO 10 execute grep src
                }
            }
        }
        ForkResult::Child => {
            // TODO 3 close the unused end of the pipe

            // TODO 3 duplicate the pipe write end to use stdout

            // TODO 4 close the writing end of the pipe

            // TODO 5 execute ls -l
        }
    }

    Ok(())
}
