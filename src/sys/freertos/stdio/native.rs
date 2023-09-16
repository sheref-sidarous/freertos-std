use crate::io;

pub struct StdinImpl;
pub struct StdoutImpl;
pub struct StderrImpl;

// Ideally, this value should be provided by the native impl.
// set it to something small enough for now
pub const STDIN_BUF_SIZE: usize = 16;

extern "C" {
    fn stdout_native_write (buff : *const u8, buff_len : usize) -> usize;
    fn stdout_native_flush ();

    fn stderr_native_write (buff : *const u8, buff_len : usize) -> usize;
    fn stderr_native_flush ();

    fn stdin_native_read (buff : *const u8, buff_max_len : usize) -> usize;
}

impl StdinImpl {
    pub(super) const fn new() -> StdinImpl {
        StdinImpl
    }

    pub(super) fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let result = unsafe {
            stdin_native_read(buf.as_ptr(), buf.len())
        };
        Ok(result)
    }
}

impl StdoutImpl {
    pub(super) const fn new() -> StdoutImpl {
        StdoutImpl
    }

    pub(super) fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = unsafe {
            stdout_native_write(buf.as_ptr(), buf.len())
        };
        Ok(result)
    }

    pub(super) fn flush(&mut self) -> io::Result<()> {
        unsafe {
            stdout_native_flush();
        }
        Ok(())
    }
}

impl StderrImpl {
    pub(super) const fn new() -> StderrImpl {
        StderrImpl
    }

    pub(super) fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = unsafe {
            stderr_native_write(buf.as_ptr(), buf.len())
        };
        Ok(result)
    }

    pub(super) fn flush(&mut self) -> io::Result<()> {
        unsafe {
            stderr_native_flush();
        }
        Ok(())
    }
}
