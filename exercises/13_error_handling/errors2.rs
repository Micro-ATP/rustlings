// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the items. Since
// the player typed in the quantity, we get it as a string. They might have
// typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all. What we want
// to do is: If we call the `total_cost` function on a string that is not a
// number, that function will return a `ParseIntError`. In that case, we want to
// immediately return that error from our function and not try to multiply and
// add.
//
// There are at least two ways to implement this that are both correct. But one
// is a lot shorter!

// 假设我们正在编写一个游戏，玩家可以用代币购买物品。
// 所有物品的价格是 5 个代币，每次购买物品时还会有 1 个代币的处理费用。
// 玩家会输入他们想购买的物品数量，`total_cost` 函数将计算这些物品的总费用。
// 由于玩家输入的是字符串，他们可能输入了任何内容，而不仅仅是数字！
// 目前，这个函数完全没有处理错误情况。
// 我们希望做的是：如果我们在一个不是数字的字符串上调用 `total_cost` 函数，该函数将返回一个 `ParseIntError`。
// 在这种情况下，我们希望立即从函数中返回该错误，而不是尝试进行乘法和加法操作。
// 有至少两种方法来实现这一点，这两种方法都是正确的。但其中一种方法要短得多！

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: Handle the error case as described above.
    // let qty = item_quantity.parse::<i32>();
    let qty = item_quantity.parse::<i32>()?;
    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
