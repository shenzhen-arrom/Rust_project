//默认泛型类型参数和运算符重载
//1.使用泛型类型参数时,可以为泛型指定一个默认的具体类型
//2.运算符重载是指在特定的情况下,自定义运算符行为的操作
// rust并不可以创建自定义运算符或者重载运算符
// 不过对于std::ops中列出的运算符和相应的trait,我们可以实现运算符相关trait来重载
use std::ops::Add;

#[derive(Debug,PartialEq)]
struct Point {
    x: i32;
    y: i32;
}

impl Add for Point {
    type Output = Point;
    fn add(self,other:Point) -> Point{
        Point{
            x:self.x+other.x,
            y:self.y+other.y,
        }
    }
}