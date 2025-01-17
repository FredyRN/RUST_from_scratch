/// RUST provides the Result<T, E> enum for returning and propagating errors.
/// By convention, the OK(T) variant represents a success and contains a value, and
/// the variant Err(E) represents an error and contains an error value.

/// The Result<T, E> enum is defined as:
/// enum Result<T, E> {
///     OK(T), // A value T was obtained
///     Err(E), // An error of type E was encountered instead.
/// }

#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}