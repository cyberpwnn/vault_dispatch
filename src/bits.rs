pub fn read_u32(startindex:u32, data:&[u8]) -> u32
{
    let start = startindex as usize;
    return ((data[start] as u32) << 24) + ((data[start+1] as u32) << 16)
        + ((data[start+2] as u32) << 8) + ((data[start+3] as u32) << 0);
}

pub fn read_u64(startindex:u32, data:&[u8]) -> u64
{
    let start = startindex as usize;
    return ((data[start] as u64) << 56) + ((data[start] as u64) << 48)
        + ((data[start] as u64) << 40) + ((data[start] as u64) << 32)
        + ((data[start] as u64) << 24) + ((data[start] as u64) << 16)
        + ((data[start] as u64) << 8) + ((data[start] as u64) << 0);
}

/// Read from a large byte array a sized vector of bytes
/// Reads the first 4 bytes (u32 length of data)
pub fn read_bytes(startindex:u32, data:&[u8]) -> Vec<u8>
{
    let start = startindex as usize;
    let len = read_u32(startindex, data) as usize;
    let mut v:Vec<u8> = vec!();
    for i in 0..len {
        v.push(data[i+start+4])
    }

    return v;
}