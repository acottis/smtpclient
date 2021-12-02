//! Wrapper around stream read and write that handle TCP and then TLS when it starts
//! 
use std::net::TcpStream;
use native_tls::{TlsConnector, TlsStream};
use std::io::{BufReader, BufRead, Write};
use std::net::ToSocketAddrs;
use crate::error::{Result, Error};

static CRLF: &str = "\r\n";

/// Struct for managing the reading and writing from TLS and TCP streams in a way that abstracts from the rest of the code
///
#[derive(Debug)] 
pub struct Stream{
    tcp_stream: TcpStream,
    tls_stream: Option<TlsStream<TcpStream>>,
}
impl Stream{
    /// Creates a stream object from a TCP Stream
    /// 
    pub fn new(host: &String, port: &String) -> Self {
        let sock = format!("{}:{}", host, port).to_socket_addrs().unwrap().next().unwrap();
        let tcp_stream = TcpStream::connect(&sock).expect("Cannot start TCP Connection");
        tcp_stream.set_read_timeout(Some(std::time::Duration::from_secs(5))).unwrap();
        tcp_stream.set_write_timeout(Some(std::time::Duration::from_secs(5))).unwrap();
        Self {
            tcp_stream,
            tls_stream: None,
        }
    }
    /// Shuts down the TCP Stream
    /// 
    pub fn shutdown(&self) -> Result<()>{
        self.tcp_stream.shutdown(std::net::Shutdown::Both).map_err(Error::IO)
    }
    /// Returns Peer Address
    /// 
    pub fn peer_addr(&self) -> std::net::SocketAddr {
        self.tcp_stream.peer_addr().expect("Could not get peer IP Address")
    }
    /// This function reads a TCP stream until a CLRF `[13, 10]` is sent then collects into a [Vec]
    /// 
    pub fn read(&mut self) -> Result<Vec<u8>>  {  
        if self.tls_stream.is_some(){
            let stream = self.tls_stream.as_mut().unwrap();
            let mut reader = BufReader::new(stream);
            let mut data: Vec<u8> = vec![];
            let now = std::time::SystemTime::now();
            loop{
                let buffer = reader.fill_buf();  
                match buffer {
                    Ok(bytes) => {
                        let length = bytes.len();
                        data.extend_from_slice(bytes); 
                        reader.consume(length);
                        // Okay checks for CLFR if more than one byte is in buffer
                        if (data.len() > 1) && (&data[data.len()-2..] == [13, 10]){
                            break;
                        }
                    },
                    _ => {}
                }
                if now.elapsed().unwrap() > std::time::Duration::from_secs(60) {
                    return Err(Error::TCPReadTimeout)
                }      
            }
            print!("C: {}", String::from_utf8_lossy(&data));
            Ok(data)
        }else{
            let stream = &self.tcp_stream;
            let mut reader = BufReader::new(stream);
            let mut data: Vec<u8> = vec![];
            let now = std::time::SystemTime::now();
            loop{
                let buffer = reader.fill_buf();  
                match buffer {
                    Ok(bytes) => {
                        let length = bytes.len();
                        data.extend_from_slice(bytes); 
                        reader.consume(length);
                        // Okay checks for CLFR if more than one byte is in buffer
                        if (data.len() > 1) && (&data[data.len()-2..] == [13, 10]){
                            break;
                        }
                    },
                    _ => {}
                }
                if now.elapsed().unwrap() > std::time::Duration::from_secs(60) {
                    return Err(Error::TCPReadTimeout)
                }      
            }
            print!("C: {}", String::from_utf8_lossy(&data));
            Ok(data)
        }
    }
    /// Wrapper around writing to TCP stream, handles the no whitespace requirement of the HELO response
    /// 
    pub fn write(&mut self, msg: String) -> Result<()> {  
        if self.tls_stream.is_some(){
            self.tls_stream.as_mut().ok_or(Error::TLSStreamNotFound)?.write_all((msg+CRLF).as_bytes()).map_err(Error::IO)?;
        } else{
            self.tcp_stream.write_all((msg+CRLF).as_bytes()).map_err(Error::IO)?;
        }
        Ok(())
    }
    /// Internal use, uses external crate native_tls to start a TLS session inside our existing TCP stream and stores in `self.tls_stream`
    /// 
    pub fn start_tls(&mut self, host: &str) -> Result<()> {
        let tls_connector = TlsConnector::new().map_err(Error::TLS)?;
        let stream =  self.tcp_stream.try_clone().unwrap();
        self.tls_stream = Some(tls_connector.connect(host, stream).map_err(Error::Handshake)?);
        Ok(())
    }
}