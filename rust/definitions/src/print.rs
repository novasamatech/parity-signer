use crate::error::ErrorSigner;

pub fn export_plain_vector<T: std::fmt::Display>(vec: &Vec<T>) -> String {
    let mut out = String::from("[");
    for (i, x) in vec.iter().enumerate() {
        if i > 0 {
            out.push_str(",")
        }
        out.push_str(&format!("\"{}\"", x))
    }
    out.push_str("]");
    out
}

pub fn export_complex_vector<T, O>(vec: &Vec<T>, op: O) -> String
where
    O: Fn(&T) -> String + Copy,
{
    let mut out = String::from("[");
    for (i, x) in vec.iter().enumerate() {
        if i > 0 {
            out.push_str(",")
        }
        out.push_str(&export_complex_single(x, op))
    }
    out.push_str("]");
    out
}

pub fn export_complex_single<T, O>(x: &T, op: O) -> String
where
    O: Fn(&T) -> String + Copy,
{
    format!("{{{}}}", op(x))
}

pub fn export_complex_vector_with_error<T, O>(vec: &Vec<T>, op: O) -> Result<String, ErrorSigner>
where
    O: Fn(&T) -> Result<String, ErrorSigner> + Copy,
{
    let mut out = String::from("[");
    for (i, x) in vec.iter().enumerate() {
        if i > 0 {
            out.push_str(",")
        }
        out.push_str(&export_complex_single_with_error(x, op)?)
    }
    out.push_str("]");
    Ok(out)
}

pub fn export_complex_single_with_error<T, O>(x: &T, op: O) -> Result<String, ErrorSigner>
where
    O: Fn(&T) -> Result<String, ErrorSigner> + Copy,
{
    Ok(format!("{{{}}}", op(x)?))
}
