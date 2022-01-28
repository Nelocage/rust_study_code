
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

//哈希 map 是同质的：所有的键必须是相同类型，值也必须都是相同类型。

use std::collections::HashMap;
let teams=vec![String::from("bule"),String::from("yellow")];
let initial_scores=vec![10,50];

let scores:HashMap<_,_>=teams.iter().zip(initial_scores.iter()).collect();

//对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。
//对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// 这里 field_name 和 field_value 不再有效，
// 尝试使用它们看看会出现什么编译错误！

//当 insert 调用将 field_name 和 field_value 移动到哈希 map 中后，将不能使用这两个绑定。

//访问哈希map中的值
let team_name=String::from("Blue");
let score=scores.get(&team_name);//get 返回的是option<T>

//遍历每一个键值对
for(key,value) in &scores{
    println!("{}:{}",key,value);
}

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

//这种插入只对没有值的键有用，or_insert方法，在键对应的值存在时就返回这个值得entry，
//若不存在则将参数，作为新值插入并返回修改过的entry 
//第二个哈希调用不会改变哈希map，因为蓝色已经有了十，不会再被覆盖


//根据旧值更新一个值
let text= "hello world wondoerful world";
let mut map=HashMap::new();

for word in text.split_whitespace()
{
    let count =map.entry(word).or_insert(0);
    *count+=1;
}
println!("{:?}",map)


//计数一些文本中每一个单词分别出现了多少次。我们使用哈希 map 以单词作为键并递增其值来记录我们遇到过几次这个单词。
//如果是第一次看到某个单词，就插入值 0。

//or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）。
//这里我们将这个可变引用储存在 count 变量中，所以为了赋值必须首先使用星号（*）解引用 count。
//这个可变引用在 for 循环的结尾离开作用域，这样所有这些改变都是安全的并符合借用规则。