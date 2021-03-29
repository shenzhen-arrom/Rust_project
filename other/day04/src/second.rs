//实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，
//返回类型为Option，溢出时返回None，可以上传代码片段，或者代码的链接；
use std::u32;

fn main() {
  let numbers:Vec<u32> = vec![42, 93, 18];
  let sum=add(&numbers).unwrap();
  println!("sum =:{}",sum);
     
}
fn add(numbers:&[u32]) ->Option<u32>{
    let mut result:u32= 0;
    let max =u32::MAX;
    for x in numbers.iter() {
        result += x  
    }
    if (result>max) {
        None
    }else{
        Some(result)
    }
    
}