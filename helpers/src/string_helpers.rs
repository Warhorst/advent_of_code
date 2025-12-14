/// Remove everything in front of the given delimiter from the given string, including the delimiter.
/// If the input string does not contain the delimiter, the input is returned.
///
/// Example:
/// ```
/// use crate::aoc_lib::string_helpers::remove_prefix;
/// 
/// let string = "Foo Bar L60";
///
/// assert_eq!(remove_prefix(string, "L"), "60");
/// ```
pub fn remove_prefix<'a>(
    input: &'a str,
    delimiter: &str,
) -> &'a str {
    match input.split_once(delimiter) {
        Some(splits) => splits.1,
        None => input,
    }
}

/// Remove everything after the given delimiter from the given string, including the delimiter.
/// If the input string does not contain the delimiter, the input is returned.
///
/// Example:
/// ```
/// use crate::aoc_lib::string_helpers::remove_suffix;
///
/// let string = "L60 Foo Bar";
///
/// assert_eq!(remove_suffix(string, "60"), "L");
/// ```
pub fn remove_suffix<'a>(
    input: &'a str,
    delimiter: &str,
) -> &'a str {
    match input.split_once(delimiter) {
        Some(splits) => splits.0,
        None => input,
    }
}

#[cfg(test)]
mod tests {
    use super::{remove_prefix, remove_suffix};

    #[test]
    fn remove_prefix_works() {
        assert_eq!(remove_prefix("L60", "L"), "60");
        assert_eq!(remove_prefix("L60", "R"), "L60");
        assert_eq!(remove_prefix("foo bar L60", "L"), "60");
    }

    #[test]
    fn remove_suffix_works() {
        assert_eq!(remove_suffix("L60", "60"), "L");
        assert_eq!(remove_suffix("L60", "R"), "L60");
        assert_eq!(remove_suffix("L60 foo bar", "60"), "L")
    }
}
