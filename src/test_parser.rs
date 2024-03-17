#[cfg(test)]
mod parse_json {
    use std::vec::Vec;
    use JSONValue::*;

    use crate::{
        parser::{parse_json, JSONValue},
        utils::{consume_string, json_file_to_string, json_stringify_strip},
    };

    use super::*;

    #[test]
    fn consume_simple_null() {
        let mut s = r#"null"#.chars().peekable();
        let result = parse_json(&mut s).expect("Must be valid json");
        let valid = JSONValue::Null;
        assert_eq!(result, valid);
    }

    #[test]
    fn consume_simple_exponent() {
        let mut s = r#"-1E-2"#.chars().peekable();
        let result = parse_json(&mut s).expect("Must be valid json");
        let valid = JSONValue::Number(-0.01);
        assert_eq!(result, valid);
    }

    #[test]
    fn consume_simple_array() {
        let mut s = r#"[]"#.chars().peekable();
        let result = parse_json(&mut s).expect("Must be valid json");
        let valid = JSONValue::Array(vec![]);
        assert_eq!(result, valid);
    }

    #[test]
    fn consume_simple_empty_obj() {
        let mut s = r#"{}"#.chars().peekable();
        let result = parse_json(&mut s).expect("Must be valid json");
        let valid = JSONValue::Object(Vec::new());
        assert_eq!(result, valid);
    }

    #[test]
    fn obj_with_empty_array() {
        let mut s = r#"{"a":[]}"#.chars().peekable();
        let result = parse_json(&mut s).expect("Must be valid json");
        let valid = Object(vec![("a".to_string(), Array(vec![]))]);
        assert_eq!(result, valid);
    }

    #[test]
    fn simiple_check() {
        let mut s = r#"[{"a":"b"}]"#.chars().peekable();
        let result = parse_json(&mut s);
        assert_eq!(
            result,
            Ok(Array(vec![Object(vec![(
                "a".to_string(),
                String("b".to_string())
            )])]))
        );
    }

    #[test]
    fn consume_closing_curly() {
        let mut s = r#"}"#.chars().peekable();
        let result = parse_json(&mut s);
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn consume_simple_number() {
        let mut s = r#"12"#.chars().peekable();
        let result = parse_json(&mut s).expect("Must be valid json");
        let valid = JSONValue::Number(12.0);
        assert_eq!(result, valid);
    }

    #[test]
    fn consume_simple_e_number() {
        let mut s = r#"1e2"#.chars().peekable();
        let result = parse_json(&mut s).expect("Must be valid json");
        let valid = JSONValue::Number(100.0);
        assert_eq!(result, valid);
    }

    #[test]
    fn consume_simple_string() {
        let mut s = r#""hello""#.chars().peekable();
        let result = parse_json(&mut s).expect("Must be valid json");
        let valid = JSONValue::String("hello".to_string());
        assert_eq!(result, valid);
    }

    #[test]
    fn consume_simple_object() {
        let mut s = r#"{"hello":"world"}"#.chars().peekable();
        let result = parse_json(&mut s).expect("Must be valid json");
        let mut hm = Vec::new();
        hm.push(("hello".to_string(), JSONValue::String("world".to_string())));
        let valid = JSONValue::Object(hm);
        assert_eq!(result, valid);
    }

    #[test]
    fn consume_array_with_strings_nested_empty() {
        let mut s = r#"[[]]"#.chars().peekable();
        let result = parse_json(&mut s).expect("Must be valid json");
        let valid = JSONValue::Array(vec![JSONValue::Array(vec![])]);
        assert_eq!(result, valid);
    }

    #[test]
    fn consume_array_with_strings() {
        let mut s = r#"["hello","world","test",["asdf","asdf"]]"#.chars().peekable();
        let result = parse_json(&mut s).expect("Must be valid json");
        let valid = JSONValue::Array(vec![
            JSONValue::String("hello".to_string()),
            JSONValue::String("world".to_string()),
            JSONValue::String("test".to_string()),
            JSONValue::Array(vec![
                JSONValue::String("asdf".to_string()),
                JSONValue::String("asdf".to_string()),
            ]),
        ]);
        assert_eq!(result, valid);
    }

    #[test]
    fn consume_array_with_strings_nested_array() {
        let mut s = json_stringify_strip(
            r#"{
    "object": {
        "thing": 1,
        "another": 2.0e10,
        "true": false,
        "exists": null,
        "items": [
            {
                "type": "item thingo"
            },
            true,
            "hey!",
            [
                false,
                true
            ]
        ]
    }
}"#,
        )
        .unwrap();
        let mut ss = s.chars().peekable();
        let result = parse_json(&mut ss).expect("Must be valid json");

        let valid = Object(vec![(
            "object".to_string(),
            Object(vec![
                ("thing".to_string(), Number(1.0)),
                ("another".to_string(), Number(20000000000.0)),
                ("true".to_string(), Boolean(false)),
                ("exists".to_string(), Null),
                (
                    "items".to_string(),
                    Array(vec![
                        Object(vec![(
                            "type".to_string(),
                            String("item thingo".to_string()),
                        )]),
                        Boolean(true),
                        String("hey!".to_string()),
                        Array(vec![Boolean(false), Boolean(true)]),
                    ]),
                ),
            ]),
        )]);
        assert_eq!(result, valid);
    }

