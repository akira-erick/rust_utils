pub fn roman_number_to_int(input: &str) -> Result<i32, &str> {
    use std::collections::HashMap;
    if input.is_empty() {
        return Err("String shouldn't be empty");
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
    let mut char_iter = input.chars().peekable();
    while let Some(current_char) = char_iter.next() {
        let current = *map
            .get(&current_char)
            .ok_or("String with a character that are not a roman digit number.")?;
        if let Some(&next_char) = char_iter.peek() {
            let next = *map
                .get(&next_char)
                .ok_or("String with a character that are not a roman digit number.")?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_string_to_int() {
        assert_eq!(roman_number_to_int("X").unwrap(), 10);
        assert_eq!(roman_number_to_int("XII").unwrap(), 12);
        assert_eq!(roman_number_to_int("IX").unwrap(), 9);
        assert_eq!(roman_number_to_int("MDCLXVI").unwrap(), 1666);
        assert_eq!(roman_number_to_int("MMMCMXCIX").unwrap(), 3999);
    }
}
