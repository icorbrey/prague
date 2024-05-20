use std::io::Read;

pub trait TryReadable {
    fn try_read_from<R: Read + TryTake>(reader: R) -> Result<Self, String>
    where
        Self: Sized;
}

impl TryReadable for i16 {
    fn try_read_from<R: Read + TryTake>(reader: R) -> Result<Self, String>
    where
        Self: Sized,
    {
        let mut buf = vec![];
        (reader.take(2))
            .read_to_end(&mut buf)
            .map_err(|_| "Unable to take bytes")?;
        Ok(i16::from_be_bytes(try_vec_to_array::<u8, 2>(buf)?))
    }
}

impl TryReadable for i32 {
    fn try_read_from<R: Read + TryTake>(reader: R) -> Result<Self, String>
    where
        Self: Sized,
    {
        let mut buf = vec![];
        (reader.take(4))
            .read_to_end(&mut buf)
            .map_err(|_| "Unable to take bytes")?;
        Ok(i32::from_be_bytes(try_vec_to_array::<u8, 4>(buf)?))
    }
}

impl TryReadable for Option<String> {
    fn try_read_from<R: Read + TryTake>(mut reader: R) -> Result<Self, String>
    where
        Self: Sized,
    {
        let length = reader.try_take::<i16>()? as u64;
        if 0 < length {
            let mut buf = vec![];
            (reader.take(length))
                .read_to_end(&mut buf)
                .map_err(|_| "Unable to take bytes")?;
            Ok(Some(
                String::from_utf8(buf).map_err(|_| "Unable to read string")?,
            ))
        } else {
            Ok(None)
        }
    }
}

pub trait TryTake {
    fn try_take<T: TryReadable + Sized>(&mut self) -> Result<T, String>;
}

impl<R: Read> TryTake for R {
    fn try_take<T: TryReadable + Sized>(&mut self) -> Result<T, String> {
        T::try_read_from(self)
    }
}

fn try_vec_to_array<T, const N: usize>(v: Vec<T>) -> Result<[T; N], String> {
    v.try_into().map_err(|e: Vec<T>| {
        format!(
            "Could not convert vector of length {0} to array of length {1}.",
            e.len(),
            N
        )
    })
}
