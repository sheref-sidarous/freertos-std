use crate::io;

#[cfg(feature = "semihosted-stdio")]
mod semihosted;
#[cfg(feature = "semihosted-stdio")]
use semihosted::HostOutImpl as StdoutImpl;
#[cfg(feature = "semihosted-stdio")]
use semihosted::HostOutImpl as StderrImpl;
#[cfg(feature = "semihosted-stdio")]
use semihosted::NullStdin as StdinImpl;
#[cfg(feature = "semihosted-stdio")]
pub use semihosted::STDIN_BUF_SIZE;

#[cfg(not(feature = "semihosted-stdio"))]
mod native;
#[cfg(not(feature = "semihosted-stdio"))]
use native::StdoutImpl;
#[cfg(not(feature = "semihosted-stdio"))]
use native::StderrImpl;
#[cfg(not(feature = "semihosted-stdio"))]
use native::StdinImpl;
#[cfg(not(feature = "semihosted-stdio"))]
pub use native::STDIN_BUF_SIZE;



pub struct Stdin {
    inner : StdinImpl,
}
pub struct Stdout {
    inner : StdoutImpl,
}
pub struct Stderr {
    inner : StderrImpl,
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout {
            inner : StdoutImpl::new(),
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
            inner : StderrImpl::new(),
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

impl Stdin {
    pub const fn new() -> Stdin {
        Stdin {
            inner : StdinImpl::new(),
        }
    }
}

impl io::Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

pub fn is_ebadf(_err: &io::Error) -> bool {
    false
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stderr::new())
}
