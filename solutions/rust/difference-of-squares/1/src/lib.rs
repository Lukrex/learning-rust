pub fn square_of_sum(n: u32) -> u32 {
    let sum = n * (1 + n) / 2;
    sum*sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut res = 0;
    for mut i in 1..=n {
        i *= i;
        res += i;
    }
    res
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
