#[derive(Debug)]
pub struct StrTable<'a> {
    pub(crate) table: &'a [u8],
}
use core::str;
impl<'a> StrTable<'a> {
    pub fn get_str(&self, idx: usize) -> &str {
        for (last, value) in self.table[idx..].iter().enumerate() {
            if *value == 0 {
                return str::from_utf8(self.table[idx..last].into()).unwrap();
            }
        }
        str::from_utf8(&[0]).unwrap()
    }
}
