use std::os::unix::io::{FromRawFd, RawFd};
use std::sync::{Mutex, MutexGuard};
use std::fs::File;
use std::io::Read;

use ::termios::*;

lazy_static! {
    static ref STDIN_TERM: Mutex<TermWrap> = unsafe { Mutex::new(TermWrap::from_fd(0)) };
}

pub struct TermWrap {
    // Old terminal state
    state: Termios,

    // File to read from (usually stdin)
    file: File,

    // Data buffer
    buf: [u8; 32],
}

impl TermWrap {
    pub unsafe fn from_fd(fd: RawFd) -> TermWrap {
        let mut termios = Termios::from_fd(fd).unwrap();

        let term = TermWrap {
            state: termios,
            file: File::from_raw_fd(fd),
            buf: [0; 32]
        };

        termios.c_lflag &= !(ICANON | ECHO);
        termios.c_cc[VMIN] = 0;
        termios.c_cc[VTIME] = 2;

        tcsetattr(fd, TCSANOW, &mut termios).unwrap();

        term
    }

    pub fn stdin<'a>() -> MutexGuard<'a, TermWrap> {
        STDIN_TERM.lock().unwrap()
    }

    pub fn read(&mut self) -> &[u8] {
        if let Ok(n) = self.file.read(&mut self.buf) {
            &self.buf[0..n]
        } else {
            &[]
        }
    }
}

impl Drop for TermWrap {
    fn drop(&mut self) {
        tcsetattr(0, TCSANOW, &mut self.state).unwrap();
    }
}
