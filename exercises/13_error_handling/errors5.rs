// This exercise is an altered version of the `errors4` exercise. It uses some
// concepts that we won't get to until later in the course, like `Box` and the
// `From` trait. It's not important to understand them in detail right now, but
// you can read ahead if you like. For now, think of the `Box<dyn ???>` type as
// an "I want anything that does ???" type.
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, The `Box` is declared as of type `Box<dyn Trait>` where
// `Trait` is the trait the compiler looks for on any value used in that
// context. For this exercise, that context is the potential errors which
// can be returned in a `Result`.

// 本练习是 `errors4` 练习的修改版本。
// 它使用了一些我们直到课程后期才会涉及的概念，比如 `Box` 和 `From` trait。
// 现在详细理解这些概念并不重要，但如果你愿意，可以提前阅读。
// 暂时可以把 `Box<dyn ???>` 类型理解为“我想要任何实现了 ??? 的类型”的类型。
// 简而言之，这种 Box 的特定用例是当你想要拥有一个值并且只关心它是否实现了某个特定的 trait 时。
// 为此，`Box` 被声明为 `Box<dyn Trait>` 类型，其中 `Trait` 是编译器在该上下文中使用的任何值上寻找的 trait。
// 对于本练习，这个上下文是 `Result` 中可能返回的潜在错误。

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// This is required so that `CreationError` can implement `Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// TODO: Add the correct return type `Result<(), Box<dyn ???>>`. What can we
// use to describe both errors? Is there a trait which both errors implement?
// fn main() {
//     let pretend_user_input = "42";
//     let x: i64 = pretend_user_input.parse()?;
//     println!("output={:?}", PositiveNonzeroInteger::new(x)?);
//     Ok(())
// }

fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}