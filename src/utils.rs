// a valid enum should start with a letter or "_" character and should contain letters,numbers of underline character

const LOWER_CASE_TABLE: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 97, 98, 99, 100, 101, 102, 103,
    104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
    91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
    131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149,
    150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168,
    169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187,
    188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206,
    207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225,
    226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244,
    245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
];

pub fn compute_string_hash(buf: &[u8]) -> u64 {
    // use FNV algorithm ==> https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
    if buf.len() == 0 {
        return 0;
    }
    let mut hash = 0xcbf29ce484222325u64;
    let mut idx = 0usize;
    while idx < buf.len() {
        hash = hash ^ (LOWER_CASE_TABLE[buf[idx] as usize] as u64);
        //hash = hash * 0x00000100000001B3u64;
        hash = hash.wrapping_mul(0x00000100000001B3u64);
        idx += 1;
    }
    return hash;
}
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
            return (suffix[0] == b'1') && (suffix[1] == b'2') && (suffix[2] == b'8');
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
