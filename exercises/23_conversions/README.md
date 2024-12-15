# Type conversions

Rust offers a multitude of ways to convert a value of a given type into another type.

The simplest form of type conversion is a type cast expression. It is denoted with the binary operator `as`. For instance, `println!("{}", 1 + 1.0);` would not compile, since `1` is an integer while `1.0` is a float. However, `println!("{}", 1 as f32 + 1.0)` should compile. The exercise [`using_as`](using_as.rs) tries to cover this.

Rust also offers traits that facilitate type conversions upon implementation. These traits can be found under the [`convert`](https://doc.rust-lang.org/std/convert/index.html) module.
The traits are the following:

- `From` and `Into` covered in [`from_into`](from_into.rs)
- `TryFrom` and `TryInto` covered in [`try_from_into`](try_from_into.rs)
- `AsRef` and `AsMut` covered in [`as_ref_mut`](as_ref_mut.rs)

Furthermore, the `std::str` module offers a trait called [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) which helps with converting strings into target types via the `parse` method on strings. If properly implemented for a given type `Person`, then `let p: Person = "Mark,20".parse().unwrap()` should both compile and run without panicking.

These should be the main ways ***within the standard library*** to convert data into your desired types.

## Further information

These are not directly covered in the book, but the standard library has a great documentation for it.

- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)


# 类型转换

Rust 提供了多种方法将一种类型的值转换为另一种类型。

最简单的类型转换形式是类型转换表达式。它用二元运算符 `as` 表示。例如，`println!("{}", 1 + 1.0);` 无法编译，因为 `1` 是整数，而 `1.0` 是浮点数。然而，`println!("{}", 1 as f32 + 1.0)` 应该能够编译。练习 [`using_as`](using_as.rs) 尝试覆盖这个内容。

Rust 还提供了在实现时简化类型转换的特性。这些特性可以在 [`convert`](https://doc.rust-lang.org/std/convert/index.html) 模块下找到。相关的特性包括：

- `From` 和 `Into`，在 [`from_into`](from_into.rs) 中讨论
- `TryFrom` 和 `TryInto`，在 [`try_from_into`](try_from_into.rs) 中讨论
- `AsRef` 和 `AsMut`，在 [`as_ref_mut`](as_ref_mut.rs) 中讨论

此外，`std::str` 模块提供了一个名为 [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) 的特性，它通过 `parse` 方法将字符串转换为目标类型。如果为给定类型 `Person` 正确实现，则 `let p: Person = "Mark,20".parse().unwrap()` 应该能够编译并在不发生 panic 的情况下运行。

这些是 ***标准库*** 中将数据转换为所需类型的主要方法。

## 更多信息

这些内容在书中没有直接涵盖，但标准库有很好的文档。

- [转换](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` 特性](https://doc.rust-lang.org/std/str/trait.FromStr.html)