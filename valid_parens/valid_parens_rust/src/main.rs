use std::collections::HashMap;

fn main() {
    println!("{}", valid_parens("{}[]()".to_string()));
    println!("{}", valid_parens("{[]()}".to_string()));
    println!("{}", valid_parens("{[}]".to_string()));
}

fn valid_parens(s: String) -> bool {
    let mut map: HashMap<char, char> = HashMap::new();
    map.insert('(', ')');
    map.insert('{', '}');
    map.insert('[', ']');

    let mut stack: Vec<&char> = Vec::new();

    for char in s.chars() {
        if let Some(c) = map.get(&char) {
            stack.push(c);
        } else {
            if let Some(popped) = stack.pop() {
                if *popped != char {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    stack.len() == 0
}
