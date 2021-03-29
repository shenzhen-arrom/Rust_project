use std::thread;
use std::sync::mpsc;

// 知识点:
// 1.发送者的send方法返回一个result<T,E>, 如果接收端已经被丢弃,将没有发送值的目标,此时发送会返回错误.
// 2.接受者的recv返回值也是一个result类型,当通道发送端关闭时,返回一个错误值
// 3.接受端这里使用的recv方法,会阻塞到有一个消息到来,我们也可用使用try_recv(),不会阻塞,会立即返回.
fn main(){

    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("xujie");
        //发送
        tx.send(val).unwrap();
    });
    
    let received = rx.recv().unwrap();

    println!("received = {}",received);
    
}