use std::fmt::Debug;
use std::str::FromStr;

/// Simplified parse from str with better error message
pub fn parse<T>(input: &str) -> T
where T: FromStr, <T as FromStr>::Err: Debug {
    input.parse::<T>().expect(&format!("{input} could not be parsed"))
}