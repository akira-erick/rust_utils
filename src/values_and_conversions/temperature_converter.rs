/// Converts celsius to fahrenheit
/// 
/// # Examples
/// ```
/// let fahrenheit = from_celsius_to_fahrenheit(0.0);
/// assert_eq!(fahrenheit, 32.0);
/// ```
pub fn from_celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

/// Converts fahrenheit to celsius
/// 
/// # Examples
/// ```
/// let celsius = from_fahrenheit_to_celsius(32.0);
/// assert_eq!(celsius, 0.0);
/// ```
pub fn from_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

/// Converts celsius to kelvin
/// 
/// # Examples
/// ```
/// let kelvin = from_celsius_to_kelvin(0.0);
/// assert_eq!(kelvin, 273.15);
/// ```
pub fn from_celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

/// Converts kelvin to celsius
/// 
/// # Examples
/// ```
/// let celsius = from_kelvin_to_celsius(273.15);
/// assert_eq!(celsius, 0.0);
/// ```
pub fn from_kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

/// Converts fahrenheit to kelvin
///     
/// # Examples
/// ```
/// let kelvin = from_fahrenheit_to_kelvin(32.0);
/// assert_eq!(kelvin, 273.15);
/// ```
pub fn from_fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    from_celsius_to_kelvin(from_fahrenheit_to_celsius(fahrenheit))
}

/// Converts kelvin to fahrenheit
/// 
/// # Examples
/// ```
/// let fahrenheit = from_kelvin_to_fahrenheit(273.15);
/// assert_eq!(fahrenheit, 32.0);
/// ```
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
