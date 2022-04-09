//! Json print helpers
//!
//! This part is going to go obsolete or at least change dramatically when
//! the data exchange between frontend and backend gets updated in one of the
//! next Signer iterations.
//!
//! Exact shape of the update is being discussed.
//!
//! Suggestions are welcome.
use crate::error_signer::ErrorSigner;

/// Prints vector of plain values into json string
///
/// `vec![a, b, c]` is printed as `["a","b","c"]`
pub fn export_plain_vector<T: std::fmt::Display>(vec: &[T]) -> String {
    let mut out = String::from("[");
    for (i, x) in vec.iter().enumerate() {
        if i > 0 {
            out.push(',')
        }
        out.push_str(&format!("\"{}\"", x))
    }
    out.push(']');
    out
}

/// Prints vector of complex values into json string
///
/// `vec![a, b, c]` is printed as `[{a_print},{b_print},{c_print}]`
///
/// `O` is operation to transform `element` into `element_print`
pub fn export_complex_vector<T, O>(vec: &[T], op: O) -> String
where
    O: Fn(&T) -> String + Copy,
{
    let mut out = String::from("[");
    for (i, x) in vec.iter().enumerate() {
        if i > 0 {
            out.push(',')
        }
        out.push_str(&export_complex_single(x, op))
    }
    out.push(']');
    out
}

/// Prints a complex value into json string
///
/// `a` is printed as `{a_print}`
///
/// `O` is operation to transform `element` into `element_print`
pub fn export_complex_single<T, O>(x: &T, op: O) -> String
where
    O: Fn(&T) -> String + Copy,
{
    format!("{{{}}}", op(x))
}

/// Prints vector of complex values into json string
///
/// `vec![a, b, c]` is printed as `[{a_print},{b_print},{c_print}]`
///
/// `O` is operation to transform `element` into `element_print`, `O` can
/// result in an error.
pub fn export_complex_vector_with_error<T, O>(vec: &[T], op: O) -> Result<String, ErrorSigner>
where
    O: Fn(&T) -> Result<String, ErrorSigner> + Copy,
{
    let mut out = String::from("[");
    for (i, x) in vec.iter().enumerate() {
        if i > 0 {
            out.push(',')
        }
        out.push_str(&export_complex_single_with_error(x, op)?)
    }
    out.push(']');
    Ok(out)
}

/// Prints a complex value into json string
///
/// `a` is printed as `{a_print}`
///
/// `O` is operation to transform `element` into `element_print`, `O` can
/// result in an error.
pub fn export_complex_single_with_error<T, O>(x: &T, op: O) -> Result<String, ErrorSigner>
where
    O: Fn(&T) -> Result<String, ErrorSigner> + Copy,
{
    Ok(format!("{{{}}}", op(x)?))
}
