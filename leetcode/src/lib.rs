use std::collections::BTreeMap;
use std::i32::MIN;
use std::i32::MAX;

// my_atoi(String::from("42"));
// my_atoi(String::from("   -42"));
// my_atoi(String::from("4193 with words"));
// my_atoi(String::from("words and 987"));
// my_atoi(String::from("-91283472332"));
// my_atoi(String::from("3.14159")));
// my_atoi(String::from(".1")));
pub fn my_atoi(str: String) -> i32 {
    // Finds if number has ".", if so trims it
    let s: &str = str.split(".").collect::<Vec<_>>()[0].trim();
    if s.is_empty() {
        return 0
    }
    // String is empty or first elem is alphabetic
    let chars = s.chars();
    println!("{:?}", chars);
    let mut output: String = String::from("");
    let mut i: i8 = 0;
    let mut negative: bool = false;
    let mut numeric: bool = true;
    for c in chars {
        if !c.is_numeric() {
            numeric = false;
        }

        if c.is_alphabetic() || c.is_whitespace() || !numeric {
            break;
        }

        if i != 2 {
            if c.is_ascii_punctuation() || c.is_alphabetic() {
                i += 1;
                if c == '-' {
                    negative = true;
                    println!("NEGATIVE {}", negative);
                }
            }

            if i == 2 || (i == 1 && s.len() == 1) {
                return 0
            }
        }

        output.push(c);
    }

    if output.is_empty() || (output.len() == 1 && (output == "-" || output == "+")) {
        return 0
    }

    if negative { // Unwraps as an i32, if it busts returns -2^31
        return output.parse::<i32>().unwrap_or(MIN);
    } else {
        return output.parse::<i32>().unwrap_or(MAX);
    }
}


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
    // if number is negative, should change it back at the end
    let flag = x.is_negative();

    // make it positive
    // convert to char
    // revert order
    let s = x.abs().to_string().chars().rev().collect::<String>();

    // convert to int and add sign if needed
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
