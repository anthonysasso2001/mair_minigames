/// Tests for Regex input validation functions in src/lib.rs
#[cfg(test)]
mod regex_tests {
    mod number_checks {
        use mair_minigames::input_validation::{check, InValType};

        /// Check if number returns true with menu regex string
        #[test]
        fn number_integer_true() {
            let regex_output = check("123", InValType::Menu);
            assert_eq!(true, regex_output)
        }

        /// Check if alphanumeric returns false with menu regex check
        #[test]
        fn alphanumeric_integer_false() {
            let regex_output = check("abc123", InValType::Menu);
            assert_eq!(false, regex_output)
        }
    }
}
