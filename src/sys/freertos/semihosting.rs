//use cortex_m_semihosting::hio;

//! Host I/O

use cortex_m_semihosting::{syscall};
use core::{fmt, slice};

pub use cortex_m_semihosting::nr;

/// A byte stream to the host (e.g., host's stdout or stderr).
#[derive(Clone, Copy)]
pub struct HostStream {
    fd: usize,
}

impl HostStream {
    /// Attempts to write an entire `buffer` into this sink
    pub fn write_all(&self, buffer: &[u8]) -> Result<(), ()> {
        write_all(self.fd, buffer)
    }
}

impl fmt::Write for HostStream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}


pub fn open(name: &str, mode: usize) -> Result<HostStream, ()> {
    let name = name.as_bytes();
    match unsafe { syscall(nr::OPEN,
                &[
                        name.as_ptr() as usize,
                        mode as usize,
                        name.len() - 1 as usize ] ) } as isize {
        -1 => Err(()),
        fd => Ok(HostStream { fd: fd as usize }),
    }
}

pub fn write_all(fd: usize, mut buffer: &[u8]) -> Result<(), ()> {
    while !buffer.is_empty() {
        match unsafe { syscall(nr::WRITE,
                        &[
                            fd as usize,
                            buffer.as_ptr() as usize,
                            buffer.len() as usize ]) } {
            // Done
            0 => return Ok(()),
            // `n` bytes were not written
            n if n <= buffer.len() => {
                let offset = (buffer.len() - n) as isize;
                buffer = unsafe { slice::from_raw_parts(buffer.as_ptr().offset(offset), n) }
            }
            #[cfg(feature = "jlink-quirks")]
            // Error (-1) - should be an error but JLink can return -1, -2, -3,...
            // For good measure, we allow up to negative 15.
            n if n > 0xfffffff0 => return Ok(()),
            // Error
            _ => return Err(()),
        }
    }
    Ok(())
}
