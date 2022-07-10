use json::codegen::Generator;
use json::object::Object;
use json::JsonValue;
use std::collections::BTreeMap;
use std::io;

use crate::constants::{
    COLON, COMMA, CURLY_END, CURLY_START, FALSE, NEWLINE, NULL, SPACE, SQUARE_END, SQUARE_START,
    TRUE,
};

/// Generator with sorting based on BTreeMap.
pub struct Gen {
    code: Vec<u8>,
    dent: u16,
    spaces_per_indent: u16,
    max_width: u16,
    key_length: u16,
}

impl Gen {
    pub fn new() -> Self {
        Gen {
            code: Vec::with_capacity(1024),
            dent: 0,
            spaces_per_indent: 2,
            max_width: 80,
            key_length: 0,
        }
    }

    pub fn consume(self) -> String {
        unsafe { String::from_utf8_unchecked(self.code) }
    }
}

fn get_char_width(json: &JsonValue) -> usize {
    match json {
        JsonValue::Null => 4,
        JsonValue::Boolean(val) => val.to_string().len(),
        JsonValue::Number(num) => {
            let mut len = num.to_string().len();
            if num.to_string().contains(".") {
                len += 1;
            }
            len
        }
        JsonValue::String(str) => str.len() + 2,
        JsonValue::Array(arr) => {
            let mut sum = 0;
            for val in arr {
                sum += get_char_width(val) + 1;
            }
            sum + 2
        }
        JsonValue::Object(obj) => {
            let mut max = 0;
            for (key, val) in obj.iter() {
                let len = get_char_width(val) + key.len() + 3;
                if len > max {
                    max = len;
                }
            }
            max + 2
        }
        JsonValue::Short(short) => short.to_string().len() + 2,
    }
}

impl Gen {
    #[inline(always)]
    fn write_kv(&mut self, key: &&str, value: &&JsonValue) -> io::Result<()> {
        self.write_string(key)?;
        self.write_char(COLON)?;
        self.write_char(SPACE)?;
        self.write_json(value)?;
        Ok(())
    }
}

impl Generator for Gen {
    type T = Vec<u8>;

    #[inline(always)]
    fn get_writer(&mut self) -> &mut Vec<u8> {
        &mut self.code
    }

    #[inline(always)]
    fn write_min(&mut self, _: &[u8], min: u8) -> io::Result<()> {
        self.code.push(min);
        Ok(())
    }

    #[inline(always)]
    fn write_object(&mut self, object: &Object) -> io::Result<()> {
        self.write_char(CURLY_START)?;
        let entries: BTreeMap<_, _> = object.iter().collect();

        let mut iter = entries.iter();
        if let Some((key, value)) = iter.next() {
            self.key_length += key.len() as u16 + 3;
            self.indent();
            self.new_line()?;
            self.write_kv(key, value)?;
        } else {
            self.write_char(CURLY_END)?;
            self.key_length = 0;
            return Ok(());
        }

        for (key, value) in iter {
            self.key_length += key.len() as u16 + 3;
            self.write_char(COMMA)?;
            self.new_line()?;
            self.write_kv(key, value)?;
        }

        self.dedent();
        self.new_line()?;
        self.key_length = 0;
        self.write_char(CURLY_END)
    }

    fn write_json(&mut self, json: &JsonValue) -> io::Result<()> {
        match *json {
            JsonValue::Null => self.write(NULL),
            JsonValue::Short(ref short) => self.write_string(short.as_str()),
            JsonValue::String(ref string) => self.write_string(string),
            JsonValue::Number(ref number) => self.write_number(number),
            JsonValue::Boolean(true) => self.write(TRUE),
            JsonValue::Boolean(false) => self.write(FALSE),
            JsonValue::Array(ref array) => {
                let estimated_width = self.dent * self.spaces_per_indent
                    + get_char_width(json) as u16
                    + self.key_length
                    + 2;
                let should_break: bool = estimated_width > self.max_width;
                self.write_char(SQUARE_START)?;
                self.indent();
                if should_break {
                    self.new_line()?;
                }

                for (i, item) in array.iter().enumerate() {
                    self.write_json(item)?;
                    if i < array.len() - 1 {
                        self.write_char(COMMA)?;
                        if should_break {
                            self.new_line()?;
                        } else {
                            self.write_char(SPACE)?;
                        }
                    }
                }

                self.dedent();
                if should_break {
                    self.new_line()?;
                }
                self.write_char(SQUARE_END)
            }
            JsonValue::Object(ref object) => self.write_object(object),
        }
    }

    fn new_line(&mut self) -> io::Result<()> {
        self.code.push(NEWLINE);
        for _ in 0..(self.dent * self.spaces_per_indent) {
            self.code.push(SPACE);
        }
        Ok(())
    }

    fn indent(&mut self) {
        self.dent += 1;
    }

    fn dedent(&mut self) {
        self.dent -= 1;
    }
}
