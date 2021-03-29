#[derive(Debug)]
enum List{
    Cons(i32,RefCell<Weak<List>>),
    Nil,
}

use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;
use crate::List::{Cons,Nil};

impl List{
    fn tail(&self) ->Option<&RefCell<Weak<List>>> {
        match self {
            Cons(_,item) =>Some(item),
            Nil => None,
        }
    }
}

fn main() {
   
    let a = Rc::new(Cons(5,RefCell::new(Weak::new())));
    println!("1 , a strong count ={},weak count = {}",Rc::strong_count(&a),Rc::weak_count(&a));
    println!("1 , a tail = {:?}",a.tail());

    let b = Rc::new(Cons(10,RefCell::new(Weak::new())));
    if let Some(link) = b.tail() {
        //b指向a
        *link.borrow_mut() = Rc::downgrade(&a);
    }
    
    println!("2 , a strong count = {},weak count = {}",Rc::strong_count(&a),Rc::weak_count(&a));
    println!("2 , b strong count {}, weak count = {}",Rc::strong_count(&b),Rc::weak_count(&b));
    println!("2 , b tail = {:?}",b.tail());
    
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::downgrade(&b);
    }



}





