#![allow(dead_code)]
#![feature(format_args_capture)]

mod error;
pub mod http;

pub use error::{NetstackError, Result};
