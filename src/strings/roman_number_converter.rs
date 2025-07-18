/// Converts a string containing an roman number to it's respective value.
///
/// This function calculates the value of roman numbers formed by the characters: I, V,
/// X, L, C, D and M. Checking each relation from the character and number it represents
/// and then subtracting or adding it's value to the total.
/// The return can be an error, because `s` can be an invalid roman number.
///
/// # Examples
///
/// ```
/// use rust_utils::strings::roman_number_converter::roman_number_to_int;
///
/// assert_eq!(roman_number_to_int("XII").unwrap(), 12);
/// assert_eq!(roman_number_to_int("MDCLXVI").unwrap(), 1666);
/// assert_eq!(roman_number_to_int("MMMCMXCIX").unwrap(), 3999);
/// ```
///
/// # Arguments
///
/// * `s` - The string slice to convert. The roman number have to contain some of
///   the characters: I, V, X, L, C, D and M.
///
/// # Returns
///
/// An integer (` i32 `) with the respective value from the input.
///
/// # Behavior Notes
///
/// * The implemented characters are I, V, X, L, C, D and M.
/// * Empty strings will have an error return.
/// * This function also validades if the string is a roman number.
///
pub fn roman_number_to_int(s: &str) -> Result<i32, String> {
    use std::collections::HashMap;
    if s.trim().is_empty() {
        return Err("String shouldn't be empty".to_string());
    }
    //more validations
    let map = HashMap::<char, i32>::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let mut total = 0;
    let mut char_iter = s.trim().chars().peekable();
    while let Some(current_char) = char_iter.next() {
        let current = *map.get(&current_char).ok_or(format!(
            "String with the character {current_char} is not a roman digit number."
        ))?;
        if let Some(&next_char) = char_iter.peek() {
            let next = *map.get(&next_char).ok_or(format!(
                "String with the character {next_char} is not a roman digit number."
            ))?;
            if current < next {
                total += next - current;
                char_iter.next();
            } else {
                total += current;
            }
        } else {
            total += current; //last digit
        }
    }

    Ok(total)
}

fn validate_roman_number(s: &str) -> bool {
    if s.trim().is_empty() {
        return false;
    }
    use std::collections::HashMap;
    let map = HashMap::<char, i32>::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let mut char_iter = s.trim().chars().peekable();
    let mut times: i32;
    while let Some(current_char) = char_iter.next() {
        let current = if let Some(&current) = map.get(&current_char) {
            current
        } else {
            return false;
        };
        if let Some(&next_char) = char_iter.peek() {
            let next = if let Some(&next) = map.get(&next_char) {
                next
            } else {
                return false;
            };
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_when_not_roman_number() {
        assert_eq!(
            roman_number_to_int("XQI"),
            Err("String with the character Q is not a roman digit number.".to_string())
        );
    }

    #[test]
    fn test_error_when_empty_string() {
        assert_eq!(
            roman_number_to_int(""),
            Err("String shouldn't be empty".to_string())
        );
        assert_eq!(
            roman_number_to_int("     "),
            Err("String shouldn't be empty".to_string())
        );
    }

    #[test]
    fn test_error_when_invalid_roman_syntax() {
        assert_eq!(
            roman_number_to_int("IIII"),
            Err("Wrong roman number syntax".to_string())
        );
        assert_eq!(
            roman_number_to_int("IC"),
            Err("Wrong roman number syntax".to_string())
        );
        assert_eq!(
            roman_number_to_int("VV"),
            Err("Wrong roman number syntax".to_string())
        );
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(roman_number_to_int("X").unwrap(), 10);
        assert_eq!(roman_number_to_int("V").unwrap(), 5);
        assert_eq!(roman_number_to_int("I").unwrap(), 1);
        assert_eq!(roman_number_to_int("L").unwrap(), 50);
        assert_eq!(roman_number_to_int("C").unwrap(), 100);
        assert_eq!(roman_number_to_int("D").unwrap(), 500);
        assert_eq!(roman_number_to_int("M").unwrap(), 1000);
    }

    #[test]
    fn test_addition() {
        assert_eq!(roman_number_to_int("XII").unwrap(), 12);
        assert_eq!(roman_number_to_int("VI").unwrap(), 6);
        assert_eq!(roman_number_to_int("III").unwrap(), 3);
        assert_eq!(roman_number_to_int("LV").unwrap(), 55);
        assert_eq!(roman_number_to_int("CC").unwrap(), 200);
        assert_eq!(roman_number_to_int("DV").unwrap(), 505);
        assert_eq!(roman_number_to_int("MII").unwrap(), 1002);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(roman_number_to_int("IX").unwrap(), 9);
        assert_eq!(roman_number_to_int("IV").unwrap(), 4);
        assert_eq!(roman_number_to_int("XL").unwrap(), 40);
        assert_eq!(roman_number_to_int("XC").unwrap(), 90);
        assert_eq!(roman_number_to_int("CD").unwrap(), 400);
        assert_eq!(roman_number_to_int("CM").unwrap(), 900);
    }

    #[test]
    fn test_multiple_actions() {
        assert_eq!(roman_number_to_int("MDCLXVI").unwrap(), 1666);
        assert_eq!(roman_number_to_int("MMMCMXCIX").unwrap(), 3999);
    }
}
