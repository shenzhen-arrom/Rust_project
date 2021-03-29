use std::{io::{self,prelude::*,BufReader,Write}, net::ToSocketAddrs};
use std::net::TcpStream;
use std::str;





fn main() -> io::Result<()> {

   


    let mut stream=TcpStream::connect("127.0.0.1:8080")?;
    //发10次
    for _ in 0..10 {
        let mut input =String::new();
        io::stdin().read_line(&mut input).expect("failed to read");
        //将读取的内容写入流里面
        stream.write(input.as_bytes()).expect("failed to write");

        let mut reader =BufReader::new(&stream);

        let mut buffer:Vec<u8> = Vec::new();

        reader.read_until(b'\n', &mut buffer).expect("failed to read into buffer");

        //打印
        println!("read from server: {}",str::from_utf8(&buffer).unwrap());
        println!("")
    }
    Ok(())
}