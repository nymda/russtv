pub fn clamp(val: i32) -> i32{
    if val < 0 { return 0; }
    if val > 255 { return 255; }
    return val;
}

pub fn Y(R: u8, G: u8, B: u8) -> u8{
    return clamp(((66 * (R as i32) + 129 * (G as i32) + 25 * (B as i32) + 128) >> 8) + 16) as u8;
}

pub fn U(R: u8, G: u8, B: u8) -> u8{
    return clamp(((-38 * (R as i32) - 74 * (G as i32) + 112 * (B as i32) + 128) >> 8) + 128) as u8;
}

pub fn V(R: u8, G: u8, B: u8) -> u8{
    return clamp(((112 * (R as i32) - 94 * (G as i32) - 18 * (B as i32) + 128) >> 8) + 128) as u8;
}