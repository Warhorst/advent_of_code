use regex::Regex;

const EMPTY: &str = "";
const MAX_CAPTURES: usize = 8;

pub type Caps<'a> = [&'a str; MAX_CAPTURES];

/// Parses a multi line puzzle input to an iter of items of type T by
/// using a regex and the provided mapper.
/// The captures are stored in an array with fixes size MAX_CAPTURES, so the regex
/// can at max have this amount of captures or this call will crash.
/// Important: The first default capture group is ignored and the captures are still zero-based.
pub fn parse_entries<'a, T>(
    input: &'a str,
    regex: &'a Regex,
    mapper: impl Fn(Caps<'a>) -> T + 'a
) -> impl IntoIterator<Item=T> + 'a {
    regex
        .captures_iter(input)
        .map(|cap| {
            let mut caps = [EMPTY; 8];

            // skipping the first capture, as it contains the whole line which
            // we are not interested in.
            for (i, c) in cap.iter().skip(1).enumerate() {
                if let Some(m) = c {
                    caps[i] = m.as_str();
                }
            }

            caps
        })
        .map(mapper)
}
