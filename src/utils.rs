// a valid enum should start with a letter or "_" character and should contain letters,numbers of underline character
pub fn validate_enum_variant_name(name: &str) -> bool {
    if name.len() == 0 {
        return false;
    }
    for (index, ch) in name.char_indices() {
        if ((ch >= 'A') && (ch <= 'Z')) || ((ch >= 'a') && (ch <= 'z')) {
            continue;
        }
        if (ch >= '0') && (ch <= '9') {
            if index == 0 {
                return false;
            } else {
                continue;
            }
        }
        if ch == '_' {
            continue;
        }
        // else --> invalid character --> exit
        return false;
    }
    return true;
}

const SUFFIX_MARKER: u8 = 254;
const UNKNWON_CHAR: u8 = 255;

#[inline]
fn validate_suffix(suffix: &[u8]) -> bool {
    match suffix.len() {
        1 => {
            return suffix[0] == b'8';
        } /* u8 or i8 */
        2 => {
            return ((suffix[0] == b'1') && (suffix[1] == b'6'))
                || ((suffix[0] == b'3') && (suffix[1] == b'2'))
                || ((suffix[0] == b'6') && (suffix[1] == b'4'));
        } /* u16/u32/u64 or i16/i32/i64 */
        3 => {
            return ((suffix[0] == b'1') && (suffix[1] == b'2') && (suffix[2] == b'8'));
        } /* u128 or i128 */
        _ => {
            return false;
        }
    }
}

#[inline]
fn char_to_hex(ch: u8) -> u8 {
    match ch {
        b'0'..=b'9' => {
            return ch - 48;
        }
        b'a'..=b'f' => {
            return ch - b'a' + 10;
        }
        b'A'..=b'F' => {
            return ch - b'A' + 10;
        }
        b'u' => {
            return SUFFIX_MARKER;
        }
        b'i' => {
            return SUFFIX_MARKER;
        }
        _ => {
            return UNKNWON_CHAR;
        }
    }
}

#[inline]
fn char_to_dec(ch: u8) -> u8 {
    match ch {
        b'0'..=b'9' => {
            return ch - 48;
        }
        b'u' => {
            return SUFFIX_MARKER;
        }
        b'i' => {
            return SUFFIX_MARKER;
        }
        _ => {
            return UNKNWON_CHAR;
        }
    }
}

#[inline]
fn char_to_oct(ch: u8) -> u8 {
    match ch {
        b'0'..=b'7' => {
            return ch - 48;
        }
        b'u' => {
            return SUFFIX_MARKER;
        }
        b'i' => {
            return SUFFIX_MARKER;
        }
        _ => {
            return UNKNWON_CHAR;
        }
    }
}

#[inline]
fn char_to_bin(ch: u8) -> u8 {
    match ch {
        b'0' => {
            return 0;
        }
        b'1' => {
            return 1;
        }
        b'u' => {
            return SUFFIX_MARKER;
        }
        b'i' => {
            return SUFFIX_MARKER;
        }
        _ => {
            return UNKNWON_CHAR;
        }
    }
}

fn text_to_number(value: &[u8], base: u128, convert: fn(u8) -> u8) -> Option<u128> {
    let mut result = 0u128;
    let len = value.len();
    let mut index = 0usize;
    while index < len {
        let v = convert(value[index]);
        if v == UNKNWON_CHAR {
            return None;
        }
        if v == SUFFIX_MARKER {
            if validate_suffix(&value[index + 1..]) == false {
                return None;
            }
            break;
        }
        let res = result * base + (v as u128);
        // check overflow cases
        if res < result {
            return None;
        }
        result = res;
        index += 1;
    }
    return Some(result);
}
pub fn string_to_number(value: &str) -> Option<u128> {
    let b = value.as_bytes();
    if b.len() == 0 {
        return None;
    };
    if (b.len() >= 2) && (b[0] == b'0') {
        match b[1] {
            b'x' => {
                return text_to_number(&b[2..], 16, char_to_hex);
            }
            b'o' => {
                return text_to_number(&b[2..], 8, char_to_oct);
            }
            b'b' => {
                return text_to_number(&b[2..], 2, char_to_bin);
            }
            _ => {}
        }
    }
    return text_to_number(&b, 10, char_to_dec);
}
