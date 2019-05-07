#[derive(Debug)]
pub enum Error {
    ParseError(::std::net::AddrParseError),
    IoError(::std::io::Error),
}
