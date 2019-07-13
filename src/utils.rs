
/* some fast conversion */
pub fn as_u64(array: &[u8]) -> u64 {
    ((array[7] as u64) << 8*7) |
    ((array[6] as u64) << 8*6) |
    ((array[5] as u64) << 8*5) |
    ((array[4] as u64) << 8*4) |    
    ((array[3] as u64) << 8*3) |
    ((array[2] as u64) << 8*2) |
    ((array[1] as u64) <<   8) |
    ((array[0] as u64) <<   0)
}

pub fn as_u16(array: &[u8]) -> u16 {
    ((array[1] as u16) <<   8) | ((array[0] as u16) <<   0)
}

pub fn as_i32(array: &[u8]) -> i32 {
    ((array[3] as i32) << 8*3) |
    ((array[2] as i32) << 8*2) |
    ((array[1] as i32) <<   8) |
    ((array[0] as i32) <<   0)
}