extern crate num;

use num::integer::Integer;

const SUN: i32 = 0;
// const MON: i32 = 1;
const TUE: i32 = 2;
const WED: i32 = 3;
// const THU: i32 = 4;
const FRI: i32 = 5;
// const SAT: i32 = 6;

pub struct DoomsDate {
    day: i32,
    month: i32,
}

fn main() {
//    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

    println!("{}", doomsday(1978));
}

pub fn is_leap(year: i32) -> bool {
    ((year.mod_floor(& 4) == 0) && (year.mod_floor(& 100) != 0)) || (year.mod_floor(& 400) == 0)
}

pub fn generate_doomsdates(year: i32) -> Vec<DoomsDate> {
    vec![DoomsDate { month: 1, day: 3 }]
}

pub fn decade_year(year: i32) -> i32 {
    year % 100
}

pub fn century(year: i32) -> i32 {
    let long_century = year - decade_year(year);
    if long_century > 1000 {
        long_century / 100
    } else {
        0
    }
}

pub fn doomsday(year: i32) -> i32 {
    let multiples = decade_year(year) % 12;
    let diff = decade_year(year) - (multiples * 12);
    let div = (multiples).div_floor(& 4);
    let day = anchor_day(century(year));
    let result = multiples + diff + div + day;

    subtract_sevens(result)
}

pub fn subtract_sevens(num: i32) -> i32 {
    if num < 7 {
        num
    } else {
        subtract_sevens(num - 7)
    }
}

pub fn step_one(year: i32) -> i32 {
    year % 12
}

pub fn anchor_day(century: i32) -> i32 {
    match century {
        18 => FRI,
        19 => WED,
        20 => TUE,
        21 => SUN,
        _ => 0 // We should really be throwing an error here...
            // Consider wrapping the result in an Optional
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_one() {
        assert_eq!(6, step_one(78));
    }

    #[test]
    fn test_subtract_sevens() {
        assert_eq!(2, subtract_sevens(16));
    }

    #[test]
    fn test_decade_year() {
        assert_eq!(15, decade_year(2015));
        assert_eq!(78, decade_year(1978));
        assert_eq!(60, decade_year(1860));
    }

    #[test]
    fn test_century() {
        assert_eq!(19, century(1991));
        assert_eq!(20, century(2015));
        assert_eq!(21, century(2150));
        assert_eq!(18, century(1852));
    }

    #[test]
    fn test_generate_doomsdates() {

    }

    #[test]
    fn test_is_leap() {
        assert!(is_leap(2016));
        assert!(is_leap(1600));
        assert!(is_leap(2000));

        assert!(!is_leap(1700));
        assert!(!is_leap(1800));
        assert!(!is_leap(1900));
    }
}
