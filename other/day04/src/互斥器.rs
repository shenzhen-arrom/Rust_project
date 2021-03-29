use std::sync::Mutex;
// Mutex 智能指针,lock调用返回一个叫做mutexGuard的智能指针,内部提供来drop方法,实现来当utexGuard离开作用域自动释放锁
fn main(){

    let m = Mutex::new(5);

    {
        let mut num =m.lock().unwrap();
        *num = 6;
    } //离开作用域自动释放
    println!("m ={:?}",m);



}