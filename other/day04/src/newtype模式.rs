//newtype模式用以在外部类型上实现外部的trait
//孤儿规则(orphan rule):只要trait或类型对于当前crate是本地的话,就可以在此类型上实现trait
//一个绕开这个限制的方法是使用newtype模式(newytpe pattern)
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper{
    fn fmt(&self ,f:&mut fmt::Formatter) ->fmt::Result{
        write!(f,"({})",self.0.join("."))
    }
}

fn main(){
    let w = Wrapper(vec![String::from("hello"),String::from("world")]);
    println!("w = {}",w);
}

