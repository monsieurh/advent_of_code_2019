use std::env::args;
fn compute_fuel(mass: u32) -> u32 {
    (((mass as f32) / 3.0).floor() - 2.0) as u32
}

fn main() {
    let first_argument: u32 = args().nth(1).expect("Missing argument").parse().unwrap();
    println!("{}", compute_fuel(first_argument));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_test() {
        assert_eq!(compute_fuel(12), 2);
        assert_eq!(compute_fuel(14), 2);
        assert_eq!(compute_fuel(1969), 654);
        assert_eq!(compute_fuel(100756), 33583);
    }
}
