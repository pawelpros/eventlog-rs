use byteorder::{ByteOrder, LittleEndian};
use anyhow::{anyhow, Result};

pub fn read_short(data: &[u8], index: usize) -> Result<(u16,usize)> {
    const SIZE: usize = 2;
    verify_len(data, index, SIZE)?;
    let value = LittleEndian::read_u16(&data[index..index + SIZE]);
    Ok((value, index + SIZE))
}

pub fn read_int(data: &[u8], index: usize) -> Result<(u32, usize)> {
    const SIZE: usize = 4;
    verify_len(data, index, SIZE)?;
    let value = LittleEndian::read_u32(&data[index..index + SIZE]);
    Ok((value, index + SIZE))
}

pub fn read_long(data: &[u8], index: usize) -> Result<(u64, usize)> {
    const SIZE: usize = 8;
    verify_len(data, index, SIZE)?;
    let value = LittleEndian::read_u64(&data[index..index + SIZE]);
    Ok((value, index + SIZE))
}

pub fn format_name(name: String) -> String {
    let mut result = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(ch.to_ascii_uppercase());
    }

    result
}

fn verify_len(data: &[u8], index: usize, size: usize) -> Result<()> {
    if index + size > data.len() {
        return Err(anyhow!(
            "Cannot read data at index {}: not enough bytes (needed {}, have {}).",
            index,
            size,
            data.len() - index
        ));
    }
    Ok(())
}
