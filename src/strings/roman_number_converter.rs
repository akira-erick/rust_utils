pub fn roman_number_to_int(input: &str) -> Result<i32, &str> {
    
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn should_convert_string_to_int(){
        assert_eq!(roman_number_to_int("X").unwrap(), 10);
        assert_eq!(roman_number_to_int("XII").unwrap(), 12);
        assert_eq!(roman_number_to_int("IIX").unwrap(), 8);
        assert_eq!(roman_number_to_int("MDCLXVI").unwrap(), 1666);
        assert_eq!(roman_number_to_int("MMMCMXCIX").unwrap(), 3999);
    }
}