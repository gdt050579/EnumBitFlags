use crate::flags_type::FlagsType;
use proc_macro::*;

use super::arguments::*;
use std::collections::HashMap;
use std::str::FromStr;
use std::u64;

enum State {
    ExpectVisibility,
    ExpectVisibilityGroup,
    ExpectEnum,
    ExpectName,
    ExpectOpenBrace,
    ExpectFlag,
    ExpectEqual,
    ExpectValue,
    ExpectComma,
}

pub struct Parser {
    output: String,
    name: String,
    state: State,
    args: Arguments,
    last_flag: String,
    visibility: String,
    last_flag_hash: u64,
    map_values: HashMap<u128, String>,
    map_names: HashMap<u64, u128>,
    has_empty_value: bool,
    all_set_bits: u128,
}

impl Parser {
    pub fn new(arguments: Arguments) -> Parser {
        Parser {
            output: String::with_capacity(1024),
            name: String::new(),
            visibility: String::new(),
            state: State::ExpectVisibility,
            args: arguments,
            last_flag: String::new(),
            last_flag_hash: 0,
            map_values: HashMap::with_capacity(8),
            map_names: HashMap::with_capacity(8),
            has_empty_value: false,
            all_set_bits: 0, 
        }
    }
    fn validate_expect_visibility(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token.clone() {
            let txt = ident.to_string();
            if txt == "pub" {
                self.state = State::ExpectVisibilityGroup;                
                self.visibility.push_str("pub");
            } else {
                self.validate_expect_enum(token);
            }
        } else {
            panic!("Expecting an enum or pub keyword but got: {:?}", token);
        }
    }    
    fn validate_expect_visibility_group(&mut self, token: TokenTree) {
        if let TokenTree::Group(g) = token.clone() {
            if g.delimiter() == Delimiter::Parenthesis {
                self.visibility.push_str(token.to_string().as_str());
                self.state = State::ExpectEnum;                
            }
            else {
                panic!("Expecting a visibility group (for example: (crate), (super), ...) but got: {:?}", token);
            }            
        } else {
            self.validate_expect_enum(token);
        }
    }    
    fn validate_expect_enum(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            let txt = ident.to_string();
            if txt == "pub" {
                self.state = State::ExpectEnum;                
                self.visibility.push_str("pub");
                return;
            }
            if txt != "enum" {
                panic!("Expecting an enum keywork but got: {}", txt);
            }
            self.output.push_str(
                r#"
            #[derive(Copy,Clone,Debug)]
            $$(VISIBILITY)$$ struct $$(NAME)$$ { 
                value: $$(BITS)$$ 
            }
            impl $$(NAME)$$ {
            "#,
            );
            self.state = State::ExpectName;
        } else {
            panic!("Expecting an enum keyword but got: {:?}", token);
        }
    }
    fn validate_expect_enum_name(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            self.name = ident.to_string();
            self.state = State::ExpectOpenBrace;
        } else {
            panic!("Expecting the name of the enum but got: {:?}", token);
        }
    }
    fn validate_expect_open_brace(&mut self, token: TokenTree) {
        if let TokenTree::Group(group) = token {
            if group.delimiter() != Delimiter::Brace {
                panic!(
                    "Expecting an open brace '{{' after enum name but got {:?}",
                    group.delimiter()
                );
            }
            self.state = State::ExpectFlag;
            self.parse(group.stream());
        } else {
            panic!(
                "Expecting an open brace '{{' after enum name but got {:?}",
                token
            );
        }
    }
    fn validate_expect_flag(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            self.last_flag = ident.to_string();
            self.last_flag_hash = super::utils::compute_string_hash(self.last_flag.as_bytes());
            if self.map_names.contains_key(&self.last_flag_hash) {
                panic!("Flag {} is used twice in the enum (keep in mind that case is not checked -> \"AB\" and \"ab\" are considered the same variant",self.last_flag.as_str());
            }
            self.output.push_str("\t$$(VISIBILITY)$$ const ");
            self.output.push_str(self.last_flag.as_str());
            self.output.push_str(": $$(NAME)$$ = $$(NAME)$$ { value: ");
            self.state = State::ExpectEqual;
        } else {
            panic!("Expecting the name of a flag but got: {:?}", token);
        }
    }
    fn validate_expect_equal(&mut self, token: TokenTree) {
        if let TokenTree::Punct(punctuation) = token {
            if punctuation.as_char() != '=' {
                panic!(
                    "Expecting equal '=' symbol but got: {}",
                    punctuation.as_char()
                );
            }
            self.state = State::ExpectValue;
        } else {
            panic!("Expecting equal '=' symbol but got: {:?}", token);
        }
    }
    fn validate_expect_value(&mut self, token: TokenTree) {
        if let TokenTree::Literal(l) = token {
            let value = super::utils::string_to_number(l.to_string().as_str());
            if value.is_none() {
                panic!("Expecting an integer value (but got: {})", l.to_string());
            }
            let value = value.unwrap();
            if (value > 0xFF) && (self.args.flags_type == FlagsType::U8) {
                panic!("Enum is set to store data on 8 bits. The value {} is larger than the 0xFF (the maximum value allowed for an 8 bit value). Change the representation by using the attribute bits or change the value !",l.to_string());
            }
            if (value > 0xFFFF) && (self.args.flags_type == FlagsType::U16) {
                panic!("Enum is set to store data on 16 bits. The value {} is larger than the 0xFFFF (the maximum value allowed for an 16 bit value). Change the representation by using the attribute bits or change the value !",l.to_string());
            }
            if (value > 0xFFFFFFFF) && (self.args.flags_type == FlagsType::U32) {
                panic!("Enum is set to store data on 32 bits. The value {} is larger than the 0xFFFFFFFF (the maximum value allowed for an 32 bit value). Change the representation by using the attribute bits or change the value !",l.to_string());
            }
            if (value > 0xFFFFFFFFFFFFFFFF) && (self.args.flags_type == FlagsType::U64) {
                panic!("Enum is set to store data on 64 bits. The value {} is larger than the 0xFFFFFFFFFFFFFFFF (the maximum value allowed for an 64 bit value). Change the representation by using the attribute bits or change the value !",l.to_string());
            }
            if self.map_values.contains_key(&value) {
                panic!(
                    "Flag {} and {} have the same value !",
                    self.map_values.get(&value).unwrap(),
                    self.last_flag.as_str()
                );
            }
            // check for None/Empty value
            if value == 0 {
                if self.args.disable_empty_generation {
                    panic!("You have disabled empty variant generation. As such, no variant with value 0 is possible. Remove the flag `{}` or remove the attribute 'disable_empty_generation'", self.last_flag.as_str());
                }
                if self.args.has_empty_value {
                    panic!("You have already specified a variant for cases where no bits are set in the arguments: '{}'. Either remove variant '{}' or remove the argument 'empty={}'", self.args.none_case.as_str(), self.last_flag.as_str(),self.args.none_case.as_str());
                }
                // all good --> mark has_empty_value so that we don't add one by default
                self.has_empty_value = true;
                self.args.none_case.clear();
                self.args.none_case.push_str(&self.last_flag);
            }                        
            self.map_values.insert(value, self.last_flag.clone());
            self.map_names.insert(self.last_flag_hash, value);
            self.output
                .push_str(&format!("0x{:X}{}", value, self.args.flags_type.as_str()));
            self.output.push_str(" };\n");
            self.state = State::ExpectComma;
            self.all_set_bits |= value;
        } else {
            panic!("Expecting the name of a flag but got: {:?}", token);
        }
    }
    fn validate_expect_comma(&mut self, token: TokenTree) {
        if let TokenTree::Punct(punctuation) = token {
            if punctuation.as_char() != ',' {
                panic!("Expecting ',' separator but got: {}", punctuation.as_char());
            }
            self.state = State::ExpectFlag;
        } else {
            panic!("Expecting ',' separator but got:  {:?}", token);
        }
    }
    pub fn parse(&mut self, input: TokenStream) {
        for token in input.into_iter() {
            match self.state {
                State::ExpectVisibility => self.validate_expect_visibility(token),
                State::ExpectVisibilityGroup => self.validate_expect_visibility_group(token),
                State::ExpectEnum => self.validate_expect_enum(token),
                State::ExpectName => self.validate_expect_enum_name(token),
                State::ExpectOpenBrace => self.validate_expect_open_brace(token),
                State::ExpectFlag => self.validate_expect_flag(token),
                State::ExpectEqual => self.validate_expect_equal(token),
                State::ExpectValue => self.validate_expect_value(token),
                State::ExpectComma => self.validate_expect_comma(token),
            }
        }
    }
    pub fn add_methods(&mut self) {
        // add empty case if needed
        if (!self.has_empty_value) && (self.args.disable_empty_generation == false) {
            self.output.push_str(
                r#"
            $$(VISIBILITY)$$ const $$(EMPTY)$$: $$(NAME)$$ = $$(NAME)$$ { value: 0 };
            "#,
            );
        }
        self.output.push_str(
            r#"        
        /// This function allows creating a new $$(NAME)$$ object from an $$(BITS)$$ value.
        /// This method returns Some($$(NAME)$$) if the parameter `value` is a valid bit configuration, or None otherwise.
        /// 
        /// # Example
        /// ```rust
        /// use EnumBitFlags::EnumBitFlags;
        /// 
        /// #[EnumBitFlags]
        /// enum MyFlags {
        ///     Flag_1 = 0x0001,
        ///     Flag_2 = 0x0002,
        ///     Flag_3 = 0x0004
        /// }
        /// if let Some(y) = MyFlags::from_value(5) {
        ///     println!("{y}");
        /// }
        /// else {
        ///     eprintln!("Could not create value!");
        /// }
        /// ```        
        $$(VISIBILITY)$$ fn from_value(value: $$(BITS)$$) -> Option<Self> {
            $$(DISABLE_EMPTY_CODE)$$
            if value & $$(ALL_SET_BITS)$$ as $$(BITS)$$ == value {
                return Some($$(NAME)$$ { value } );
            }
            None
        } 
       
        /// Checks if all the values in the specified `mask` are set
        /// within the internal value of the current object.
        /// 
        /// # Parameters
        /// 
        /// - `mask`: A `$$(NAME)$$` value representing the mask to check.
        /// 
        /// # Returns
        ///
        /// - `true` if all values in the `mask` are set in the current value.
        /// - `false` otherwise.
        #[inline(always)]
        $$(VISIBILITY)$$ fn contains(&self, mask: $$(NAME)$$) -> bool { 
            return ((self.value & mask.value) == mask.value) && (mask.value!=0);
        }
        /// Checks if at least on of the values in the specified `mask` are set
        /// within the internal value of the current object.
        /// 
        /// # Parameters
        /// 
        /// - `mask`: A `$$(NAME)$$` value representing the mask to check.
        /// 
        /// # Returns
        ///
        /// - `true` if at least one value in the `mask` is set in the current value.
        /// - `false` otherwise.
        #[inline(always)]
        $$(VISIBILITY)$$ fn contains_one(&self, mask: $$(NAME)$$) -> bool { 
            return (self.value & mask.value) != 0 ;
        }
        /// Checks if the current value is not set or if `disable_empty_generation` is `false` and the object is the empty value
        /// # Returns
        ///
        /// - `true` if the current value is not set or is the empty value.
        /// - `false` otherwise.
        #[inline(always)]        
        $$(VISIBILITY)$$ fn is_empty(&self) -> bool { 
            return self.value == 0;
        }
        /// Clears the value or sets it to the empty value.
        /// # Returns
        ///
        /// - `true` if the current value is not set or is the empty value.
        /// - `false` otherwise.
        #[inline(always)]
        $$(VISIBILITY)$$ fn clear(&mut self) {
            self.value = 0;
        }
        /// Removes the values set in the `mask` parameter from the current value.        
        /// 
        /// # Parameters
        /// 
        /// - `mask`: A `$$(NAME)$$` value representing the mask to remove.
        ///         
        #[inline(always)]
        $$(VISIBILITY)$$ fn remove(&mut self, mask: $$(NAME)$$) {
            self.value = self.value - (self.value & mask.value);
        }
        /// Adds the values set in the `mask` parameter to the current value.        
        /// 
        /// # Parameters
        /// 
        /// - `mask`: A `$$(NAME)$$` value representing the mask to add.
        ///         
        #[inline(always)]
        $$(VISIBILITY)$$ fn set(&mut self, mask: $$(NAME)$$) {
            self.value |= mask.value;
        }
        /// Returns the underlying `$$(BITS)$$` value for this object.
        /// 
        /// # Returns
        /// 
        /// - The `$$(BITS)$$` value.
        ///         
        #[inline(always)]
        $$(VISIBILITY)$$ const fn get_value(&self)->$$(BITS)$$ {
            self.value
        }
    }

        "#,
        );
    }
    pub fn add_operators(&mut self) {
        // suport for bitor '|' operations
        self.output.push_str(r#"
        impl std::ops::BitOr for $$(NAME)$$ {
            type Output = Self;        
            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self::Output { $$(NAME)$$ {value: self.value | rhs.value } }            
        }"#);

        // suport for bitorassign '|=' operations
        self.output.push_str(
            r#"
        impl std::ops::BitOrAssign for $$(NAME)$$ {   
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: Self)  { self.value |= rhs.value; }            
        }"#,
        );

        // suport for bitand '&' operations
        self.output.push_str(r#"
        impl std::ops::BitAnd for $$(NAME)$$ {
            type Output = Self;        
            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self::Output { $$(NAME)$$ {value: self.value & rhs.value } }            
        }"#);

        // suport for bitandassign '&=' operations
        self.output.push_str(
            r#"
        impl std::ops::BitAndAssign for $$(NAME)$$ {   
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: Self)  { self.value &= rhs.value; }            
        }"#,
        );

        // suport for partial EQ '==' and '!=' operations
        self.output.push_str(
            r#"
        impl std::cmp::Eq for $$(NAME)$$ { }
        impl std::cmp::PartialEq for $$(NAME)$$ {   
            #[inline(always)]
            fn eq(&self, other: &Self) -> bool  { self.value == other.value }            
        }"#,
        );

        // suport default
        self.output.push_str(
            r#"
        impl std::default::Default for $$(NAME)$$ {
            fn default() -> Self { $$(NAME)$$ { value: 0 } }
        }"#,
        );

        // suport for Display
        self.output.push_str(
            r#"
        impl std::fmt::Display for $$(NAME)$$ {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "$$(NAME)$$ (")?;
                if self.value == 0 {
                    write!(f,"$$(EMPTY)$$)")?;
                } else {
                    let mut first = true;
                "#,
        );
        self.output.push_str("\n");
        // sort all items
        let mut enum_variants: Vec<(&u128, &String)> = self.map_values.iter().collect();
        enum_variants.sort_by(|e1, e2| e1.1.cmp(e2.1));
        for (value, name) in enum_variants {
            self.output.push_str("\t\tif (self.value & ");
            self.output
                .push_str(&format!("0x{}{}", value, self.args.flags_type.as_str()));
            self.output.push_str(") == ");
            self.output
                .push_str(&format!("0x{}{}", value, self.args.flags_type.as_str()));
            self.output.push_str(
                " { if !first { write!(f,\" | \")?; } else { first = false; }; write!(f, \"",
            );
            self.output.push_str(name);
            self.output.push_str("\")?; }\n");
        }
        self.output.push_str(
            r#"
                    write!(f,")")?;
                }
                Ok(())            
            }
        }
        "#,
        );
    }
    pub fn replace_template_parameters(&mut self) {
        self.output = self.output.replace("$$(NAME)$$", self.name.as_str());
        self.output = self
            .output
            .replace("$$(EMPTY)$$", self.args.none_case.as_str());
        self.output = self
            .output
            .replace("$$(BITS)$$", self.args.flags_type.as_str());
        self.output = self
            .output
            .replace("$$(VISIBILITY)$$", self.visibility.as_str());
        self.output = self
            .output
            .replace("$$(ALL_SET_BITS)$$", self.all_set_bits.to_string().as_str());
        if self.args.disable_empty_generation {
            self.output = self
                .output
                .replace("$$(DISABLE_EMPTY_CODE)$$", "if value==0 { return None; };")
        }
        else {
            self.output = self
                .output
                .replace("$$(DISABLE_EMPTY_CODE)$$", "")
        }
    }
    pub fn stream(self) -> TokenStream {
        if self.args.debug_mode {
            println!("Debug mode enable ==> Printing the output !");
            println!("=====================================================================================================");            
            println!("{}",self.output.as_str());
            println!("=====================================================================================================");            
        }
        return TokenStream::from_str(self.output.as_str())
            .expect("Failed to parse string as tokens");
    }
}
