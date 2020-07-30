pub struct StrTable {
    pub(crate) table: Vec<u8>,
}
use std::str;
impl StrTable {
    pub fn get_str(&self, idx: usize) -> &str {
        for (last, value) in self.table[idx..].iter().enumerate() {
            if *value == 0 {
                return str::from_utf8(self.table[idx..last].into()).unwrap();
            }
        }
        str::from_utf8(&[0]).unwrap()
    }
}
