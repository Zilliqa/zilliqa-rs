pub fn is_address(address: &str) -> bool {
    is_byte_string(address, 40)
}

pub fn is_private_key(private_key: &str) -> bool {
    is_byte_string(private_key, 64)
}

pub fn is_byte_string(str: &str, len: usize) -> bool {
    let regex = regex::Regex::new(&format!("^[0-9a-fA-F]{{{}}}$", len)).expect("Failed to create the regex for `is_byte_string`");
    let str = str.replace("0x", "");
    regex.is_match(&str)
}

pub fn is_bech32(raw: &str) -> bool {
    let regex =
        regex::Regex::new("^zil1[qpzry9x8gf2tvdw0s3jn54khce6mua7l]{38}$").expect("Failed to create the regex for `is_bech32`");

    regex.is_match(raw)
}

#[cfg(test)]
mod is_byte_string_tests {
    use super::is_byte_string;

    #[test]
    fn is_byte_string_should_return_true_for_a_valid_byte_string_with_correct_size() {
        let str = "1234567890";
        assert!(is_byte_string(str, str.len()))
    }

    #[test]
    fn is_byte_string_should_return_true_for_a_valid_byte_string_with_correct_size_even_if_its_prepended_with_0x() {
        let str = "0x1234567890";
        assert!(is_byte_string(str, str.len() - 2)) // -2 for 0x
    }

    #[test]
    fn is_byte_string_should_return_true_for_a_valid_byte_string_with_correct_size_when_it_contains_letters_a_f() {
        let str = "1234567890aabbccddeeff";
        assert!(is_byte_string(str, str.len()))
    }

    #[test]
    fn is_byte_string_should_return_false_if_size_is_incorrect() {
        let str = "1234567890aabbccddeeff";
        assert_eq!(is_byte_string(str, str.len() - 2), false);
    }

    #[test]
    fn is_byte_string_should_return_false_if_contains_out_of_a_f_characters() {
        let str = "1234567890aabbccddeeffgg";
        assert_eq!(is_byte_string(str, str.len()), false);
    }
}

#[cfg(test)]
mod is_private_key_tests {
    use claim::{assert_gt, assert_lt};

    use super::is_private_key;

    #[test]
    fn is_private_key_should_return_true_for_a_valid_private_key() {
        let str = "e53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89";
        assert!(is_private_key(str));
    }

    #[test]
    fn is_private_key_should_return_true_for_a_valid_private_key_even_if_its_prepended_with_0x() {
        let str = "0xe53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89";
        assert!(is_private_key(str));
    }

    #[test]
    fn should_return_false_for_strings_smaller_than_64_bytes() {
        let str = "0x53d1c3edaff7a7bab5418eb836cf75819a8272b4a1a0f1c7fcf5c3e020b89";
        println!("LEN: {}", str.len());
        assert_lt!(str.len(), 64);
        assert_eq!(is_private_key(str), false);
    }

    #[test]
    fn should_return_false_for_strings_longer_than_64_bytes() {
        let str = "0x3353d1c3edaff7a7bab5418eb836cf75819a8272b4a1a0f1c7fcf5c3e020b89";
        println!("LEN: {}", str.len());
        assert_gt!(str.len(), 64);
        assert_eq!(is_private_key(str), false);
    }
}

#[cfg(test)]
mod is_bech32_tests {
    use crate::util::validation::is_bech32;

    #[test]
    fn is_bech32_should_return_true_for_valid_one() {
        assert!(is_bech32("zil18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2"))
    }

    #[test]
    fn is_bech32_should_return_false_for_invalid_ones() {
        assert!(!is_bech32("liz18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2"));
        assert!(!is_bech32("zil18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2ssaas"));
    }
}
