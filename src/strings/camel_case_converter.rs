


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
}