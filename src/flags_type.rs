#[derive(Clone,Copy,PartialEq)]
#[repr(u8)]
pub enum FlagsType {
    U8,
    U16,
    U32,
    U64,
    U128,
}

impl FlagsType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FlagsType::U8 => return "u8",
            FlagsType::U16 => return "u16",
            FlagsType::U32 => return "u32",
            FlagsType::U64 => return "u64",
            FlagsType::U128 => return "u128"
        }
    }
}