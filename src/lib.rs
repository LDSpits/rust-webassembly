#[no_mangle]
pub fn add_one(x: i32) -> i32 {
    return x + x;
}

#[no_mangle]
pub fn multiply(amount: i32, times: i32) -> i32 {

    let variable = 0;
    return amount * times;
}
