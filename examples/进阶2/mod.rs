
use std::io::Result as IoResult;    //可以进行重命名
use std::{cmp::Ordering,io};    //指定嵌套的路径在一行中将多个带有相同前缀的项引入作用域
use std::io::{self, Write};
use std::collections::*;
mod sound


{
    
     pub   mod instrument  //允许层级结构
    {
     pub fn guitar()
    {

        println!("hello");
    }
    fn breathe()
    {

    }
    }
}

//路径 可以有两种形式：有点像电脑中的路径
    //绝对路径（absolute path）从 crate 根开始，以 crate 名或者字面值 crate 开头。
    //相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头
//绝对路径和相对路径都后跟一个或多个由双冒号（::）分割的标识符。
fn main()
{
    //绝对路径
    crate::sound::instrument::guitar();

    //相对路径
    sound::instrument::guitar();

    //super 相当于文件系统中的..
    super::breathe();
}

//私有性规则适用于结构体、枚举、函数和方法以及模块

//如果在结构体定义中使用 pub，可以使结构体公有。
//然而结构体的字段仍是私有的。可以在每一个字段的基准上选择其是否公有