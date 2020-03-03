use std::collections::BTreeMap;
use std::i32::MIN;
use std::i32::MAX;


pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut s: String = String::from("");
    let numerals: BTreeMap<i32, &str> = [
        (1,    "I"),
        (4,    "IV"),
        (5,    "V"),
        (9,    "IX"),
        (10,   "X"),
        (40,   "XL"),
        (50,   "L"),
        (90,   "XC"),
        (100,  "C"),
        (400,  "CD"),
        (500,  "D"),
        (900,  "CM"),
        (1000, "M")
    ].iter().cloned().collect();

    println!("ORIGINAL: {}", num);
    for sv in numerals.iter().rev() {
        let quotient: i32 = num / sv.0;
        if num >= *sv.0 {
            for _ in 0..quotient {
                s.push_str(sv.1);
            }

            num = num % sv.0;
        }
    }

    s
}

// my_atoi(String::from("42"))
// my_atoi(String::from("   -42"))
// my_atoi(String::from("4193 with words"))
// my_atoi(String::from("words and 987"))
// my_atoi(String::from("  -0012a42"))
// my_atoi(String::from("-"))
// my_atoi(String::from("+-2"))
// my_atoi(String::from("2147483648"))
// my_atoi(String::from("-abc"))
// my_atoi(String::from("0-1"))
pub fn my_atoi(str: String) -> i32 {
    // Finds if number has ".", if so trims it
    let s: &str = str.split(".").collect::<Vec<_>>()[0].trim_start();
    if s.is_empty() {
        return 0
    }
    // String is empty or first elem is alphabetic
    let chars = s.chars();
    println!("{:?}", chars);
    let mut s: String = String::from("");
    let mut negative: bool = false;
    let mut numeric: bool = false;
    for c in chars {
        if c.is_numeric() {
            numeric = true;
            s.push(c);
        } else {
            if !numeric { //It was not a numeric yet
                numeric = true;
                if c == '-' {
                    negative = true;
                    s.push(c);
                    continue;
                }

                if c == '+' {
                    s.push(c);
                    continue;
                }
            }
            //We already found a numeric before
            break;
        }
    }

    if s.is_empty() || (s.len() == 1 && (s == "-" || s == "+")) {
        return 0
    }

    // Unwraps as an i32, if it busts returns -2^31 or 2^31-1
    return s.parse::<i32>().unwrap_or(if negative { MIN } else { MAX });
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
