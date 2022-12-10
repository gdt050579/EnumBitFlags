use proc_macro::*;
pub enum FlagsType
{
    U8,
    U16,
    U32,
    U64,
    U128
}
pub struct Arguments
{
    pub flags_type: FlagsType,
    pub none_case: String
}

impl Arguments {
    pub fn new()-> Arguments {
        Arguments {
            flags_type: FlagsType::U32,
            none_case: String::from("None")
        }
    }
    pub fn parse(&mut self, input: TokenStream) {

    }
}