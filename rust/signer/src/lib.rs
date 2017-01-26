#[no_mangle]
pub extern fn simple() -> i32 {
	1234
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
