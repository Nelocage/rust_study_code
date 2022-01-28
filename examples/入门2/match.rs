enum coin
{
    penny,
    nickel,
    dime,
    quarter(usstate),
}

//绑定值的模式
#[derive(Debug)]
enum usstate
{
    alabama,
    alaska,
}


fn value_in_cents(coin:coin)->u32
{
    match coin {
        coin::penny=>{
            println!("luckey");
            1
        },     //=> 运算符将模式和将要运行的代码分开
        coin::nickel=>5,
        coin::dime=>10,
        coin::quarter(state)=>
        {
            println!("state quatter from {:?}",state);
            25
        },
    }
}


fn plus_one(x:Option<i32>)->Option<i32>
{
    match x
    {
        None=>None,     //match中的语句使用逗号分隔，而不是分号
        Some(i)=>Some(i+1),
    }
}

//match 一个枚举，绑定其中的值到一个变量，接着根据其值执行代码

// _ 通配符
let  some_u8_value=0u8;
match some_u8_value
{
    1=>println!("one"),
    _=>(),      //匹配之前没有指定的可能的值，
}
