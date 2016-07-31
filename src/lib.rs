extern crate byteorder;

use byteorder::Error;

#[derive(Debug)]
pub enum Amf0Error {
    ByteOrder(Error)
}