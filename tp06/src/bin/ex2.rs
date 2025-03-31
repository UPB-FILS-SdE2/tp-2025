use nix::unistd::dup2;
use nix::unistd::execvp;
use nix::unistd::{close, fork, pipe, ForkResult};
use std::ffi::CString;
use std::fs;
use std::io::Read;
use std::os::unix::io::FromRawFd;

fn main() {
    // TODO 1 create a pipe (use the pipe function)
    // Hint: search https://docs.rs/nix/latest/nix/

    // TODO 2 fork
    // Parent - reads the output from the pipe
    // Child - executes a command and redirects the output to the pipe
    // Hint: dup2

    /*  match pid {
        Ok(pid) => match pid {
            ForkResult::Parent { .. } => {
                // TODO 3 close the unused end of the pipe

                // TODO 4 read from the pipe

                // TODO 5 print the output
            }
            ForkResult::Child => {
                // TODO 6 close the unused end of the pipe

                // TODO 7 duplicate the pipe write end to use stdout

                // TODO 8 close the writing end of the pipe

                // TODO 9 execute ls -l
            }
        },
        Err(error) => eprintln!("For failed {}", error),
    }
    */
}
