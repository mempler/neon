use crate::serializable::Serializable;

pub struct Writer {
    data: Vec<u8>,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            data: vec![]
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn write_i8(&mut self, v: i8) {
        self.write_bytes(&v.to_le_bytes());
    }

    pub fn write_i16(&mut self, v: i16) {
        self.write_bytes(&v.to_le_bytes());
    }

    pub fn write_i32(&mut self, v: i32) {
        self.write_bytes(&v.to_le_bytes());
    }

    pub fn write_i64(&mut self, v: i64) {
        self.write_bytes(&v.to_le_bytes());
    }

    pub fn write_u8(&mut self, v: u8) {
        self.write_bytes(&v.to_le_bytes());
    }

    pub fn write_u16(&mut self, v: u16) {
        self.write_bytes(&v.to_le_bytes());
    }

    pub fn write_u32(&mut self, v: u32) {
        self.write_bytes(&v.to_le_bytes());
    }

    pub fn write_u64(&mut self, v: u64) {
        self.write_bytes(&v.to_le_bytes());
    }

    pub fn write_f32(&mut self, v: f32) {
        self.write_bytes(&v.to_le_bytes());
    }

    pub fn write_f64(&mut self, v: f64) {
        self.write_bytes(&v.to_le_bytes());
    }

    pub fn write_uleb128(&mut self, mut v: u64) {
        while v >= 0x80 {
            self.write_u8((v | 0x80) as u8);
            v >>= 7;
        }

        self.write_u8(v as u8);
    }

    pub fn write_str(&mut self, v: &str) {
        self.write_u8(0x0b);

        if v.len() > 0 {
            let bytes = v.as_bytes();

            self.write_uleb128(bytes.len() as _);
            self.write_bytes(bytes);
        } else {
            self.write_u8(0x00);
        }
    }

    pub fn write_string(&mut self, v: String) {
        self.write_str(&v);
    }

    pub fn write<T: ?Sized>(&mut self, v: &T)
    where T: Serializable {
        v.serialize(self);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Serializable for &'_ str {
    fn serialize(&self, writer: &mut Writer) {
        writer.write_str(self)
    }
}

impl Serializable for String {
    fn serialize(&self, writer: &mut Writer) {
        writer.write_str(&self);
    }
}

impl<T: Serializable> Serializable for &'_ [T] {
    fn serialize(&self, writer: &mut Writer) {
        writer.write_i16(self.len() as _);

        for it in self.iter() {
            writer.write(it);
        }
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn serialize(&self, writer: &mut Writer) {
        writer.write_i16(self.len() as _);

        for it in self.iter() {
            writer.write(it);
        }
    }
}

impl Serializable for &'_ () {
    fn serialize(&self, _writer: &mut Writer) {}
}

impl Serializable for () {
    fn serialize(&self, _writer: &mut Writer) {}
}

macro_rules! impl_primitive {
    ($(($type: ty, $fn: ident)),*) => {
        $(
            impl Serializable for $type {
                fn serialize(&self, writer: &mut Writer) {
                    writer.$fn(*self);
                }
            }
        )*
    };
}

impl_primitive! {
    (i8, write_i8),
    (i16, write_i16),
    (i32, write_i32),
    (i64, write_i64),
    (u8, write_u8),
    (u16, write_u16),
    (u32, write_u32),
    (u64, write_u64),
    (f32, write_f32),
    (f64, write_f64)
}
