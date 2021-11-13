#![feature(format_args_capture)]

use std::str;

use lib_netstack::{http, Result};

fn main() -> Result<()> {
    let host = "www.rustinaction.com";
    let port = 80;

    let mut buf = vec![];
    http::get(&mut buf, host, port)?;

    let result = str::from_utf8(&buf)?;
    let body = result.split("\r\n\r\n").last().unwrap_or("").trim();
    println!("{body}");

    Ok(())
}
