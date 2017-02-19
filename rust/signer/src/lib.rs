extern crate libc;

mod string;

use string::StringPtr;

#[no_mangle]
pub extern fn tmp_string() -> *mut StringPtr {
  let hello: StringPtr = "hello from rust".into();
  // let's put it on the heap
  let boxed_hello = Box::new(hello);
  // let's make a pointer to the heap, return it and forget about it
  Box::into_raw(boxed_hello)
}

#[no_mangle]
pub extern fn simple() -> i32 {
	12345
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
