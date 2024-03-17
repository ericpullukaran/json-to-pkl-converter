use crate::parser::{JSONFile, JSONValue};

pub struct Interpreter {
    json_file: JSONFile,
    out: String,
    obj_count: i32,
    arr_count: i32,
    tab_amount: i32,
}

impl Interpreter {
    pub fn convert_to_pkl(&mut self, values: JSONValue) -> String {
        match values {
            JSONValue::String(_) => todo!(),
            JSONValue::Number(_) => todo!(),
            JSONValue::Null => todo!(),
            JSONValue::Boolean(_) => todo!(),
            JSONValue::Object(_) => todo!(),
            JSONValue::Array(_) => todo!(),
        }
    }
}
