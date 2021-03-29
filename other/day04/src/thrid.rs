//实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，
//比如圆形，三角形，正方形，需要用到泛型和泛型约束，可以上传代码片段，或者代码的链接。
use std::f32::consts::PI;

fn main() {
    let shape = ShapeType::Circle(4.0);
    notify(&shape)   
}
enum ShapeType {
    Circle(f32),//圆
    Triangle(f32,f32),//三角形
    Square(f32),//正方形
}

pub trait CalAreaData {
    //计算面积
    fn calArea(&self) ->f32;
}

impl CalAreaData for ShapeType{

    fn calArea(&self) ->f32{
         match &self {
            ShapeType::Circle(radius) =>  radius*radius*PI, //圆形
            ShapeType::Triangle(bottom,high) =>  bottom*high/2.0, //三角形
            ShapeType::Square(side) =>  side*side, //正方形
        }
    }
}
pub fn notify<T:CalAreaData> (item:&T){
    println!(" calArea:{}",item.calArea())
}