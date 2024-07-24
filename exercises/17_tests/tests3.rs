struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // 不要修改这个函数
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // 返回 `Result` 会更好。但我们要学习如何测试可能会恐慌的函数。
            panic!("Rectangle width and height can't be negative");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // 你可以在这里进行实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // 检查宽度
        assert_eq!(rect.height, 20); // 检查高度
    }

    #[test]
    fn negative_width() {
        let result = std::panic::catch_unwind(|| {
            Rectangle::new(-10, 10);
        });
        assert!(result.is_err()); // 确保创建带有负宽度的矩形时发生了恐慌
    }

    #[test]
    fn negative_height() {
        let result = std::panic::catch_unwind(|| {
            Rectangle::new(10, -10);
        });
        assert!(result.is_err()); // 确保创建带有负高度的矩形时发生了恐慌
    }
}
