// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}

// ref 用于在模式匹配中创建引用而不是移动值
// 当你需要在 match 后继续使用匹配的值时，使用 ref 是很好的选择
// 对于不实现 Copy trait 的类型尤其有用
// 可以和 mut 结合使用来创建可变引用