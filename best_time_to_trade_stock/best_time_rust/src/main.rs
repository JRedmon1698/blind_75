fn main() {
    println!("{}", max_profit(vec![7, 1, 5, 3, 6, 4]));
    println!("{}", max_profit(vec![7, 6, 4, 3, 1]));
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy = prices[0];
    let mut profit = 0i32;

    for price in prices {
        if price < buy {
            buy = price;
        } else if price - buy > profit {
            profit = price - buy;
        }
    }

    profit
}
