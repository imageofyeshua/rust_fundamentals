use std::os::raw::c_char;

fn count_x(p: *const c_char, x: c_char) -> i32 {
    // Count the number of occurrences of x in p[]
    // p is assumed to point to a null-terminated array of c_char (or to nothing)
    if p.is_null() {
        return 0;
    }
    let mut count = 0;
    unsafe {
        let mut ptr = p;
        while *ptr != 0 {
            if *ptr == x {
                count += 1;
            }
            ptr = ptr.add(1); // move pointer to the next element
        }
    }
    count
}

fn main() {
    let c_str = std::ffi::CString::new("Our Father in heaven").unwrap();
    let c_ptr = c_str.as_ptr();

    let count = count_x(c_ptr, 'a' as c_char);
    println!("The character 'a' appears {} times", count);
}
