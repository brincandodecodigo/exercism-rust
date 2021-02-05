pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let number_of_digits: u32 = num_str.len() as u32;

    let sum: u32 = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|n| n.pow(number_of_digits))
        .sum();

    sum == num
}
