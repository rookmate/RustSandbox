fn main() {
    println!("{}", leetcode::my_atoi(String::from("42")));
    println!("{}", leetcode::my_atoi(String::from("   -42")));
    println!("{}", leetcode::my_atoi(String::from("4193 with words")));
    println!("{}", leetcode::my_atoi(String::from("words and 987")));
    println!("{}", leetcode::my_atoi(String::from("  -0012a42")));
    println!("{}", leetcode::my_atoi(String::from("-")));
    println!("{}", leetcode::my_atoi(String::from("+-2")));
    println!("{}", leetcode::my_atoi(String::from("2147483648")));
    println!("{}", leetcode::my_atoi(String::from("-abc")));
    println!("{}", leetcode::my_atoi(String::from("0-1")));
}
