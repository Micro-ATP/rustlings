# Options

Type Option represents an optional value: every Option is either Some and contains a value, or None, and does not.
Option types are very common in Rust code, as they have a number of uses:

- Initial values
- Return values for functions that are not defined over their entire input range (partial functions)
- Return value for otherwise reporting simple errors, where None is returned on error
- Optional struct fields
- Struct fields that can be loaned or "taken"
- Optional function arguments
- Nullable pointers
- Swapping things out of difficult situations

## Further Information

- [Option Enum Format](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)


# 选项

`Option` 类型表示一个可选的值：每个 `Option` 要么是 `Some` 并包含一个值，要么是 `None` 并不包含值。`Option` 类型在 Rust 代码中非常常见，因为它们有多种用途：

- 初始值
- 对于未定义在整个输入范围内的函数的返回值（部分函数）
- 用于报告简单错误的返回值，其中 `None` 表示错误
- 可选的结构体字段
- 可以借用或“取走”的结构体字段
- 可选的函数参数
- 可为空的指针
- 从困难情况下交换出东西

## 更多信息

- [Option 枚举格式](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Option 模块文档](https://doc.rust-lang.org/std/option/)
- [Option 枚举文档](https://doc.rust-lang.org/std/option/enum.Option.html)
- [`if let`](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [`while let`](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)