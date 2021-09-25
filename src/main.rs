fn main() {
    // Let's take a mutable piece of data, a 4-byte integer in this case
    let mut some_data: u32 = 14;

    // Create a mutable raw pointer pointing to the data above
    let data_ptr: *mut u32 = &mut some_data as *mut u32;

    // Note: creating a raw pointer is totally safe but dereferencing a raw pointer requires an
    // unsafe block
    unsafe {
        *data_ptr = 20;
        println!("Dereferenced data: {}", some_data);
    }
    println!("some_data = {}", some_data);
}
