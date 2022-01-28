//if let 语法让我们以一种不那么冗长的方式结合 if 和 let
//来处理只匹配一个模式的值而忽略其他模式的情况
//用来匹配只有一个值的情况
let some_vale=Some(0u8);

match some_vale
{
    Some(3)=>println!("three"),
    _=>(),      
}

//if let 获取通过等号分隔的一个模式和一个表达式
if let Some(3)=some_vale
{
    println!("three")
}
else {
    None;
}

