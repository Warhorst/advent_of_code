use std::fmt::Debug;
use std::str::FromStr;
use proc_macros::from_regex;

#[test]
fn enum_all_variants() {
    #[from_regex]
    #[derive(Debug, Eq, PartialEq)]
    enum Enum {
        #[reg(r#"A (\d+) ([a-zA-Z]+)"#)]
        A {
            num: usize,
            string: String
        },
        #[reg(r#"B (\d+) ([a-zA-Z]+)"#)]
        B(usize, String),
        #[reg(r#"C"#)]
        C
    }

    assert_eq!(
        Enum::A {
            num: 42,
            string: "Wololo".to_string()
        },
        Enum::from_regex("A 42 Wololo")
    );
    assert_eq!(
        Enum::B(42, "Wololo".to_string()),
        Enum::from_regex("B 42 Wololo")
    );
    assert_eq!(
        Enum::C,
        Enum::from_regex("C")
    )
}

#[test]
fn enum_generics() {
    #[from_regex]
    #[derive(Debug, Eq, PartialEq)]
    enum Enum<AT, BT> where AT: FromStr, <AT as FromStr>::Err: Debug, BT: FromStr, <BT as FromStr>::Err: Debug {
        #[reg(r#"A (\d+) ([a-zA-Z]+)"#)]
        A {
            num: AT,
            string: BT
        },
        #[reg(r#"B (\d+) ([a-zA-Z]+)"#)]
        B(AT, BT),
        #[reg(r#"C"#)]
        C
    }

    assert_eq!(
        Enum::<usize, String>::A {
            num: 42,
            string: "Wololo".to_string()
        },
        Enum::<usize, String>::from_regex("A 42 Wololo")
    );
    assert_eq!(
        Enum::<usize, String>::B(42, "Wololo".to_string()),
        Enum::<usize, String>::from_regex("B 42 Wololo")
    );
    assert_eq!(
        Enum::<usize, String>::C,
        Enum::<usize, String>::from_regex("C")
    )
}