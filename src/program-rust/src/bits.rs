pub fn read_u32(start:u32, data:&[u8]) -> u32
{
    return (data[start] << 24) + (data[start+1] << 16)
        + (data[start+2] << 8) + (data[start+3] << 0);
}

pub fn read_u64(start:u32, data:&[u8]) -> u64
{
    return (data[start] << 56 as u64) + (data[start] << 48 as u64)
        + (data[start] << 40 as u64) + (data[start] << 32 as u64)
        + (data[start] << 24 as u64) + (data[start] << 16)
        + (data[start] << 8) + (data[start] << 0);
}

/// Read from a large byte array a sized vector of bytes
/// Reads the first 4 bytes (u32 length of data)
pub fn read_bytes(start:u32, data:&[u8]) -> Vec<u8>
{
    let len:u32 = read_u32(start, data);
    let mut v:Vec<u8> = vec!();
    for i in 0..len {
        v.push(data[i+start+4])
    }

    return v;
}