// TODO: This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was instead of just returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Change
// the function signature and body to return `Result<String, String>` instead
// of `Option<String>`.

// TODO: 这个函数在你传递一个空字符串时会拒绝生成用于名牌的文本。
// 如果它能解释一下问题是什么，而不是仅仅返回 `None`，会更好。
// 幸运的是，Rust 有一个类似于 `Option` 的构造体可以用来表示错误情况。
// 将函数签名和主体更改为返回 `Result<String, String>`，而不是 `Option<String>`。

fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("Empty names aren't allowed".to_string())
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // You can optionally experiment here.
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
