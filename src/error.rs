pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error{
    IO(std::io::Error),
    SmtpStatus(super::StatusCodes, String),
    TLS(native_tls::Error),
    Handshake(native_tls::HandshakeError<std::net::TcpStream>),
    OpenFile(std::io::Error),
    TLSStreamNotFound,
    TCPStreamNotFound,
    UTF8Convert,
    EmailRejectedByForwardSever,
}