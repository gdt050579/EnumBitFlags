mod parser;
mod arguments;
mod flags_type;
mod utils;

use proc_macro::*;
use parser::*;
use arguments::*;

extern crate proc_macro;

#[proc_macro_attribute]
pub fn EnumBitFlags(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut a = Arguments::new();
    a.parse(args);
    let mut p = Parser::new(a);
    p.parse(input);
    p.add_methods();
    p.add_operators();
    p.replace_template_parameters();
    return p.stream();    
}
