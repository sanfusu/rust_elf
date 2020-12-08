pub trait MetaData: Sized {
    fn as_slice<'a>(&'a self) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
    fn as_mut_slice<'a>(&'a mut self) -> &'a mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self as *mut Self as *mut u8,
                std::mem::size_of::<Self>(),
            )
        }
    }

    fn read_from_slice(&mut self, src: &[u8]) {
        self.as_mut_slice().copy_from_slice(src);
    }
}

pub trait Ehdr {}
pub trait Section {
    fn name(&self) -> String;
}
pub trait Elf {
    fn sections<T: Section>(&self) -> Vec<T>;
    fn programs(&self);
}
