use nix::unistd::{close, fork, pipe, ForkResult};
use std::fs;
use std::io::{stdin, Read, Write};
use std::os::unix::io::FromRawFd;

fn main() {
    // TODO 1 create a pipe (use the pipe function)
    // Hint: search https://docs.rs/nix/latest/nix/

    // TODO 14 create a new pipe used for reading back from the child
    // close the unused ends in the parent and in the child

    // TODO 2 fork
    // Parent - reads a string from the keyboard and sends it to the pipe
    // Child - write the message on the screen

    /* match _pid {
        Ok(pid) => match pid {
            ForkResult::Parent { .. } => {
                // TODO 3 close the reading end of the pipe

                // TODO 4 read a string from the keyboard and delete the line ending
                // Hint: use trim

                // TODO 5 get File from the pipe's file descriptor
                // Hint: search from_raw_fs

                // TODO 6 write to the pipe using the write method of File
                // Hint: f.write(...) and s.as_bytes()

                // TODO 8 close the writing end of the pipe

                // TODO 8 run the program, why does it panic?

                // TODO 16 read a string from the second pipe

                // TODO 17 print the string
            }
            ForkResult::Child => {
                // TODO 9 close the writing end of the pipe

                // TODO 10 get a File from the pipe's reading end

                // TODO 11 use the File's read_to_string method to read a string from the pipe

                // TODO 12 close the reading end of the pipe

                // TODO 13 print the string

                // TODO 15 reverse the string

                // TODO 15 write the string to the second pipe
            }
        },
        Err(error) => eprintln!("For failed {}", error),
    }
    */
}
