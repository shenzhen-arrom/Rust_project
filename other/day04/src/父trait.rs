//4.父trait用于另一个trait中使用某trait的功能
//

use std::fmt;
trait OutPrint : fmt::Display { //要求实现display trait
    fn out_print(&self) {
        let output = self.to_string();
        println!("output:{}",output);
    }
}
struct Point{
    x: i32,
    y: i32,
}
impl OutPrint for Point{}
impl fmt::Display for Point {
    fn fmt(&self , f: &mut fmt::Formatter) ->fmt::Result {
        write!(f,"({},{})",self.x,self.y)
    } 
}

fn main(){

}