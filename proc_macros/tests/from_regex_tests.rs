use std::fmt::Debug;
use std::str::FromStr;
use proc_macros::from_regex;

#[test]
fn struct_all_types() {
    #[from_regex]
    #[reg(r#"A (\d+) ([a-zA-Z]+)"#)]
    #[derive(Debug, Eq, PartialEq)]
    struct Named {
        num: usize,
        string: String
    }

    #[from_regex]
    #[reg(r#"B (\d+) ([a-zA-Z]+)"#)]
    #[derive(Debug, Eq, PartialEq)]
    struct Unnamed(usize, String);

    #[from_regex]
    #[reg(r#"C"#)]
    #[derive(Debug, Eq, PartialEq)]
    struct Unit;

    assert_eq!(
        Named {
            num: 42,
            string: "Wololo".to_string()
        },
        Named::from_regex("A 42 Wololo")
    );
    assert_eq!(
        Unnamed(42, "Wololo".to_string()),
        Unnamed::from_regex("B 42 Wololo")
    );
    assert_eq!(
        Unit,
        Unit::from_regex("C")
    )
}

#[test]
fn struct_generics() {
    #[from_regex]
    #[reg(r#"A (\d+) ([a-zA-Z]+)"#)]
    #[derive(Debug, Eq, PartialEq)]
    struct Named<A, B> where A: FromStr, B: FromStr, <A as FromStr>::Err: Debug, <B as FromStr>::Err: Debug {
        num: A,
        string: B
    }

    #[from_regex]
    #[reg(r#"B (\d+) ([a-zA-Z]+)"#)]
    #[derive(Debug, Eq, PartialEq)]
    struct Unnamed<A, B>(A, B) where A: FromStr, B: FromStr, <A as FromStr>::Err: Debug, <B as FromStr>::Err: Debug;

    #[from_regex]
    #[reg(r#"C"#)]
    #[derive(Debug, Eq, PartialEq)]
    struct Unit;

    assert_eq!(
        Named {
            num: 42,
            string: "Wololo".to_string()
        },
        Named::from_regex("A 42 Wololo")
    );
    assert_eq!(
        Unnamed(42, "Wololo".to_string()),
        Unnamed::from_regex("B 42 Wololo")
    );
    assert_eq!(
        Unit,
        Unit::from_regex("C")
    )
}

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
        Enum::<usize, String>::C,
        Enum::<usize, String>::from_regex("C")
    )
}