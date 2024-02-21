#[macro_export]
macro_rules! debug {
    ($($e:expr),+) => {
        {
            #[cfg(debug_assertions)]
            {
                dbg!($($e),+)
            }
            #[cfg(not(debug_assertions))]
            {
                ($($e),+)
            }
        }
    };
}

/// List of main menu inputs.
///
/// # Attributes
/// * `Settings` - Option for settings page, for any needed configurations.
/// * `Minesweeper` - Option to enter minesweeper game.
pub enum MainMenuOptions {
    Settings,
    Minesweeper,
}

/// Module for input validation, mainly through regex although more sophisticated validation may be added in the future.
pub mod input_validation {
    use {once_cell::sync::Lazy, regex::Regex};

    /// List of types for check function for input validation.
    ///
    /// # Attributes
    /// * `Menu` - check for menu inputs, only allows positive integers.
    #[derive(Debug, PartialEq)]
    pub enum InValType {
        Menu,
    }

    /// Input validation for strings using regex, returns bool of match results.
    ///
    /// # Attributes
    /// * `in_str` - string to be checked against format type.
    /// * `in_type` - enum of format type from InValType list.
    ///
    /// # Example
    /// ```
    /// use mair_minigames::input_validation::{check,InValType};
    /// let regex_result = check("1",InValType::Menu);
    /// if(regex_result){
    ///     println!("result is valid");
    /// }else{
    ///     println!("result is invalid");
    /// }
    /// ```
    pub fn check(in_str: &str, in_type: InValType) -> bool {
        // match in_type against static regex (so it only compiles once)
        static NUM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([\d]+)$").unwrap());

        match in_type {
            InValType::Menu => NUM_RE.is_match(in_str),
            // _ => false,
        }
    }
}
