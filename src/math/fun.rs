/// Collatz conjecture for more information [here](https://en.wikipedia.org/wiki/Collatz_conjecture) .
pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => collatz(n / 2).map(|x| x + 1),
        n => match n.overflowing_mul(3) {
            (_, false) => match (n * 3).overflowing_add(1) {
                (_, false) => collatz(3 * n + 1).map(|x| x + 1),
                (_, true) => None,
            },
            (_, true) => None,
        },
    }
}

/// Armstrong number for more information [here](https://en.wikipedia.org/wiki/Narcissistic_number) .
pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let length = num_str.len() as u32;
    num == num_str
        .chars()
        .map(|digit| digit.to_digit(10).unwrap().pow(length))
        .sum()
}