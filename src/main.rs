use std::io::Read;
use std::{cell::RefCell, fs};
fn main() {
    let b = std::rc::Rc::new(RefCell::new(Vec::<u8>::new()));
    b.borrow_mut().resize(1000, 0);
    let mut file = fs::File::open("./test/elf64_example").unwrap();
    file.read(b.borrow_mut().as_mut()).unwrap();
    println!("{:?}", b);
}
