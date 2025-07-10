/// Create a value from a given haystack based on a regex. Should only be used with the associated 
/// derive macro from proc_macros.
/// 
/// Do I parse values from regexes? No.
/// Is the name of this trait therefore bad? Yes.
/// Do I care? No.
/// 
/// Maybe this should not be a trait, but works for now.
pub trait FromRegex {
    fn from_regex(haystack: &str) -> Self;
}