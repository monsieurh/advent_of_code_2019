use std::env::args;
fn has_sequence(s: &String, lenght: usize) -> bool {
    let digits: Vec<u64> = s.chars().map(|c| c as u64).collect();

    let mut count = 0;
    let mut last_digit: u64 = 10;
    for i in 0..digits.len() {
        if last_digit == digits[i] {
            count += 1;
        } else {
            if count == lenght {
                return true;
            }
            count = 1;
        }
        last_digit = digits[i];
    }
    if count == lenght {
        return true;
    }
    return false;
}

fn is_sorted(s: &String) -> bool {
    let digits: Vec<u64> = s.chars().map(|c| c as u64).collect();
    for i in 0..digits.len() - 1 {
        if digits[i] > digits[i + 1] {
            return false;
        }
    }
    true
}

fn is_valid_pwd(pin: u64) -> bool {
    let str_pin = pin.to_string();
    str_pin.len() == 6 && is_sorted(&str_pin) && has_sequence(&str_pin, 2)
}

fn main() {
    let min_range: u64 = args()
        .nth(1)
        .expect("Missing min_range")
        .parse()
        .expect("Invalid min_range");
    let max_range: u64 = args()
        .nth(2)
        .expect("Missing max_range")
        .parse()
        .expect("Invalid max_range");

    let count = (min_range..max_range).filter(|i| is_valid_pwd(*i)).count();

    println!(
        "Valid passwords in [{}..{}] range : {}",
        min_range, max_range, count
    );
}

mod tests {
    use super::*;
    #[test]
    fn sample_input1() {
        //111111 meets these criteria (double 11, never decreases).
        assert!(!is_valid_pwd(111111)); //no longer with second star
    }

    #[test]
    fn sample_input2() {
        //223450 does not meet these criteria (decreasing pair of digits 50).
        assert!(!is_valid_pwd(223450));
    }

    #[test]
    fn sample_input3() {
        //123789 does not meet these criteria (no double).
        assert!(!is_valid_pwd(123789));
    }

    #[test]
    fn sample_input4() {
        //112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
        assert!(is_valid_pwd(112233));
    }

    #[test]
    fn sample_input5() {
        //123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
        assert!(!is_valid_pwd(123444));
    }

    #[test]
    fn sample_input6() {
        //111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
        assert!(is_valid_pwd(111122));
    }
}
