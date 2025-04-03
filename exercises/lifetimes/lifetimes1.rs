// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.


// 错误原因：
// 函数返回一个字符串引用（&str），但编译器无法确定这个返回的引用的生命周期
// 返回值可能来自 x 或 y，但编译器需要知道返回的引用与输入参数的生命周期关系
// Rust 需要通过生命周期标注来确保返回的引用在使用时一定是有效的

// 为什么需要生命周期标注：
// 防止悬垂引用（dangling references）
// 确保返回的引用不会比输入参数的引用活得更久
// 帮助编译器进行借用检查

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
