unsafe fn dagerous() {
    println!(" do something denger");
}
fn foo() {
    let mut  num =5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!(" r1 :{}",r1);
        println!(" r2 :{}",r2);
    }
}

fn main(){
    unsafe{
        dagerous();
    }
    foo();
}
