fn main() {
    let s1 = String::from("A man, a plan, a canal: Panama");
    let s2 = String::from("race a car");
    let s3 = String::from(" ");
    println!("{}", is_palindrome(s1));
    println!("{}", is_palindrome(s2));
    println!("{}", is_palindrome(s3));
}

pub fn is_palindrome(s: String) -> bool {
    let mut rev = String::new();

    for char in s.chars().rev() {
        rev.push(char);
    }

    let trimmed_s: String = s
        .chars()
        .filter(|c| !c.is_whitespace() && c.is_alphanumeric())
        .collect();
    let trimmed_rev: String = rev
        .chars()
        .filter(|c| !c.is_whitespace() && c.is_alphanumeric())
        .collect();

    trimmed_s.to_lowercase() == trimmed_rev.to_lowercase()
}
