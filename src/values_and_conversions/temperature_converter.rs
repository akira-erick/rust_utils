//! This module provides functions for converting between common temperature scales.
//!
//! It includes conversions between Celsius, Fahrenheit, and Kelvin.
//! All functions operate on `f64` floating-point numbers.
//!
//! # Functions
//!
//! * [`from_celsius_to_fahrenheit`] - Converts Celsius to Fahrenheit.
//! * [`from_fahrenheit_to_celsius`] - Converts Fahrenheit to Celsius.
//! * [`from_celsius_to_kelvin`] - Converts Celsius to Kelvin.
//! * [`from_kelvin_to_celsius`] - Converts Kelvin to Celsius.
//! * [`from_fahrenheit_to_kelvin`] - Converts Fahrenheit to Kelvin.
//! * [`from_kelvin_to_fahrenheit`] - Converts Kelvin to Fahrenheit.
//!
//! [`from_celsius_to_fahrenheit`]: crate::values_and_conversions::temperature_converter::from_celsius_to_fahrenheit
//! [`from_fahrenheit_to_celsius`]: crate::values_and_conversions::temperature_converter::from_fahrenheit_to_celsius
//! [`from_celsius_to_kelvin`]: crate::values_and_conversions::temperature_converter::from_celsius_to_kelvin
//! [`from_kelvin_to_celsius`]: crate::values_and_conversions::temperature_converter::from_kelvin_to_celsius
//! [`from_fahrenheit_to_kelvin`]: crate::values_and_conversions::temperature_converter::from_fahrenheit_to_kelvin
//! [`from_kelvin_to_fahrenheit`]: crate::values_and_conversions::temperature_converter::from_kelvin_to_fahrenheit

/// Converts a temperature from Celsius to Fahrenheit.
///
/// # Examples
///
/// ```
/// use rust_utils::values_and_conversions::temperature_converter::from_celsius_to_fahrenheit;
///
/// assert_eq!(from_celsius_to_fahrenheit(0.0), 32.0);
/// assert_eq!(from_celsius_to_fahrenheit(100.0), 212.0);
/// assert_eq!(from_celsius_to_fahrenheit(-40.0), -40.0);
/// ```
///
/// # Arguments
///
/// * `celsius` - The temperature value in Celsius (`f64`).
///
/// # Returns
///
/// The equivalent temperature in Fahrenheit (`f64`).
pub fn from_celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

/// Converts a temperature from Fahrenheit to Celsius.
///
/// # Examples
///
/// ```rust
/// use rust_utils::values_and_conversions::temperature_converter::from_fahrenheit_to_celsius;
///
/// assert_eq!(from_fahrenheit_to_celsius(32.0), 0.0);
/// assert_eq!(from_fahrenheit_to_celsius(212.0), 100.0);
/// assert_eq!(from_fahrenheit_to_celsius(-40.0), -40.0);
/// ```
///
/// # Arguments
///
/// * `fahrenheit` - The temperature value in Fahrenheit (`f64`).
///
/// # Returns
///
/// The equivalent temperature in Celsius (`f64`).
pub fn from_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

/// Converts a temperature from Celsius to Kelvin.
///
/// The formula used is `$K = C + 273.15$$.
///
/// # Examples
///
/// ```rust
/// use rust_utils::values_and_conversions::temperature_converter::from_celsius_to_kelvin;
///
/// assert_eq!(from_celsius_to_kelvin(0.0), 273.15);
/// assert_eq!(from_celsius_to_kelvin(100.0), 373.15);
/// assert_eq!(from_celsius_to_kelvin(-273.15), 0.0);
/// ```
///
/// # Arguments
///
/// * `celsius` - The temperature value in Celsius (`f64`).
///
/// # Returns
///
/// The equivalent temperature in Kelvin (`f64`).
pub fn from_celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

/// Converts a temperature from Kelvin to Celsius.
///
/// # Examples
///
/// ```rust
/// use rust_utils::values_and_conversions::temperature_converter::from_kelvin_to_celsius;
///
/// assert_eq!(from_kelvin_to_celsius(273.15), 0.0);
/// assert_eq!(from_kelvin_to_celsius(373.15), 100.0);
/// assert_eq!(from_kelvin_to_celsius(0.0), -273.15);
/// ```
///
/// # Arguments
///
/// * `kelvin` - The temperature value in Kelvin (`f64`).
///
/// # Returns
///
/// The equivalent temperature in Celsius (`f64`).
pub fn from_kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

/// Converts a temperature from Fahrenheit to Kelvin.
///
/// This conversion is performed by first converting Fahrenheit to Celsius,
/// and then Celsius to Kelvin.
///
/// # Examples
///
/// ```rust
/// use rust_utils::values_and_conversions::temperature_converter::from_fahrenheit_to_kelvin;
///
/// assert_eq!(from_fahrenheit_to_kelvin(32.0), 273.15);
/// assert_eq!(from_fahrenheit_to_kelvin(212.0), 373.15);
/// ```
///
/// # Arguments
///
/// * `fahrenheit` - The temperature value in Fahrenheit (`f64`).
///
/// # Returns
///
/// The equivalent temperature in Kelvin (`f64`).
pub fn from_fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    from_celsius_to_kelvin(from_fahrenheit_to_celsius(fahrenheit))
}

/// Converts a temperature from Kelvin to Fahrenheit.
///
/// This conversion is performed by first converting Kelvin to Celsius,
/// and then Celsius to Fahrenheit.
///
/// # Examples
///
/// ```rust
/// use rust_utils::values_and_conversions::temperature_converter::from_kelvin_to_fahrenheit;
///
/// assert_eq!(from_kelvin_to_fahrenheit(273.15), 32.0);
/// assert_eq!(from_kelvin_to_fahrenheit(373.15), 212.0);
/// ```
///
/// # Arguments
///
/// * `kelvin` - The temperature value in Kelvin (`f64`).
///
/// # Returns
///
/// The equivalent temperature in Fahrenheit (`f64`).
pub fn from_kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    from_celsius_to_fahrenheit(from_kelvin_to_celsius(kelvin))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(from_celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(from_celsius_to_fahrenheit(100.0), 212.0);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(from_fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(from_fahrenheit_to_celsius(212.0), 100.0);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        assert_eq!(from_celsius_to_kelvin(0.0), 273.15);
        assert_eq!(from_celsius_to_kelvin(100.0), 373.15);
    }

    #[test]
    fn test_kelvin_to_celsius() {
        assert_eq!(from_kelvin_to_celsius(273.15), 0.0);
        assert_eq!(from_kelvin_to_celsius(373.15), 100.0);
    }

    #[test]
    fn test_fahrenheit_to_kelvin() {
        assert_eq!(from_fahrenheit_to_kelvin(32.0), 273.15);
        assert_eq!(from_fahrenheit_to_kelvin(212.0), 373.15);
    }

    #[test]
    fn test_kelvin_to_fahrenheit() {
        assert_eq!(from_kelvin_to_fahrenheit(273.15), 32.0);
        assert_eq!(from_kelvin_to_fahrenheit(373.15), 212.0);
    }
}
