// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
       if let Some(word) = optional_target {
            assert_eq!(word,  target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}


// 在 simple_option 测试中：
// 我们使用 if let Some(word) = optional_target 来解构 Some 类型。
// 这样可以直接访问 Some 中的值，而不需要额外的匹配。
// 在 layered_option 测试中：
// 我们使用 while let Some(Some(integer)) = optional_integers.pop() 来处理双层的 Option。
// optional_integers.pop() 返回 Option<Option<i8>>，因为 pop 本身返回一个 Option，而向量中的元素也是 Option<i8>。