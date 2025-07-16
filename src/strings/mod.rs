//! This module contains various types and functions for handling strings.
//!
//! This includes utilities for case conversion, formatting, parsing, and more.
//!
//! ## Modules
//!
//! * [`camel_case_converter`] - Contains functions for converting strings to camel case.
//! * [`roman_number_converter`] - Contains functions for converting strings with roman numbers to integers.
//!
//! [`camel_case_converter`]: crate::strings::camel_case_converter
//! [`roman_number_converter`]: crate::strings::roman_number_converter

/// This module provides a function to convert string to camel case.
///
/// Use `to_camel_case` to transform strings from `snake_case`, `kebab-case`,
/// or `space separated` into `camelCase`.
pub mod camel_case_converter;

/// This module provides a function to convert strings with roman number to integer.
///
/// Use `roman_number_to_int` to convert roman numbers to its value.
pub mod roman_number_converter;
