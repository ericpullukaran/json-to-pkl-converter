use json_pkl_converter::{parser::parse_json, utils::json_stringify_strip};

fn main() {
    const V1: &str = include_str!("../test_data/test_v01.json");
    let _ = parse_json(&mut json_stringify_strip(V1).expect("exist").chars().peekable()).unwrap();
}
