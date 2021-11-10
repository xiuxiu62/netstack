use std::{
    io::{self, Write},
    net::TcpStream,
};

use crate::error::Result;

pub fn get<W>(writer: &mut W, host: &str, port: u16) -> Result<()>
where
    W: Write,
{
    let address = format!("{host}:{port}");
    let mut conn = TcpStream::connect(address)?;

    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n")?;

    conn.write_all(format!("Host: {host}").as_bytes())?;
    conn.write_all(b"\r\n\r\n")?;

    io::copy(&mut conn, writer)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str;

    use crate::error::Result;

    use super::get;

    #[test]
    fn get_works() -> Result<()> {
        let host = "www.rustinaction.com";
        let port = 80;

        let mut buf = vec![];
        get(&mut buf, host, port)?;

        let result = str::from_utf8(&buf)?;
        println!("{result}");

        Ok(())
    }
}
