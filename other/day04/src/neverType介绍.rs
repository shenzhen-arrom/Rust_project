// 从不返回的 never type
// rust 有一个叫做!的特殊类型.在类型理论术语中,它被称为empty type,因为它没有值
//我们更倾向于称为never type .在函数不返回的时候充当返回值
fn bar() ->!{
    loop{}
}