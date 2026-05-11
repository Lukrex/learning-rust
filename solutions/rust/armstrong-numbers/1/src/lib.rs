pub fn is_armstrong_number(num: u32) -> bool {
    let length = num.to_string().len() as u32;
    let mut sum = 0;
    for d in num.to_string().chars() {
        sum += d.to_digit(10).expect("No digit").pow(length);
    }
    num == sum
}
