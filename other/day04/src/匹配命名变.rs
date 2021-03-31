fn main(){

    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("50"),
        Some(y) => println!("value = {} ", y),
        _=> println!("other"),
    };

    println!("x = {:?} , y = {:?}",x,y);

}