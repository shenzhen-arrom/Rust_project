// 忽略模式中的值
// fn foo(_: i32 , y:i32){
//     println!("y = {}",y);
// }

// fn main(){
//     foo(1,2);
//     let numbers = (1,2,3,4);
//     match numbers {
//         (one,_,three,_) =>{
//             println!("one:{} , three: {}",one.three);
//         },
//     }
// }
fn main() {

    // let _x= 5; // _表示忽略变量
    // let _y= 5;

    // let s = Some(String::from("hello"));
    // if let Some(_c) =s { // 忽略变量C
    //     println!(" found a string");
    // }
    // println!(" s : {:?}",s);

    let h = Some(String::from("xujie"));
    if let Some(_) = h {
        println!("found a string");
    }
    println!(" h : {:?} ",h);

    let numbers = (1,2,3,4,5,6,7);
    match numbers {
        (first , .., last) =>{
            println!("first:{} , last :{}", first,last);
        },
    };
    




}



