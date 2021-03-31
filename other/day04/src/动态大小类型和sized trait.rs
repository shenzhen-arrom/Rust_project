//动态大小类型和sized trait
//动态大小类型(dynamically sized types) ,有时被称为DST或unsized types
//这些类型可以我们处理只有在运行时才知道大小的类型
//最典型的就是str
//动态大小类型的黄金规则:必须将动态大小类型的值置于某种指针之后,eg:Box<str>\Rc<str>
//另一个动态大小类型就是trait 
