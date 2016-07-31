extern crate byteorder;

#[derive(Debug)]
pub enum Amf0Error {
    ByteOrder(std::io::Error)
}
