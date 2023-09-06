use crate::io;
use crate::cell::OnceCell;
use cortex_m_semihosting::hio;
use core::ops::Deref;

pub struct Stdin;
pub struct Stdout {
    host_stdout : OnceCell<hio::HostStream>,
}
pub struct Stderr;

extern "C" {
    fn uart_write ( buff : *const u8, buff_len : usize);
}

impl Stdin {
    pub const fn new() -> Stdin {
        Stdin
    }
}

impl io::Read for Stdin {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout {
            host_stdout : OnceCell::new(),
        }
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        _ = self.host_stdout.get_or_init( || { hio::hstdout().unwrap() });
        let host_stdout = self.host_stdout.get_mut().unwrap();
        match host_stdout.write_all(buf) {
            Ok(()) => Ok(buf.len()),
            Err(()) => Err(io::Error::new(io::ErrorKind::Other, "semihosting write failed")),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr
    }
}

impl io::Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}

pub fn panic_output() -> Option<Vec<u8>> {
    None
}
