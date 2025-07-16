/// Converts a string from `snake_case`, `kebab-case`, or `space separated`
/// to `camelCase`.
///
/// This function handles common delimiters like underscores (`_`), hyphens (`-`),
/// and whitespace, converting the character immediately following a delimiter
/// to uppercase, and removing the delimiter itself. The very first character
/// of the output string will be lowercase.
///
/// # Examples
///
/// Basic conversions:
/// ```
/// use rust_utils::strings::camel_case_converter::to_camel_case;
///
/// assert_eq!(to_camel_case("snake_case"), "snakeCase");
/// assert_eq!(to_camel_case("kebab-case"), "kebabCase");
/// assert_eq!(to_camel_case("normal case"), "normalCase");
/// ```
///
/// Handling leading/trailing delimiters and multiple delimiters:
/// ```
/// use rust_utils::strings::camel_case_converter::to_camel_case;
///
/// assert_eq!(to_camel_case("_leading_underscore"), "leadingUnderscore");
/// assert_eq!(to_camel_case("trailing_underscore_"), "trailingUnderscore");
/// assert_eq!(to_camel_case("  multiple   spaces  "), "multipleSpaces");
/// assert_eq!(to_camel_case("multiple__underscores"), "multipleUnderscores");
/// ```
///
/// Empty and single-word strings:
/// ```
/// use rust_utils::strings::camel_case_converter::to_camel_case;
///
/// assert_eq!(to_camel_case(""), "");
/// assert_eq!(to_camel_case("singleword"), "singleword");
/// assert_eq!(to_camel_case(" SingleWord "), "singleWord");
/// ```
///
/// # Arguments
///
/// * `s` - The string slice to convert. This can be in `snake_case`,
///   `kebab-case`, `space separated`, or any combination of these delimiters.
///
/// # Returns
///
/// A new `String` representing the camel-cased version of the input.
///
/// # Behavior Notes
///
/// * The very first character of the output string will always be lowercase,
///   even if the input string starts with an uppercase letter (e.g., "HelloWorld" becomes "helloWorld").
/// * Consecutive delimiters (e.g., `__`, `--`, `  `) are treated as a single delimiter.
/// * Leading and trailing delimiters are ignored.
/// * Non-alphanumeric characters that are not delimiters are included as-is.
///
pub fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    let mut first_char_processed = false;

    for c in s.chars() {
        if c == '_' || c == '-' || c.is_whitespace() {
            capitalize_next = true;
            continue;
        }

        if !first_char_processed {
            result.push(c.to_ascii_lowercase());
            first_char_processed = true;
            capitalize_next = false;
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

    #[test]
    fn test_camel_case_from_kebab_case() {
        assert_eq!(to_camel_case("hello-world"), "helloWorld");
        assert_eq!(to_camel_case("this-is-a-test"), "thisIsATest");
        assert_eq!(to_camel_case("leading-space"), "leadingSpace");
    }

    #[test]
    fn test_first_letter_upper_case() {
        assert_eq!(to_camel_case("Snake_case"), "snakeCase");
        assert_eq!(to_camel_case("Kebab-case"), "kebabCase");
        assert_eq!(to_camel_case("Normal case"), "normalCase");
    }
}
