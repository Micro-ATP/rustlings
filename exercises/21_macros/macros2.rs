// fn main() {
//     my_macro!();
// }

// // TODO: Fix the compiler error by moving the whole definition of this macro.
// macro_rules! my_macro {
//     () => {
//         println!("Check out my macro!");
//     };
// }

// 将宏定义移到 `main` 函数之前
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
