use crate::serializable::Deserializable;

use std::convert::TryInto;

pub struct Reader<'a> {
    input:    &'a [u8],
    position: usize,
}

impl<'a> Reader<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            position: 0,
        }
    }

    pub fn available(&self) -> usize {
        self.input.len() - self.position
    }

    pub fn read_bytes(&mut self, amount: usize) -> Option<&'a [u8]> {
        if self.available() < amount {
            None
        } else {
            self.position += amount;

            Some(&self.input[self.position - amount .. self.position])
        }
    }

    pub fn read_i8(&mut self) -> Option<i8> {
        Some(self.read_bytes(1)?[0] as _)
    }

    pub fn read_i16(&mut self) -> Option<i16> {
        match self.read_bytes(2)?.try_into() {
            Ok(bytes) => Some(i16::from_le_bytes(bytes)),
            Err(_) => None,
        }
    }

    pub fn read_i32(&mut self) -> Option<i32> {
        match self.read_bytes(4)?.try_into() {
            Ok(bytes) => Some(i32::from_le_bytes(bytes)),
            Err(_) => None,
        }
    }

    pub fn read_i64(&mut self) -> Option<i64> {
        match self.read_bytes(8)?.try_into() {
            Ok(bytes) => Some(i64::from_le_bytes(bytes)),
            Err(_) => None,
        }
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        Some(self.read_bytes(1)?[0])
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        match self.read_bytes(2)?.try_into() {
            Ok(bytes) => Some(u16::from_le_bytes(bytes)),
            Err(_) => None,
        }
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        match self.read_bytes(4)?.try_into() {
            Ok(bytes) => Some(u32::from_le_bytes(bytes)),
            Err(_) => None,
        }
    }

    pub fn read_u64(&mut self) -> Option<u64> {
        match self.read_bytes(8)?.try_into() {
            Ok(bytes) => Some(u64::from_le_bytes(bytes)),
            Err(_) => None,
        }
    }

    pub fn read_f32(&mut self) -> Option<f32> {
        match self.read_bytes(4)?.try_into() {
            Ok(bytes) => Some(f32::from_le_bytes(bytes)),
            Err(_) => None,
        }
    }

    pub fn read_f64(&mut self) -> Option<f64> {
        match self.read_bytes(8)?.try_into() {
            Ok(bytes) => Some(f64::from_le_bytes(bytes)),
            Err(_) => None,
        }
    }

    pub fn read<T: Sized>(&mut self) -> Option<T::Output<'a>>
    where T: Deserializable {
        T::deserialize(self)
    }
}

macro_rules! impl_primitive {
    ($(($type: ty, $fn: ident)),*) => {
        $(
            impl Deserializable for $type {
                type Output<'a> = $type;

                fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
                    reader.$fn()
                }
            }

            impl Deserializable for &'_ $type {
                type Output<'a> = $type;

                fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
                    reader.$fn()
                }
            }
        )*
    };
}

impl_primitive! {
    (i8, read_i8),
    (i16, read_i16),
    (i32, read_i32),
    (i64, read_i64),
    (u8, read_u8),
    (u16, read_u16),
    (u32, read_u32),
    (u64, read_u64),
    (f32, read_f32),
    (f64, read_f64)
}
