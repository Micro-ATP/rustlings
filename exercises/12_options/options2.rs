fn main() {
    // You can optionally experiment here.
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn simple_option() {
//         let target = "rustlings";
//         let optional_target = Some(target);

//         // TODO: Make this an if-let statement whose value is `Some`.
//         word = optional_target {
//             assert_eq!(word, target);
//         }
//     }

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // 使用 `if let` 语句来处理 `Option` 类型的值
        if let Some(word) = optional_target {
            assert_eq!(word, target); // 当 `optional_target` 是 `Some(target)` 时，`word` 的值应该与 `target` 相等
        }
    }



// #[test]
//     fn layered_option() {
//         let range = 10;
//         let mut optional_integers: Vec<Option<i8>> = vec![None];

//         for i in 1..=range {
//             optional_integers.push(Some(i));
//         }

//         let mut cursor = range;

//         // TODO: Make this a while-let statement. Remember that `Vec::pop()`
//         // adds another layer of `Option`. You can do nested pattern matching
//         // in if-let and while-let statements.
//         integer = optional_integers.pop() {
//             assert_eq!(integer, cursor);
//             cursor -= 1;
//         }

//         assert_eq!(cursor, 0);
//     }

#[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // 使用 `while let` 语句来处理嵌套的 `Option`
        while let Some(integer) = optional_integers.pop().flatten() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }

}
