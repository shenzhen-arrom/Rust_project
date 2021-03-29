use std::thread;
use std::time::Duration;

fn main(){
    
    //必包函数
    let handle=thread::spawn(||{
        for i in 1..10{
            println!("number {} in spawn thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

}