use std::io::{self, Read, Write};
use std::net::TcpStream;

pub struct Connection {
    reader: TcpStream,
    writer: TcpStream,
}

#[derive(Debug)]
pub struct ReadHalf {
    stream: TcpStream,
}

#[derive(Debug)]
pub struct WriteHalf {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let reader = stream.try_clone()?;
        let writer = stream;

        Ok(Connection { reader, writer })
    }

    pub fn connect<A: std::net::ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Self::new(stream)
    }

    pub fn split(self) -> (ReadHalf, WriteHalf) {
        let reader = ReadHalf {
            stream: self.reader,
        };
        let writer = WriteHalf {
            stream: self.writer,
        };
        (reader, writer)
    }
}

impl ReadHalf {
    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.stream.read_exact(buf)
    }

    #[allow(dead_code)]
    pub fn shutdown(&self) -> io::Result<()> {
        self.stream.shutdown(std::net::Shutdown::Read)
    }
}

impl WriteHalf {
    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }

    pub fn shutdown(&self) -> io::Result<()> {
        self.stream.shutdown(std::net::Shutdown::Write)
    }
}

impl Read for ReadHalf {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read(buf)
    }
}

impl Write for WriteHalf {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush()
    }
}
