use bincode::{
    config::{Configuration, Fixint, LittleEndian, NoLimit, SkipFixedArrayLength},
    error::{DecodeError, EncodeError},
    BorrowDecode, Encode,
};

const CONFIG: Configuration<LittleEndian, Fixint, SkipFixedArrayLength, NoLimit> =
    bincode::config::standard()
        .with_fixed_int_encoding()
        .skip_fixed_array_length();

pub fn encode<M>(message: &M, buf: &mut Vec<u8>) -> Result<u32, EncodeError>
where
    M: Encode,
{
    let len = bincode::encode_into_std_write(message, buf, CONFIG)? as u32;
    Ok(len)
}

pub fn decode<'d, M>(buf: &'d [u8]) -> Result<M, DecodeError>
where
    M: BorrowDecode<'d>,
{
    let (message, _) = bincode::decode_from_slice(buf, CONFIG)?;
    Ok(message)
}
