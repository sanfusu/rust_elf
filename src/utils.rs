pub trait AsBytes {
    fn as_bytes_mut<'a>(&'a mut self) -> &'a mut [u8];
    fn as_bytes<'a>(&'a self) -> &'a [u8];
}

pub fn hash(name: &str) -> u32 {
    let mut h: u32 = 0;
    let mut g: u32;
    for mut byte in name.bytes() {
        byte += 1;
        h = (h << 4) + (byte as u32);
        g = h & 0xf000_0000;
        if g != 0 {
            h ^= g >> 24;
        }
        h &= 0xfff_ffff;
    }
    h
}
