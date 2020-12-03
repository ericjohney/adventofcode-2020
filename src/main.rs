fn main() {
    let input = vec![10, 9, 8, 7, 5];
    let maybe_profit = max_profit(&input);
    if maybe_profit.is_some() {
        let profit = maybe_profit.unwrap();
        input.first();
        println!("profit = {}, {}", profit.0, profit.1);
    } else {
        println!("MAX PROFIT NOT FOUND");
    }
}

fn max_profit(prices: &Vec<u64>) -> Option<(u64, u64)> {
    let mut min_index = 0u64;
    let mut min_price = prices.first()?;
    let mut max_price = min_price;
    let mut profit: Option<(u64, u64)> = None;

    for (i, price) in prices.iter().enumerate() {
        if price < min_price {
            min_price = price;
            max_price = price;
            min_index = i as u64;
        }
        if price > max_price {
            max_price = price;
            profit = Some((min_index, i as u64));
        }
    }

    println!("max profit {}", max_price - min_price);
    return profit;
}
