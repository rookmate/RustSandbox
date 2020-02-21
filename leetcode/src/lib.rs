
// zigzag_conversion(String::from("PAYPALISHIRING"), 3);
// zigzag_conversion(String::from("PAYPALISHIRING"), 4);
pub fn zigzag_conversion(s: String, num_rows: i32) -> String {

    s
}



// Add this tests in the future in lib.rs
// reverse(123);
// reverse(-123);
// reverse(1534236469);
pub fn reverse_integer(x: i32) -> i32 {
    let flag = x.is_negative();

    let s = x.abs().to_string().chars().rev().collect::<String>();

    let i = match s.parse::<i32>() {
        Err(_e) => return 0,
        Ok(i) => i,
    };

    if flag {
         i * -1
    } else {
        i
    }
}
