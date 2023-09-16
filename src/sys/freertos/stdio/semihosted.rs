use crate::io;
use cortex_m_semihosting::hio;

pub const STDIN_BUF_SIZE: usize = 0;

pub(super) struct HostOutImpl {
    host_stdout : Option<hio::HostStream>,
}

impl HostOutImpl {
    pub(super) const fn new() -> HostOutImpl {
        HostOutImpl {
            host_stdout : None,
        }
    }

    pub(super) fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
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

    pub(super) fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub(super) struct NullStdin;

impl NullStdin {
    pub(super) const fn new() -> NullStdin {
        NullStdin
    }

    pub(super) fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
}
