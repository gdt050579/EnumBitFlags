use proc_macro::*;
pub enum FlagsType
{
    U8,
    U16,
    U32,
    U64,
    U128
}
enum State {
    ExpectKey,
    ExpectEqual,
    ExpectValue,
    ExpectComma
}
pub struct Arguments
{
    pub flags_type: FlagsType,
    pub none_case: String,
    state: State,
    key: String,
    value: String,
}

impl Arguments {
    pub fn new()-> Arguments {
        Arguments {
            flags_type: FlagsType::U32,
            none_case: String::from("None"),
            state: State::ExpectKey,
            key: String::new(),
            value: String::new()
        }
    }
    fn validate_expect_key(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            self.key = ident.to_string();           
            self.state = State::ExpectEqual;
        } else {
            panic!("Expecting a key (a-zA-Z0-9) but got: `{}`",token.to_string());
        }
    }
    fn validate_expect_equal(&mut self, token: TokenTree) {
        if let TokenTree::Punct(punctuation) = token {
            if (punctuation.as_char() != '=') || (punctuation.as_char() != ':') {
                panic!("Expecting asignamne ('=' or ':') symbol but got: {}",punctuation.as_char());
            }
            self.state = State::ExpectValue;
        } else {
            panic!("Expecting asignamne ('=' or ':') symbol but got: {}",token.to_string());
        }        
    }   
    fn validate_expect_value(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            self.value = ident.to_string();           
            self.state = State::ExpectComma;
        } else {
            panic!("Expecting a value (a-zA-Z0-9) but got: `{}`",token.to_string());
        }       
    } 
    fn validate_expect_comma(&mut self, token: TokenTree) {
        if let TokenTree::Punct(punctuation) = token {
            if punctuation.as_char() != ',' {
                panic!("Expecting delimiter (',' comma) symbol but got: {}",punctuation.as_char());
            }
            self.state = State::ExpectKey;
        } else {
            panic!("Expecting delimiter (',' comma) symbol but got:{}",token.to_string());
        }         
    } 
    pub fn parse(&mut self, input: TokenStream) {
        for token in input.into_iter() {
            match self.state {
                State::ExpectKey => self.validate_expect_key(token),
                State::ExpectEqual => self.validate_expect_equal(token),
                State::ExpectValue => self.validate_expect_value(token),
                State::ExpectComma => self.validate_expect_comma(token),
            }
        } 
    }
}