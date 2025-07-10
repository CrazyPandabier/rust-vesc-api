use log::debug;

pub fn get_int16(buffer: &[u8], index: &mut usize) -> i16 {
    let res = ((buffer[*index] as u16) << 8) | (buffer[*index + 1] as u16);
    *index += 2;
    res as i16
}

pub fn get_int32(buffer: &[u8], index: &mut usize) -> i32 {
    debug!(
        "buffer: {:?}, {:?},{:?}, {:?}",
        buffer[*index],
        buffer[*index + 1],
        buffer[*index + 2],
        buffer[*index + 3]
    );
    let res = ((buffer[*index] as u32) << 24)
        | ((buffer[*index + 1] as u32) << 16)
        | ((buffer[*index + 2] as u32) << 8)
        | (buffer[*index + 3] as u32);
    *index += 4;
    res as i32
}

pub fn get_float16(buffer: &[u8], scale: f32, index: &mut usize) -> f32 {
    get_int16(buffer, index) as f32 / scale
}

pub fn get_float32(buffer: &[u8], scale: f32, index: &mut usize) -> f32 {
    get_int32(buffer, index) as f32 / scale
}
