fn main() {
    zigzag_conversion(String::from("PAYPALISHIRING"), 3);
}


pub fn zigzag_conversion(s: String, num_rows: i32) -> String {

    s
}


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
