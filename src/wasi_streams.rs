use std::io;

use bindings::streams;

use crate::error::{Error, Result};

pub struct InputStream(pub(crate) streams::InputStream);

impl InputStream {
    fn blocking_read(&self, len: usize) -> Result<Vec<u8>> {
        let (data, _eos) = bindings::streams::blocking_read(self.0, len as u64)?;
        if data.len() > len {
            return Err(Error::other("blocking_read returned too much data"));
        }
        Ok(data)
    }
}

impl io::Read for InputStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let data = self.blocking_read(buf.len())?;
        let n = data.len();
        buf[..n].copy_from_slice(&data);
        Ok(n)
    }
}

pub struct OutputStream(pub(crate) streams::OutputStream);

impl OutputStream {
    fn blocking_write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = streams::blocking_write(self.0, buf)?;
        Ok(n as usize)
    }
}

impl io::Write for OutputStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(self.blocking_write(buf)?)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