    #[test]
    fn consume_nested_object() {
        let mut s =
            r#"{"person":{"name":"John","age":30,"address":{"city":"New York","zip":10001}}}"#
                .chars()
                .peekable();
        let result = parse_json(&mut s).expect("Must be valid json");

        let mut address = Vec::new();
        address.push((
            "city".to_string(),
            JSONValue::String("New York".to_string()),
        ));
        address.push(("zip".to_string(), JSONValue::Number(10001.0)));

        let mut person = Vec::new();
        person.push(("name".to_string(), JSONValue::String("John".to_string())));
        person.push(("age".to_string(), JSONValue::Number(30.0)));
        person.push(("address".to_string(), JSONValue::Object(address)));
        let mut fin = Vec::new();
        fin.push(("person".to_string(), JSONValue::Object(person)));

        let valid = JSONValue::Object(fin);
        assert_eq!(result, valid);
    }

    #[test]
    fn consume_array_of_objects() {
        let mut s = r#"[{"name":"John","age":30},{"name":"Jane","age":25}]"#
            .chars()
            .peekable();
        let result = parse_json(&mut s).expect("Must be valid json");

        let mut person1 = Vec::new();
        person1.push(("name".to_string(), JSONValue::String("John".to_string())));
        person1.push(("age".to_string(), JSONValue::Number(30.0)));

        let mut person2 = Vec::new();
        person2.push(("name".to_string(), JSONValue::String("Jane".to_string())));
        person2.push(("age".to_string(), JSONValue::Number(25.0)));

        let valid = JSONValue::Array(vec![JSONValue::Object(person1), JSONValue::Object(person2)]);
        assert_eq!(result, valid);
    }

    #[test]
    fn consume_complex_nested_structure() {
        let mut s =
            r#"{"data":{"items":["Apple","Banana","Grape"],"info":{"count":3,"valid":true,"time":null}}}"#
                .chars().peekable();
        let result = parse_json(&mut s).expect("Must be valid json");

        let items = vec![
            JSONValue::String("Apple".to_string()),
            JSONValue::String("Banana".to_string()),
            JSONValue::String("Grape".to_string()),
        ];

        let mut info = Vec::new();
        info.push(("count".to_string(), JSONValue::Number(3.0)));
        info.push(("valid".to_string(), JSONValue::Boolean(true)));
        info.push(("time".to_string(), JSONValue::Null));

        let mut data = Vec::new();
        data.push(("items".to_string(), JSONValue::Array(items)));
        data.push(("info".to_string(), JSONValue::Object(info)));

        let mut fin = Vec::new();
        fin.push(("data".to_string(), JSONValue::Object(data)));

        let valid = JSONValue::Object(fin);
        assert_eq!(result, valid);
    }

    #[test]
    fn test_v01() {
        let s = json_stringify_strip(
            &json_file_to_string("test_data/test_v01.json").expect("To be there"),
        )
        .expect("exist");
        let _ = parse_json(&mut s.chars().peekable());
    }

    #[test]
    fn test_v02() {
        let s = json_stringify_strip(
            &json_file_to_string("test_data/test_v02.json").expect("To be there"),
        )
        .expect("exist");
        let _ = parse_json(&mut s.chars().peekable());
    }
}
