use std::collections::BTreeMap;

// zigzag_conversion(String::from("PAYPALISHIRING"), 3);
// zigzag_conversion(String::from("PAYPALISHIRING"), 4);
pub fn zigzag_conversion(s: String, num_rows: i32) -> String {
    if num_rows >= s.len() as i32 || num_rows <= 1 {
        return s
    }

    // Ok so, we hashmap the lines then print them in order
    // Compromise to get the hashmap ordered so using a binary search tree
    let mut s_by_line: BTreeMap<i32, String> = BTreeMap::new();
    let mut line: i32 = 0;
    let mut down = 0;

    for c in s.chars() {
        // Attempts to get key, if does not exist creates with empty string
        let value = s_by_line.entry(line).or_insert(String::from(""));
        // Pushes character to said line (key)
        value.push(c);
        // If first or last line change direction
        if line == 0 || line == num_rows - 1 {
            down ^= 1;
        }
        // Going up or down the lines
        if down  == 1 {
            line += 1;
        } else {
            line -= 1;
        }
    }

    s_by_line.values().map(|s_by_line| &**s_by_line).collect::<Vec<_>>().join("")
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
