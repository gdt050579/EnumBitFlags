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
