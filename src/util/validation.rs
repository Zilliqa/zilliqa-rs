pub fn is_tx_hash(tx_hash: &str) -> bool {
    is_byte_string(tx_hash, 64)
}

/// Checks if a given string is a valid byte string of a specified length.
pub fn is_byte_string(str: &str, len: usize) -> bool {
    let regex = regex::Regex::new(&format!("^[0-9a-fA-F]{{{}}}$", len)).expect("Failed to create the regex for `is_byte_string`");
    let str = str.replace("0x", "");
    regex.is_match(&str)
}

#[cfg(test)]
mod is_tx_hash_tests {
    use super::is_tx_hash;

    #[test]
    fn is_tx_hash_should_return_true_for_a_valid_hash() {
        let hash = "bdadfd994f452df803cc223d1f417b02830ac96dbe5edad1b9f8d58613f95206";
        assert!(is_tx_hash(hash));

        let hash = "bdadfd994f452df803cc223d1f417b02830ac96dbe5edad1b9f8d58613f95206".to_ascii_uppercase();
        assert!(is_tx_hash(&hash));
    }

    #[test]
    fn is_tx_hash_should_return_false_for_a_invalid_hash() {
        let hash = "bdadfd994f452df803cc223d102830ac96dbe5edad1b9f8d58613f95206";
        assert!(!is_tx_hash(hash));
    }
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
