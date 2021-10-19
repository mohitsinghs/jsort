use json::codegen::Generator;
use json::object::Object;
use json::JsonValue;
use std::collections::BTreeMap;
use std::io;

/// Generator with sorting based on BTreeMap.
pub struct Gen {
    code: Vec<u8>,
    dent: u16,
    spaces_per_indent: u16,
}

impl Gen {
    pub fn new() -> Self {
        Gen {
            code: Vec::with_capacity(1024),
            dent: 0,
            spaces_per_indent: 2,
        }
    }

    pub fn consume(self) -> String {
        unsafe { String::from_utf8_unchecked(self.code) }
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
        self.write_char(b'{')?;
        let entries: BTreeMap<_, _> = object.iter().collect();

        let mut iter = entries.iter();
        if let Some((key, value)) = iter.next() {
            self.indent();
            self.new_line()?;
            self.write_string(key)?;
            self.write_char(b':')?;
            self.write_char(b' ')?;
            self.write_json(value)?;
        } else {
            self.write_char(b'}')?;
            return Ok(());
        }

        for (key, value) in iter {
            self.write_char(b',')?;
            self.new_line()?;
            self.write_string(key)?;
            self.write_char(b':')?;
            self.write_char(b' ')?;
            self.write_json(value)?;
        }

        self.dedent();
        self.new_line()?;
        self.write_char(b'}')
    }

    fn write_json(&mut self, json: &JsonValue) -> io::Result<()> {
        match *json {
            JsonValue::Null => self.write(b"null"),
            JsonValue::Short(ref short) => self.write_string(short.as_str()),
            JsonValue::String(ref string) => self.write_string(string),
            JsonValue::Number(ref number) => self.write_number(number),
            JsonValue::Boolean(true) => self.write(b"true"),
            JsonValue::Boolean(false) => self.write(b"false"),
            JsonValue::Array(ref array) => {
                self.write_char(b'[')?;
                let mut iter = array.iter();

                if let Some(item) = iter.next() {
                    self.indent();
                    self.new_line()?;
                    self.write_json(item)?;
                } else {
                    self.write_char(b']')?;
                    return Ok(());
                }

                for item in iter {
                    self.write_char(b',')?;
                    self.new_line()?;
                    self.write_json(item)?;
                }

                self.dedent();
                self.new_line()?;
                self.write_char(b']')
            }
            JsonValue::Object(ref object) => self.write_object(object),
        }
    }

    fn new_line(&mut self) -> io::Result<()> {
        self.code.push(b'\n');
        for _ in 0..(self.dent * self.spaces_per_indent) {
            self.code.push(b' ');
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
