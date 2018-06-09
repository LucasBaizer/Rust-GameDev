use bytebuffer::ByteBuffer;
use std::collections::HashMap;

pub struct Nbt {
    strings: HashMap<String, String>,
    ints: HashMap<String, i32> 
}

impl Nbt {
    pub fn from_buffer(mut buf: ByteBuffer) -> Nbt {
        let mut nbt = Nbt {
            strings: HashMap::new(),
            ints: HashMap::new()
        };

        let tags = buf.read_u8();
        for _ in 0..tags {
            let tag_type = buf.read_u8();
            let strlen = buf.read_u8();
            let bytes = buf.read_bytes(strlen as usize);
            let tag_name = String::from_utf8(bytes).unwrap();

            if tag_type == 0 {
                let vallen = buf.read_u8();
                let val_bytes = buf.read_bytes(vallen as usize);
                let val = String::from_utf8(val_bytes).unwrap();

                nbt.strings.insert(tag_name, val);
            } else if tag_type == 1 {
                nbt.ints.insert(tag_name, buf.read_i32());
            }
        }

        nbt
    }

    pub fn to_buffer(&self, buf: &mut ByteBuffer) {
        let length = self.strings.len() + self.ints.len();

        buf.write_u8(length as u8);
        for (key, value) in &self.strings {
            buf.write_u8(0);
            buf.write_u8(key.len() as u8);
            buf.write_bytes(key.as_bytes());
            buf.write_u8(value.len() as u8);
            buf.write_bytes(value.as_bytes());
        }
        for (key, value) in &self.ints {
            buf.write_u8(1);
            buf.write_u8(key.len() as u8);
            buf.write_bytes(key.as_bytes());
            buf.write_i32(*value);
        }
    }

    pub fn get_string(&self, tag: &String) -> &String {
        self.strings.get(tag).unwrap()
    }

    pub fn get_i32(&self, tag: &String) -> i32 {
        *self.ints.get(tag).unwrap()
    }
}