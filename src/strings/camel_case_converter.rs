/// Converts normalized strings to camel case.
///
/// # Examples
/// ```
/// use rust_utils::strings::camel_case_converter::to_camel_case;
///
/// let from_snake_case = to_camel_case("hello_world");
/// assert_eq!(from_snake_case, "helloWorld");
///
/// let from_normal = to_camel_case("hello world");
/// assert_eq!(from_normal, "helloWorld");
/// ```
pub fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    let mut first_char = true;

    for c in input.chars() {
        if first_char {
            if c.is_whitespace() || c == '_' {
                continue;
            } else {
                first_char = false;
            }
        }

        if c == '_' || c.is_whitespace() {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case_conversion() {
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
        assert_eq!(to_camel_case("this_is_a_test"), "thisIsATest");
        assert_eq!(to_camel_case("alreadyCamelCase"), "alreadyCamelCase");
        assert_eq!(to_camel_case(""), "");
        assert_eq!(to_camel_case("singleword"), "singleword");
    }

    #[test]
    fn test_camel_case_with_spaces() {
        assert_eq!(to_camel_case("hello world"), "helloWorld");
        assert_eq!(to_camel_case("this is a test"), "thisIsATest");
        assert_eq!(to_camel_case("leading space"), "leadingSpace");
        assert_eq!(to_camel_case("trailing space "), "trailingSpace");
        assert_eq!(to_camel_case(" multiple   spaces "), "multipleSpaces");
    }
}
