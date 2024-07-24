// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

/*

- **任务**：我们要构建一个小机器，形式是一个函数。作为输入，我们将提供一个字符串和命令的列表。这些命令决定了将对字符串应用什么操作。操作可以是：
  - 将字符串转换为大写
  - 去除字符串的前后空白
  - 根据指定次数将 `"bar"` 追加到字符串末尾

- **具体形式**：
  - 输入是一个包含2元组的向量，第一个元素是字符串，第二个元素是命令。
  - 输出元素是一个字符串的向量。
*/


enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function.
    // pub fn transformer(input: ???) -> ??? { ??? }
    pub fn transformer(input:Vec::<(String, Command)>) -> Vec::<String> {
        let mut output = Vec::<String>::new();
        for (s, c) in input {
            match c {
                Command::Uppercase => output.push(s.to_uppercase()),
                Command::Trim => output.push(s.trim().to_string()),
                Command::Append(n) => output.push(s + &"bar".repeat(n)),
            }
        }
        output
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
