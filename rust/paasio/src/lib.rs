use std::{io::{Read, Result, Write}};

pub struct ReadStats<R> {
    wrapped: R,
    bytes_through: usize,
    operations: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self{wrapped, bytes_through: 0, operations: 0}
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.operations
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.operations += 1;
        match self.wrapped.read(buf) {
            Ok(size) => {
                self.bytes_through += size;
                Ok(size)
            }
            Err(e) => Err(e),
        }
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    bytes_through: usize,
    operations: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self{wrapped, bytes_through: 0, operations: 0}
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.operations
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.operations += 1;
        match self.wrapped.write(buf) {
            Ok(size) => {
                self.bytes_through += size;
                Ok(size)
            }
            Err(e) => Err(e),
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
