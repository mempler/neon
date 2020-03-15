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

    pub fn seek(&mut self, pos: usize) {
        if self.available() >= pos {
            self.position = pos
        }
    }

    pub fn seek_forward(&mut self, pos: usize) {
        if self.available() >= pos {
            self.position += pos
        }
    }

    pub fn seek_backward(&mut self, pos: usize) {
        if self.available() >= pos {
            self.position -= pos
        }
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

    pub fn read_uleb128(&mut self) -> Option<u64> {
        let mut result = 0;
        let mut shift = 0;

        loop {
            let byte = self.read_u8()?;

            result |= ((byte & 0x7F) as u64) << shift;
            shift += 7;

            if (byte & 0x80) == 0 {
                break
            }
        }

        Some(result)
    }

    pub fn read_str(&mut self) -> Option<&'a str> {
        let tag = self.read_u8()?;

        match tag {
            11 => {
                let length = self.read_uleb128()?;
                let bytes = self.read_bytes(length as _)?;

                match std::str::from_utf8(bytes) {
                    Ok(string) => Some(string),
                    Err(_) => None,
                }
            },
            _ => None,
        }
    }

    pub fn read_string(&mut self) -> Option<String> {
        Some(self.read_str()?.to_string())
    }

    pub fn read<T: Sized>(&mut self) -> Option<T::Output<'a>>
    where T: Deserializable {
        T::deserialize(self)
    }
}

impl Deserializable for &'_ str {
    type Output<'a> = &'a str;

    fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
        reader.read_str()
    }
}

impl Deserializable for String {
    type Output<'a> = String;

    fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
        reader.read_string()
    }
}

impl<T: Deserializable> Deserializable for &'_ [T] {
    type Output<'a> = Vec<T::Output<'a>>;

    fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
        let length = reader.read_i16()?;
        let mut result = Vec::with_capacity(length as _);

        for _ in 0 .. length {
            result.push(T::deserialize(reader)?);
        }

        Some(result)
    }
}

impl<T: Deserializable> Deserializable for Vec<T> {
    type Output<'a> = Vec<T::Output<'a>>;

    fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
        let length = reader.read_i16()?;
        let mut result = Vec::with_capacity(length as _);

        for _ in 0 .. length {
            result.push(T::deserialize(reader)?);
        }

        Some(result)
    }
}

impl Deserializable for &'_ () {
    type Output<'a> = ();

    fn deserialize<'a>(_reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
        Some(())
    }
}

impl Deserializable for () {
    type Output<'a> = ();

    fn deserialize<'a>(_reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
        Some(())
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
