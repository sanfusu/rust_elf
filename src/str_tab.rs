use core::ops::Index;


/// 直接通过索引来获取字符串表中的数值
/// # Example
/// ```
/// use elf::StrTab;
///
/// let tmp = ['a' as u8,'b' as u8,'c' as u8,'\0' as u8];
/// let str_tab = StrTab::new(&tmp);
/// assert_eq!(&str_tab[0], "abc");
/// assert_eq!(&str_tab[1], "bc");
/// assert_eq!(&str_tab[3], "");
///
/// let tmp_witout_null = ['a' as u8,'b' as u8,'c' as u8];
/// let str_tab_without_null = StrTab::new(&tmp_witout_null);
/// assert_eq!(&str_tab_without_null[0], "abc");
/// assert_eq!(&str_tab_without_null[1], "bc");
/// assert_eq!(&str_tab_without_null[3], "");
///
/// let tmp_split_null = ['a' as u8,'\0' as u8,'c' as u8];
/// let str_tab_split_null = StrTab::new(&tmp_split_null);
/// assert_eq!(&str_tab_split_null[0], "a");
/// assert_eq!(&str_tab_split_null[1], "");
/// assert_eq!(&str_tab_split_null[3], "");
/// ```
pub struct StrTab<'a> {
    src: &'a [u8],
}

impl<'a> StrTab<'a> {
    pub fn new(src: &'a [u8]) -> Self {
        Self { src }
    }
}

impl Index<usize> for StrTab<'_> {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        let src = &self.src[index..];
        let end = src.iter().position(|&x| x == 0).unwrap_or(src.len());
        unsafe { core::str::from_utf8_unchecked(&src[..end]) }
    }
}
