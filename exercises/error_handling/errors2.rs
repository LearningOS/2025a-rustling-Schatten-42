// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the tokens. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `parse` function on a string that is not a number, that function will
// return a `ParseIntError`, and in that case, we want to immediately return
// that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.
/*
假设我们正在编写一款游戏，您可以在其中使用代币购买物品。所有物品成本
5 个代币，每当您购买物品时，都需要支付 1 的手续费
令 牌。游戏玩家将输入他们想要购买的物品数量，以及
“total_cost”函数将计算代币的总成本。因为
玩家输入数量，但是，我们得到它作为字符串——他们
可能输入了任何东西，而不仅仅是数字！
目前，此函数根本没有处理错误情况（也不是
正确处理成功案例）。我们想做的是：如果我们调用
非数字字符串上的“parse”函数，该函数将
返回 'ParseIntError'，在这种情况下，我们希望立即返回
我们的函数中的错误，而不是尝试乘法和相加。
至少有两种方法可以实现这一点，它们都是正确的——但一种
短了很多！
*/



use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    // #1
    // let qty: i32 = item_quantity.parse::<i32>()?; // `?`作用：- 失败了 立即调用return返回 - 成功了，自动解包 Ok(i32) -> i32
    // Ok(qty * 5 + 1)
    
    // #2
   // 调用 .map() --> Err 会自动忽略.map()，透传出去
    item_quantity.parse::<i32>().map(|qty| qty * 5 + 1)

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
