use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn compute_fuel_for_mass(mass: i64) -> i64 {
    (((mass as f64) / 3.0).floor() - 2.0) as i64
}

fn compute_fuel_for_fuel(fuel_mass: i64) -> i64 {
    let mut fuel_sum: i64 = 0;
    let mut mass = fuel_mass;
    loop {
        mass = compute_fuel_for_mass(mass);
        if mass <= 0 {
            break;
        }
        fuel_sum += mass;
    }
    fuel_sum
}

fn main() -> Result<()> {
    let file_path = args().nth(1).expect("Missing argument");
    println!("Reading from file {}", file_path);

    let file = File::open(file_path)?;
    let fuelreq_per_module: Vec<i64> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .map(|mass| compute_fuel_for_mass(mass))
        .collect();

    let total_fuelreq_two: i64 = fuelreq_per_module
        .iter()
        .map(|fuel| fuel + compute_fuel_for_fuel(*fuel))
        .sum();

    let total_fuelreq: i64 = fuelreq_per_module.into_iter().sum();

    println!("Total fuelreq1: {}", total_fuelreq);
    println!("Total fuelreq2: {}", total_fuelreq_two);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_test() {
        //day one, first star
        assert_eq!(compute_fuel_for_mass(12), 2);
        assert_eq!(compute_fuel_for_mass(14), 2);
        assert_eq!(compute_fuel_for_mass(1969), 654);
        assert_eq!(compute_fuel_for_mass(100756), 33583);
    }

    #[test]
    fn updated_formula() {
        assert_eq!(compute_fuel_for_fuel(12), 2);
        assert_eq!(compute_fuel_for_fuel(1969), 966);
        assert_eq!(compute_fuel_for_fuel(100756), 50346);
    }
}
