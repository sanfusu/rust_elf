#[derive(Ordinalize, Debug)]
#[repr(u8)]
pub enum StBindNormal {
    LOCAL = 0,
    GLOABL,
    WEAK,
    LOPROC = 13,
    HIPROC = 15,
}
pub enum StBind {
    Bind(StBindNormal),
    ProcessorSpecific(u8),
}
#[derive(Ordinalize, Debug)]
#[repr(u8)]
pub enum StTypeNormal {
    NOTYPE = 0,
    OBJECT = 1,
    FUNC = 2,
    SECTION = 3,
    FILE = 4,
    LOPROC = 13,
    HIPROC = 15,
}

pub enum StType {
    Type(StTypeNormal),
    ProcessorSpecific(u8),
}
