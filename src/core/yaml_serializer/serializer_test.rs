#[cfg(test)]
mod tests {
    use crate::core::data::kv::Kv;
    use crate::core::data::kvs::Kvs;
    use crate::core::data::path::Path;
    use crate::core::data::tokens::Tokens;
    use crate::core::yaml_serializer::serializer::Serializer;

    #[test]
    fn test_root_dict() {
        let mut serializer = Serializer::new(Kvs {
            items: vec![
                Kv {
                    path: Path::from("$"),
                    value: Tokens::MkDict,
                },
                Kv {
                    path: Path::from("$.a"),
                    value: Tokens::String("aaa".to_string()),
                },
                Kv {
                    path: Path::from("$"),
                    value: Tokens::EndDict,
                },
            ],
        });
        serializer.serialize();
        let actual = serializer.get_raw();

        assert_eq!(actual, String::from("a: aaa\n"));
    }

    #[test]
    fn test_root_array() {
        let mut serializer = Serializer::new(Kvs {
            items: vec![
                Kv {
                    path: Path::from("$"),
                    value: Tokens::MkArray,
                },
                Kv {
                    path: Path::from("$[0]"),
                    value: Tokens::String("aaa".to_string()),
                },
                Kv {
                    path: Path::from("$"),
                    value: Tokens::EndArray,
                },
            ],
        });
        serializer.serialize();
        let actual = serializer.get_raw();

        assert_eq!(actual, String::from("- aaa\n"));
    }

    #[test]
    fn test_nested_dict() {
        let mut serializer = Serializer::new(Kvs {
            items: vec![
                Kv {
                    path: Path::from("$"),
                    value: Tokens::MkDict,
                },
                Kv {
                    path: Path::from("$.a"),
                    value: Tokens::String("aaa".to_string()),
                },
                Kv {
                    path: Path::from("$.b"),
                    value: Tokens::MkDict,
                },
                Kv {
                    path: Path::from("$.b.c"),
                    value: Tokens::String("ddd".to_string()),
                },
                Kv {
                    path: Path::from("$.b"),
                    value: Tokens::EndDict,
                },
                Kv {
                    path: Path::from("$.e"),
                    value: Tokens::Number(108),
                },
                Kv {
                    path: Path::from("$"),
                    value: Tokens::EndDict,
                },
            ],
        });
        serializer.serialize();
        let actual = serializer.get_raw();

        assert_eq!(
            actual,
            String::from(
                "a: aaa
b: 
  c: ddd
e: 108
"
            )
        );
    }

    #[test]
    fn test_array_in_array() {
        let mut serializer = Serializer::new(Kvs {
            items: vec![
                Kv {
                    path: Path::from("$"),
                    value: Tokens::MkDict,
                },
                Kv {
                    path: Path::from("$.a"),
                    value: Tokens::MkArray,
                },
                Kv {
                    path: Path::from("$.a[0]"),
                    value: Tokens::MkArray,
                },
                Kv {
                    path: Path::from("$.a[0][0]"),
                    value: Tokens::String("aa".to_string()),
                },
                Kv {
                    path: Path::from("$.a[0]"),
                    value: Tokens::EndArray,
                },
                Kv {
                    path: Path::from("$.a"),
                    value: Tokens::EndArray,
                },
                Kv {
                    path: Path::from("$"),
                    value: Tokens::EndDict,
                },
            ],
        });
        serializer.serialize();
        let actual = serializer.get_raw();

        assert_eq!(
            actual,
            String::from(
                "a: 
- - aa
"
            )
        );
    }
}
