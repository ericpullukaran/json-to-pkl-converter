use std::{fs::File, io::Read, iter::Peekable, path::Path, str::Chars};

pub fn json_file_to_string(path: &str) -> Result<String, &str> {
    let mut file_content = String::new();
    let mut file = match File::open(Path::new(path)) {
        Ok(file) => file,
        Err(_) => return Err("Unable to open file"),
    };

    if let Err(_) = file.read_to_string(&mut file_content) {
        return Err("Unable to read file");
    }
    Ok(file_content)
}

pub fn json_stringify_strip(file_content: &str) -> Result<String, &'static str> {
    let mut in_string = false;
    let mut prev_char_was_backslash = false;
    let mut cleaned_content = String::new();

    for c in file_content.chars() {
        match c {
            '"' if !prev_char_was_backslash => in_string = !in_string,
            _ if c.is_whitespace() && !in_string => continue,
            _ => {}
        }

        cleaned_content.push(c);
        prev_char_was_backslash = c == '\\' && !prev_char_was_backslash;
    }
    Ok(cleaned_content)
}

pub fn consume_string(stream: &mut Peekable<Chars>) -> String {
    let mut s = String::new();
    let mut prev_char_was_backslash = false;
    while let Some(ch) = stream.next() {
        match ch {
            '\\' if !prev_char_was_backslash => prev_char_was_backslash = true,
            _ if prev_char_was_backslash => {
                s.push(ch);
                prev_char_was_backslash = false
            }
            '"' => break,
            _ => {
                s.push(ch);
            }
        }
    }
    s
}

#[cfg(test)]
mod consume_string {
    use crate::utils::consume_string;

    use super::*;

    #[test]
    fn consume_simple_string() {
        let mut stream = "Hello, world!\"".chars().peekable();
        let result = consume_string(&mut stream);
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn consume_string_with_escaped_quote() {
        let mut stream = "Hello, \\\"world!\\\"\"".chars().peekable();
        let result = consume_string(&mut stream);
        assert_eq!(result, "Hello, \"world!\"");
    }

    #[test]
    fn consume_string_with_escaped_backslash() {
        let mut stream = "\\\\\"".chars().peekable();
        let result = consume_string(&mut stream);
        assert_eq!(result, "\\"); // The string consists of an escaped backslash
    }

    #[test]
    fn consume_string_with_multiple_escaped_sequences() {
        let mut stream = "Line 1\\\nLine 2\\\tTabbed\"".chars().peekable();
        let result = consume_string(&mut stream);
        assert_eq!(result, "Line 1\nLine 2\tTabbed");
    }

    #[test]
    fn consume_string_ends_at_first_non_escaped_quote() {
        let mut stream = "git clean -xdf .next .turbo node_mo\\\"dules\""
            .chars()
            .peekable();
        let result = consume_string(&mut stream);
        assert_eq!(result, "git clean -xdf .next .turbo node_mo\"dules");
    }
}
