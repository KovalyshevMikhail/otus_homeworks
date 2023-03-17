use std::io::{Read, Write, Result, ErrorKind, Error};

pub mod tcp_entity;
pub mod tcp_command;

pub const HOME_PORT: u32 = 53_000;

trait TcpHomeConnect {

}

pub fn send_string<Data: AsRef<str>, Writer: Write>(data: Data, writer: &mut Writer) -> Result<()> {
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes)?;
    writer.write_all(bytes)?;
    Ok(())
}

pub fn recv_string<Reader: Read>(mut reader: Reader) -> Result<String> {
    println!("read count bytes");
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    let len = u32::from_be_bytes(buf);

    println!("read message");
    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|_| Error::from(ErrorKind::Unsupported))
}

