pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut min_price = 10_000;

    for price in prices {
        if price < min_price {
            min_price = price;
        }

        let total_price = price - min_price;
        if total_price > max_profit {
            max_profit = total_price;
        }
    }

    max_profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_5() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let result = max_profit(prices);
        assert_eq!(result, 5);
    }

    #[test]
    fn should_return_0_for_descending_prices() {
        let prices = vec![7, 6, 4, 3, 1];
        let result = max_profit(prices);
        assert_eq!(result, 0);
    }

    #[test]
    fn should_return_0_for_single_day() {
        let prices = vec![7];
        let result = max_profit(prices);
        assert_eq!(result, 0);
    }
}
