use proc_macro::*;
use super::arguments::*;
use std::str::FromStr;

enum State {
    ExpectEnum,
    ExpectName,
    ExpectOpenBrace,
    ExpectFlag,
    ExpectEqual,
    ExpectValue,
    ExpectSeparatorOrCloseBrace,
}

pub struct Parser {
    output: String,
    name: String,
    state: State,
    args: Arguments,
}

impl Parser {
    pub fn new(arguments: Arguments) -> Parser {
        Parser {
            output: String::with_capacity(1024),
            name: String::new(),
            state: State::ExpectEnum,
            args: arguments
        }
    }
    fn validate_expect_enum(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            let txt = ident.to_string();
            if (txt!="enum") && (txt!="flags") {
                panic!("Expecting an enum keywork but got: {}",txt);
            }
            self.output.push_str(r#"
            #[derive(Copy,Clone,Debug)]
            struct $$(NAME)$$ { 
                value: $$(BITS)$$ 
            }
            impl $$(NAME)$$ {
                pub const $$(EMPTY)$$: $$(NAME)$$ = $$(NAME)$$ { value: 0 };
            "#);
            self.state = State::ExpectName;
        } else {
            panic!("Expecting an enum keyword but got: {:?}",token);
        }
    }
    fn validate_expect_enum_name(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            self.name = ident.to_string();           
            self.state = State::ExpectOpenBrace;
        } else {
            panic!("Expecting the name of the enum but got: {:?}",token);
        }
    }    
    fn validate_expect_open_brace(&mut self, token: TokenTree) {
        if let TokenTree::Group(group) = token {
            if group.delimiter() != Delimiter::Brace {
                panic!("Expecting an open brace '{{' after enum name but got {:?}",group.delimiter());
            }
            self.state = State::ExpectFlag;
            self.parse(group.stream());
        } else {
            panic!("Expecting an open brace '{{' after enum name but got {:?}",token);
        }
    } 
    fn validate_expect_flag(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            self.output.push_str("\tpub const ");         
            self.output.push_str(ident.to_string().as_str());
            self.output.push_str(": $$(NAME)$$ = $$(NAME)$$ { value: ");         
            self.state = State::ExpectEqual;
        } else {
            panic!("Expecting the name of a flag but got: {:?}",token);
        }
    } 
    fn validate_expect_equal(&mut self, token: TokenTree) {
        if let TokenTree::Punct(punctuation) = token {
            if punctuation.as_char() != '=' {
                panic!("Expecting equal '=' symbol but got: {}",punctuation.as_char());
            }
            self.state = State::ExpectValue;
        } else {
            panic!("Expecting equal '=' symbol but got: {:?}",token);
        }
    } 
    fn validate_expect_value(&mut self, token: TokenTree) {
        if let TokenTree::Literal(l) = token {
            let text = l.to_string();
            let mut text_str = text.as_str();
            if let Some(pos) = text_str.find(|ch| (ch=='u' || (ch=='i'))) {
                text_str = &text_str[..pos];
            }
            //println!("Text={}, str={}",&text,text_str);
            let value = text_str.parse::<u32>();
            if value.is_err() {
                panic!("Expecting an integer value (but got: {})",text);
            }
            self.output.push_str(&text);              
            self.output.push_str(" };\n");       
            self.state = State::ExpectSeparatorOrCloseBrace;
        } else {
            panic!("Expecting the name of a flag but got: {:?}",token);
        }
    } 
    fn validate_expect_separator_or_close_braket(&mut self, token: TokenTree) {
        if let TokenTree::Punct(punctuation) = token {
            if punctuation.as_char() != ',' {
                panic!("Expecting ',' separator but got: {}",punctuation.as_char());
            }     
            self.state = State::ExpectFlag;
        } else {
            panic!("Expecting ',' separator but got:  {:?}",token);
        }
    }     
    pub fn parse(&mut self, input: TokenStream) {
        for token in input.into_iter() {
            match self.state {
                State::ExpectEnum => self.validate_expect_enum(token),
                State::ExpectName => self.validate_expect_enum_name(token),
                State::ExpectOpenBrace => self.validate_expect_open_brace(token),
                State::ExpectFlag => self.validate_expect_flag(token),
                State::ExpectEqual => self.validate_expect_equal(token),
                State::ExpectValue => self.validate_expect_value(token),
                State::ExpectSeparatorOrCloseBrace => self.validate_expect_separator_or_close_braket(token),
            }
        }    
    }
    pub fn add_methods(&mut self) {
        self.output.push_str(r#"
        pub fn contains(&self, obj: $$(NAME)$$) -> bool { 
            return ((self.value & obj.value) == obj.value) && (obj.value!=0);
        }
        pub fn contains_one(&self, obj: $$(NAME)$$) -> bool { 
            return (self.value & obj.value) != 0 ;
        }        
        pub fn is_empty(&self) -> bool { 
            return self.value == 0;
        }
        pub fn clear(&mut self) {
            self.value = 0;
        }
        pub fn remove(&mut self, obj: $$(NAME)$$) {
            self.value = self.value - (self.value & obj.value);
        }
        pub fn set(&mut self, obj: $$(NAME)$$) {
            self.value |= obj.value;
        }
    }

        "#);
    }
    pub fn add_operators(&mut self) {
        // suport for bitor '|' operations
        self.output.push_str(r#"
        impl std::ops::BitOr for $$(NAME)$$ {
            type Output = Self;        
            #[inline]
            fn bitor(self, rhs: Self) -> Self::Output { $$(NAME)$$ {value: self.value | rhs.value } }            
        }"#);
        // suport for bitorassign '|=' operations
        self.output.push_str(r#"
        impl std::ops::BitOrAssign for $$(NAME)$$ {   
            #[inline]
            fn bitor_assign(&mut self, rhs: Self)  { self.value |= rhs.value; }            
        }"#);        
    }
    pub fn replace_template_parameters(&mut self) {
        self.output = self.output.replace("$$(NAME)$$", self.name.as_str());
        self.output = self.output.replace("$$(EMPTY)$$", self.args.none_case.as_str());
        self.output = self.output.replace("$$(BITS)$$", self.args.flags_type.as_str());
    }
    pub fn stream(self) -> TokenStream {
        //println!("result = {}",self.output.as_str());
        return TokenStream::from_str(self.output.as_str()).expect("Failed to parse string as tokens");
    }
}


