use std::net::{TcpListener,TcpStream};
use std::thread;
use std::time;
use std::io::{self,Read,Write};


fn handle_client(mut stream:TcpStream) ->io::Result<()>{
    let mut buf=[0;512];
    for _ in 0..1000{
        // 从流里面读取内容
        let bytes_read=stream.read(&mut buf)?;
        if  bytes_read == 0{//结束
             return Ok(());
        }
        //将读取的内容写回去
        stream.write(&buf[..bytes_read])?;
        //休眠1s
        thread::sleep(time::Duration::from_secs(1))
    }
    Ok(())
}

fn main() ->io::Result<()>{
    let listener =TcpListener::bind("127.0.0.1:8080")?;
    //线程容器
    let mut thread_ver:Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming(){
        let stream=stream.expect("failed");
        let handle=thread::spawn(move||{
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error));
        });
        thread_ver.push(handle);
    }

    for handle in thread_ver{
        handle.join().unwrap();
    }

    
    Ok(())
}
