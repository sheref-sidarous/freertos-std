use crate::io;
use crate::cell::OnceCell;
use cortex_m_semihosting::hio;
use core::ops::Deref;

pub struct Stdin;
pub struct Stdout {
    inner : HostOutImpl,
}
pub struct Stderr {
    inner : HostOutImpl,
}

struct HostOutImpl {
    host_stdout : Option<hio::HostStream>,
}

impl HostOutImpl {
    const fn new() -> HostOutImpl {
        HostOutImpl {
            host_stdout : None,
        }
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut host_stdout = match self.host_stdout {
            None => {
                let new_stream = match hio::hstdout() {
                    Ok(x) => x,
                    _ => {
                        return Err(io::Error::new(io::ErrorKind::Other, "semihosting creation failed"));
                    },
                };
                self.host_stdout = Some(new_stream);
                new_stream
            },
            Some(x) => x,
        };
        match host_stdout.write_all(buf) {
            Ok(()) => Ok(buf.len()),
            Err(()) => Err(io::Error::new(io::ErrorKind::Other, "semihosting write failed")),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

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
            inner : HostOutImpl::new(),
        }
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr {
            inner : HostOutImpl::new(),
        }
    }
}

impl io::Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}

pub fn panic_output() -> Option<Vec<u8>> {
    None
}
