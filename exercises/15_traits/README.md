# Traits

A trait is a collection of methods.

Data types can implement traits. To do so, the methods making up the trait are defined for the data type. For example, the `String` data type implements the `From<&str>` trait. This allows a user to write `String::from("hello")`.

In this way, traits are somewhat similar to Java interfaces and C++ abstract classes.

Some additional common Rust traits include:

- `Clone` (the `clone` method)
- `Display` (which allows formatted display via `{}`)
- `Debug` (which allows formatted display via `{:?}`)

Because traits indicate shared behavior between data types, they are useful when writing generics.

## Further information

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

## 特质

特质（Trait）是一组方法的集合。

数据类型可以实现特质。为此，需要为该数据类型定义组成特质的方法。例如，`String` 数据类型实现了 `From<&str>` 特质。这允许用户编写 `String::from("hello")`。

这样，特质在某种程度上类似于 Java 的接口和 C++ 的抽象类。

一些常见的 Rust 特质包括：

- `Clone`（包含 `clone` 方法）
- `Display`（允许通过 `{}` 格式化显示）
- `Debug`（允许通过 `{:?}` 格式化显示）

由于特质表明了数据类型之间的共享行为，因此它们在编写泛型时非常有用。

## 更多信息

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)