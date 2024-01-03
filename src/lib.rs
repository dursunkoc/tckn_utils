//! # TCKN UTILS
//! 
//! `tckn_utils` is a collection of utilities for generating dummy TCKN values and
//! validating against the tckn rules.

use rand::seq::IteratorRandom;
use std::str;
const ALL_DIGITS: &str = "0123456789";
const NON_ZERO_DIGITS: &str = "123456789";

/// Generates a valid random tckn value
/// # Examples
/// 
/// ```
/// let generated = tckn_utils::generate();
/// let actual = tckn_utils::validate(&generated);
/// assert_eq!(actual, true);
/// ```

pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    let first = NON_ZERO_DIGITS
        .chars()
        .choose(&mut rng)
        .map(|c| c.to_string().parse::<u16>().unwrap())
        .unwrap();

    let mut evens = ALL_DIGITS
        .chars()
        .choose_multiple(&mut rng, 4)
        .iter()
        .map(|c| c.to_string().parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    let mut odds = ALL_DIGITS
        .chars()
        .choose_multiple(&mut rng, 4)
        .iter()
        .map(|c| c.to_string().parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    odds.insert(0, first);

    let tenth = generate_tenth(&odds, &evens);
    evens.push(tenth);

    let eleventh = generate_eleventh(&odds, &evens);

    let mut all = odds
        .iter()
        .zip(&evens)
        .flat_map(|(a, b)| vec![a, b])
        .collect::<Vec<&u16>>();
    all.push(&eleventh);

    all.iter().map(|d| d.to_string()).collect::<String>()
}

fn generate_tenth(odds: &Vec<u16>, evens: &Vec<u16>) -> u16 {
    let odds_sum_times_7 = odds.iter().sum::<u16>() * 7;
    let evens_sum = evens.iter().sum::<u16>();
    let diff = odds_sum_times_7 - evens_sum;
    diff % 10
}

fn generate_eleventh(odds: &Vec<u16>, evens: &Vec<u16>) -> u16 {
    let odds_sum = odds.iter().sum::<u16>();
    let evens_sum = evens.iter().sum::<u16>();
    let sum = odds_sum + evens_sum;
    sum % 10
}

/// Validates if the given &str represents a valid tckn
/// 
/// # Examples
/// 
/// ```
/// let in_valid = "12345678951";
/// let actual = tckn_utils::validate(in_valid);
/// assert_eq!(actual, false);
/// ```
/// 
/// ```
/// let valid = "12345678950";
/// let actual = tckn_utils::validate(valid);
/// assert_eq!(actual, true);
/// ```
pub fn validate(tckn: &str) -> bool {
    if tckn.len() != 11 {
        return false;
    }
    if contains_non_digital(tckn) {
        return false;
    }
    if tckn.starts_with("0") {
        return false;
    }
    if !tenth_digit_check(tckn) {
        return false;
    }
    if !eleventh_digit_check(tckn) {
        return false;
    }

    true
}

fn contains_non_digital(tckn: &str) -> bool {
    tckn.as_bytes().iter().any(|b| !b.is_ascii_digit())
}

fn tenth_digit_check(tckn: &str) -> bool {
    let odds_sum_times_7: u8 = tckn
        .bytes()
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| (i + 1) % 2 == 1 && i < 10)
        .map(|(_, b)| std::str::from_utf8(&vec![b]).unwrap().to_string())
        .map(|s| s.parse::<u8>().unwrap())
        .sum::<u8>()
        * 7;
    let evens_sum: u8 = tckn
        .bytes()
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| (i + 1) % 2 == 0 && i < 9)
        .map(|(_, b)| std::str::from_utf8(&vec![b]).unwrap().to_string())
        .map(|s| s.parse::<u8>().unwrap())
        .sum();
    let tenth = std::str::from_utf8(&vec![tckn.bytes().nth(9).unwrap()])
        .unwrap()
        .parse::<u8>()
        .unwrap();
    let diff = odds_sum_times_7 - evens_sum;
    diff % 10 == tenth
}

fn eleventh_digit_check(tckn: &str) -> bool {
    let all_sum = tckn[0..10]
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .sum::<u8>();
    let eleventh = tckn[10..11].parse::<u8>().unwrap();
    all_sum % 10 == eleventh
}

#[cfg(test)]
mod tests {
    use crate::{validate, generate};
    use rstest::rstest;

    #[test]
    fn test_validate_should_returns_false_when_more_than_eleven_chars() {
        let in_valid = "1234567891113";
        let actual = validate(in_valid);
        assert_eq!(actual, false);
    }

    #[test]
    fn test_validate_should_returns_false_when_less_than_eleven_chars() {
        let in_valid = "1234567891";
        let actual = validate(in_valid);
        assert_eq!(actual, false);
    }

    #[rstest]
    #[case("a")]
    #[case(".")]
    #[case("#")]
    #[case("?")]
    #[case("!")]
    #[case(",")]
    #[case(";")]
    #[case(":")]
    #[case(" ")]
    #[case("\t")]
    #[case("\n")]
    fn test_validate_should_returns_false_when_exactly_eleven_chars_with_non_digital(
        #[case] non_digit: &str,
    ) {
        let mut core = String::from("1234567891");
        core.push_str(non_digit);
        let in_valid = core.as_str();
        let actual = validate(in_valid);
        assert_eq!(actual, false);
    }
    #[test]
    fn test_validate_should_returns_false_when_starts_with_a_0() {
        let in_valid = "01234567891";
        let actual = validate(in_valid);
        assert_eq!(actual, false);
    }

    #[test]
    fn test_validate_should_returns_false_when_tenth_digit_check_fails() {
        let in_valid = "12345678916";
        let actual = validate(in_valid);
        assert_eq!(actual, false);
    }
    #[test]
    fn test_validate_should_returns_false_when_elevent_digit_check_fails() {
        let in_valid = "12345678951";
        let actual = validate(in_valid);
        assert_eq!(actual, false);
    }
    #[test]
    fn test_validate_should_returns_true_when_all_succeed() {
        let in_valid = "12345678950";
        let actual = validate(in_valid);
        assert_eq!(actual, true);
    }
    #[test]
    fn test_generate_should_generate_a_valid_tckn(){
        let generated_tckn = generate();
        let actual = validate(&generated_tckn);
        assert_eq!(true, actual);
    }
}
