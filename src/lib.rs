use proc_macro::*;

extern crate proc_macro;

enum State {
    ExpectEnum,
    ExpectName,
    ExpectOpenBrace,
    ExpectFlag,
    ExpectEqual,
    ExpectValue,
    ExpectSeparatorOrCloseBrace,
}

struct Parser {
    output: String,
    name: String,
    state: State,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            output: String::new(),
            name: String::new(),
            state: State::ExpectEnum,
        }
    }
    fn validate_expect_enum(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            let txt = ident.to_string();
            if (txt!="enum") && (txt!="flags") {
                panic!("Expecting an enum keywork but got: {}",txt);
            }
            self.output.push_str("struct ");
            self.state = State::ExpectName;
        } else {
            panic!("Expecting an enum keyword but got: {:?}",token);
        }
    }
    fn validate_expect_enum_name(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            self.name = ident.to_string();           
            self.output.push_str(self.name.as_str());
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
            self.output.push_str(" {");
            self.state = State::ExpectFlag;
            self.parse(group.stream());
        } else {
            panic!("Expecting an open brace '{{' after enum name but got {:?}",token);
        }
    } 
    fn validate_expect_flag(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            self.name = ident.to_string();  
            self.output.push_str("\n");         
            self.output.push_str(self.name.as_str());
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
            self.output.push_str(" = ");         
            self.state = State::ExpectValue;
        } else {
            panic!("Expecting the name of a flag but got: {:?}",token);
        }
    } 
    fn validate_expect_value(&mut self, token: TokenTree) {
        if let TokenTree::Literal(l) = token {
            let text = l.to_string();
            let mut text_str = text.as_str();
            if let Some(pos) = text_str.find(|ch| (ch=='u' || (ch=='i'))) {
                text_str = &text_str[..pos];
            }
            println!("Text={}, str={}",&text,text_str);
            let value = text_str.parse::<u32>();
            if value.is_err() {
                panic!("Expecting an integer value (but got: {})",text);
            }
            self.output.push_str(&text);                     
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
            self.output.push_str(", \n");         
            self.state = State::ExpectFlag;
        } else {
            panic!("Expecting ',' separator but got:  {:?}",token);
        }
    }     
    fn parse(&mut self, input: TokenStream) {
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
}

#[proc_macro_attribute]
pub fn EnumBitFlags(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut p = Parser::new();
    p.parse(input);
    println!("{}",p.output.as_str());
    return TokenStream::new();
}
