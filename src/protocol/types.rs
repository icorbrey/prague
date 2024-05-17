use std::io::Read;

pub trait StringExt {
    fn try_read_maybe<R: Read>(reader: &mut R) -> Result<Option<String>, ()>;
}

impl StringExt for String {
    fn try_read_maybe<R: Read>(mut reader: &mut R) -> Result<Option<String>, ()> {
        let length = try_read_i16(&mut reader)?;

        if 0 < length {
            Ok(Some(try_read_string(&mut reader, length as u64)?))
        } else {
            Ok(None)
        }
    }
}

fn try_read_i16<R: Read>(reader: &mut R) -> Result<i16, ()> {
    let mut buf = vec![];
    reader.take(2).read_to_end(&mut buf).map_err(|_| ())?;
    Ok(i16::from_be_bytes(try_vec_to_array::<u8, 2>(buf)?))
}

fn try_read_string<R: Read>(reader: &mut R, length: u64) -> Result<String, ()> {
    let mut buf = vec![];
    reader.take(length).read_to_end(&mut buf).map_err(|_| ())?;
    Ok(String::from_utf8(buf).map_err(|_| ())?)
}

fn try_vec_to_array<T, const N: usize>(v: Vec<T>) -> Result<[T; N], ()> {
    v.try_into().map_err(|_| ())
}
