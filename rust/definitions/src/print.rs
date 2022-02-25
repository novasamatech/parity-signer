use crate::error::ErrorSigner;

pub fn export_plain_vector<T: std::fmt::Display> (vec: &[T]) -> String {
    let mut out = String::from("[");
    for (i, x) in vec.iter().enumerate() {
        if i>0 {out.push(',')}
        out.push_str(&format!("\"{}\"", x))
    }
    out.push(']');
    out
}

pub fn export_complex_vector<T, O> (vec: &[T], op: O) -> String
where O: Fn(&T) -> String + Copy,
{
    let mut out = String::from("[");
    for (i, x) in vec.iter().enumerate() {
        if i>0 {out.push(',')}
        out.push_str(&export_complex_single(x, op))
    }
    out.push(']');
    out
}

pub fn export_complex_single<T, O> (x: &T, op: O) -> String
where O: Fn(&T) -> String + Copy,
{
    format!("{{{}}}", op(x))
}

pub fn export_complex_vector_with_error<T, O> (vec: &[T], op: O) -> Result<String, ErrorSigner>
where O: Fn(&T) -> Result<String, ErrorSigner> + Copy,
{
    let mut out = String::from("[");
    for (i, x) in vec.iter().enumerate() {
        if i>0 {out.push(',')}
        out.push_str(&export_complex_single_with_error(x, op)?)
    }
    out.push(']');
    Ok(out)
}

pub fn export_complex_single_with_error<T, O> (x: &T, op: O) -> Result<String, ErrorSigner>
where O: Fn(&T) -> Result<String, ErrorSigner> + Copy,
{
    Ok(format!("{{{}}}", op(x)?))
}
