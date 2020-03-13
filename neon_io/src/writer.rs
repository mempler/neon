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

    pub fn write<T: ?Sized>(&mut self, v: &T)
    where T: Serializable {
        v.serialize(self);
    }
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