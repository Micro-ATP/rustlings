# Lifetimes

Lifetimes tell the compiler how to check whether references live long
enough to be valid in any given situation. For example lifetimes say
"make sure parameter 'a' lives as long as parameter 'b' so that the return
value is valid".

They are only necessary on borrows, i.e. references,
since copied parameters or moves are owned in their scope and cannot
be referenced outside. Lifetimes mean that calling code of e.g. functions
can be checked to make sure their arguments are valid. Lifetimes are
restrictive of their callers.

If you'd like to learn more about lifetime annotations, the
[lifetimekata](https://tfpk.github.io/lifetimekata/) project
has a similar style of exercises to Rustlings, but is all about
learning to write lifetime annotations.

## Further information

- [Lifetimes (in Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)


# 生命周期

生命周期告诉编译器如何检查引用是否在任何给定情况下都足够长以保持有效。例如，生命周期会指定“确保参数 `a` 的生命周期和参数 `b` 一样长，以便返回值有效”。

生命周期只在借用（即引用）上是必要的，因为复制的参数或移动的值在其作用域内拥有所有权，不能在外部被引用。生命周期的作用是检查例如函数的调用代码，确保其参数是有效的。生命周期限制了其调用者。

如果你想了解更多关于生命周期注解的内容，[lifetimekata](https://tfpk.github.io/lifetimekata/) 项目提供了类似 Rustlings 的练习风格，但完全专注于学习如何编写生命周期注解。

## 更多信息

- [生命周期（Rust By Example）](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [使用生命周期验证引用](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)