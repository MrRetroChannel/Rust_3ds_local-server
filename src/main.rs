use std::net::TcpStream;
use std::net::TcpListener;
use std::io::prelude::*;
use std::fs::File;

fn to_bytes(name: &str) -> Vec<u8>
{
    let mut file = File::open(&name).expect("A");
    let meta = std::fs::metadata(&name).expect("B");
    let mut buf: Vec<u8> = vec![0; meta.len() as usize];
    file.read(&mut buf).expect("C");
    return buf;
}

fn handle(stream: &mut std::net::TcpStream, filename: &str) -> std::io::Result<()>
{
    let endpoints: [&str; 3] = ["GET /sos HTTP/1.1\r\n", "sas", "piss"];
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let bytes = to_bytes(filename);
    if buffer.starts_with(endpoints[0].as_bytes())
    {
        stream.write(b"HTTP/1.1 200 OK\n\nSos")?;
    }
    stream.write("HTTP/1.1 200 OK\nContent-Disposition: attachment; filename=\"123.cia\"\n".as_bytes()).unwrap();
    stream.write(format!("Content-Length: {}\n\n", bytes.len()).as_bytes()).unwrap();
    stream.write(&bytes).unwrap();
    stream.flush().unwrap();
    return Ok(());
}


fn main() -> std::io::Result<()>{
    let server = TcpListener::bind("0.0.0.0:8000").unwrap();
    let mut socket = TcpStream::connect("192.168.1.167:5000").unwrap();
    let url = "http://192.168.1.194:8000/";
    socket.write(&[(url.len() >> 24) as u8, (url.len() >> 16) as u8, (url.len() >> 8) as u8, url.len() as u8])?;
    socket.write(url.as_bytes())?;
    for str in server.incoming()
    {
        let mut stream = str.unwrap();
        handle(&mut stream, "C:\\Users\\MrRetro\\Desktop\\1.cia")?;
    }
    
    return Ok(());
}