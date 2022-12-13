use super::flags_type::FlagsType;
use super::utils;
use proc_macro::*;

enum State {
    ExpectKey,
    ExpectEqual,
    ExpectValue,
    ExpectComma,
}
pub struct Arguments {
    pub flags_type: FlagsType,
    pub none_case: String,
    pub has_empty_value: bool,
    pub disable_empty_generation: bool,
    state: State,
    key: String,
    value: String,
}

impl Arguments {
    pub fn new() -> Arguments {
        Arguments {
            flags_type: FlagsType::U32,
            none_case: String::from("None"),
            state: State::ExpectKey,
            key: String::new(),
            value: String::new(),
            has_empty_value: false, 
            disable_empty_generation: false,
        }
    }
    fn validate_bits_attribute(&mut self) {
        match self.value.as_str() {
            "8" => self.flags_type = FlagsType::U8,
            "16" => self.flags_type = FlagsType::U16,
            "32" => self.flags_type = FlagsType::U32,
            "64" => self.flags_type = FlagsType::U64,
            "128" => self.flags_type = FlagsType::U128,
            _ => {
                panic!("The value for `bits` attribute can be 8,16,32,64 or 128. Provided value was: {}",self.value.as_str());
            }
        }
    }
    fn validate_empty_attribute(&mut self) {
        if !utils::validate_enum_variant_name(self.value.as_str()) {
            panic!("Invalid name for the empty case. A valid name should contains letters, numbers or underline and must not start with a number.");
        }
        self.none_case.clear();
        self.none_case.push_str(self.value.as_str());
        self.has_empty_value = true;
    }  
    fn validate_noempty_attribute(&mut self) {
        match self.value.as_str() {
            "true" => self.disable_empty_generation = true,
            "yes" => self.disable_empty_generation = true,
            "false" => self.disable_empty_generation = false,
            "no" => self.disable_empty_generation = false,
            _ => {
                panic!("The value for `disable_empty_generation` attribute can only be 'true' or 'false'. Provided value was: {}",self.value.as_str());
            }
        }
    }
  
    fn validate_key_value_pair(&mut self) {
        match self.key.as_str() {
            "bits" => self.validate_bits_attribute(),
            "empty" => self.validate_empty_attribute(),
            "disable_empty_generation" => self.validate_noempty_attribute(),
            _ => {
                panic!("Unknown attribute `{}` for EnumBitFlags. Accepted attributes are 'bits' , 'empty' and 'disable_empty_generation' !",self.key.as_str());
            }
        }
    }
    fn validate_expect_key(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            self.key = ident.to_string();
            self.state = State::ExpectEqual;
        } else {
            panic!(
                "Expecting a key (a-zA-Z0-9) but got: `{}`",
                token.to_string()
            );
        }
    }
    fn validate_expect_equal(&mut self, token: TokenTree) {
        if let TokenTree::Punct(punctuation) = token {
            if (punctuation.as_char() != '=') && (punctuation.as_char() != ':') {
                panic!(
                    "Expecting asignamne ('=' or ':') symbol but got: {}",
                    punctuation.as_char()
                );
            }
            self.state = State::ExpectValue;
        } else {
            panic!(
                "Expecting asignamne ('=' or ':') symbol but got: {}",
                token.to_string()
            );
        }
    }
    fn validate_expect_value(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            self.value = ident.to_string();
            self.state = State::ExpectComma;
        } else if let TokenTree::Literal(literal) = token {
            self.value = literal.to_string();
            self.state = State::ExpectComma;
        } else {
            panic!(
                "Expecting a value (a-zA-Z0-9) but got: `{}`",
                token.to_string()
            );
        }
        self.validate_key_value_pair();
    }
    fn validate_expect_comma(&mut self, token: TokenTree) {
        if let TokenTree::Punct(punctuation) = token {
            if punctuation.as_char() != ',' {
                panic!(
                    "Expecting delimiter (',' comma) symbol but got: {}",
                    punctuation.as_char()
                );
            }
            self.state = State::ExpectKey;
        } else {
            panic!(
                "Expecting delimiter (',' comma) symbol but got:{}",
                token.to_string()
            );
        }
    }
    pub fn parse(&mut self, input: TokenStream) {
        for token in input.into_iter() {
            // println!("arg_token: {:?}", token);
            match self.state {
                State::ExpectKey => self.validate_expect_key(token),
                State::ExpectEqual => self.validate_expect_equal(token),
                State::ExpectValue => self.validate_expect_value(token),
                State::ExpectComma => self.validate_expect_comma(token),
            }
        }
    }
}
