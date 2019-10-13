#[cfg(test)]
mod tests {
    extern crate advapi32;
    extern crate winapi;

    use std::ptr;

    #[test]
    fn it_works() {
        assert_eq!(get_user_name(), "caleb");
    }

    fn get_user_name() -> String {
	unsafe {
	    let mut size = 0;
	    let retval = advapi32::GetUserNameW(ptr::null_mut(), &mut size);
	    assert_eq!(retval, 0, "Should have failed");

	    let mut username = Vec::with_capacity(size as usize);
	    let retval = advapi32::GetUserNameW(username.as_mut_ptr(), &mut size);
	    assert_ne!(retval, 0, "Perform better error handling");
	    assert!((size as usize) <= username.capacity());
	    username.set_len(size as usize);

	    // Beware: This leaves the trailing NUL character in the final string,
	    // you may want to remove it!
	    String::from_utf16(&username).unwrap()
	}
    }
}
