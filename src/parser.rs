use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    iter::{Map, Peekable, StepBy},
    path::Path,
    str::Chars,
};

use crate::utils::{consume_string, json_file_to_string, json_stringify_strip};

#[derive(Debug, Clone, PartialEq)]
pub enum JSONValue {
    String(String),
    Number(f64),
    Null,
    Boolean(bool),
    Object(Vec<(String, JSONValue)>),
    Array(Vec<JSONValue>),
}

pub struct JSONFile {
    pathname: String,
    json_value: JSONValue,
}

impl JSONFile {
    fn new(path: &str) -> Self {
        let s =
            json_stringify_strip(&json_file_to_string(path).expect("To be there")).expect("exist");
        let json_value = parse_json(&mut s.chars().peekable()).expect("msg");
        JSONFile {
            pathname: path.to_string(),
            json_value,
        }
    }
}

pub fn parse_json(mut stream: &mut Peekable<Chars>) -> Result<JSONValue, &'static str> {
    match stream.next() {
        Some('{') => {
            // parse the value
            let mut hm = vec![];
            loop {
                match stream.next() {
                    Some('}') => return Ok(JSONValue::Object(hm)),
                    Some('"') => {}
                    _ => return Err("Expected '\"' after key in object"),
                }
                let k = consume_string(stream);
                if stream.next() != Some(':') {
                    return Err("Expected ':' after key in object");
                }

                match parse_json(stream) {
                    Ok(v) => hm.push((k, v)),
                    Err(_) => return Err("Invalid object value"),
                };
                match stream.next() {
                    Some(',') => continue,
                    Some('}') => break,
                    _ => return Err("Unexpected character in object"),
                }
            }
            Ok(JSONValue::Object(hm))
        }
        Some('\"') => {
            let s = consume_string(stream); // should error check strings
            Ok(JSONValue::String(s))
        }
        Some('[') => {
            let mut vec_elements = Vec::new();
            loop {
                if stream.peek() == Some(&']') {
                    stream.next();
                    return Ok(JSONValue::Array(vec_elements));
                }
                let c = stream.peek();
                match parse_json(stream) {
                    Ok(v) => vec_elements.push(v),
                    Err(_) => return Err("Invalid array value"),
                }
                match stream.next() {
                    Some(',') => continue,
                    Some(']') => break,
                    _ => return Err("Invalid end of line in object"),
                }
            }
            Ok(JSONValue::Array(vec_elements))
        }
        Some(c) if c.is_ascii_digit() || c == '-' => parse_number(&mut stream, c.to_string()),
        Some(c) if c == 't' || c == 'f' => parse_boolean(&mut stream, c == 't'),
        Some(c) if c == 'n' => parse_null(stream),
        _ => Err("Unsupported JSON value"),
    }
}

fn parse_null(stream: &mut Peekable<Chars>) -> Result<JSONValue, &'static str> {
    if stream.take(3).collect::<String>() == "ull" {
        Ok(JSONValue::Null)
    } else {
        Err("Unsupported JSON value (assumed null value)")
    }
}

fn parse_boolean(stream: &mut Peekable<Chars>, initial: bool) -> Result<JSONValue, &'static str> {
    if initial {
        if stream.take(3).collect::<String>() == "rue" {
            Ok(JSONValue::Boolean(true))
        } else {
            Err("Unsupported JSON value (assumed boolean)")
        }
    } else {
        if stream.take(4).collect::<String>() == "alse" {
            Ok(JSONValue::Boolean(false))
        } else {
            Err("Unsupported JSON value (assumed boolean)")
        }
    }
}

// TODO 1E-13
fn parse_number(
    stream: &mut Peekable<Chars>,
    starting_digit: String,
) -> Result<JSONValue, &'static str> {
    let mut num_str: String = starting_digit;

    while let Some(&next_char) = stream.clone().next().as_ref() {
        if next_char.is_digit(10)
            || next_char == '.'
            || next_char == 'e'
            || next_char == 'E'
            || next_char == '-'
            || next_char == '+'
        {
            stream.next();
            num_str.push(next_char);
        } else {
            break;
        }
    }

    num_str
        .parse::<f64>()
        .map(JSONValue::Number)
        .map_err(|_| "Invalid number")
}
