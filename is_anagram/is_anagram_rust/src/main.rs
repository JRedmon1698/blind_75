use std::collections::HashMap;

fn main() {
    let s1 = String::from("cat");
    let s2 = String::from("car");
    println!("{}", is_anagram(s1, s2));
}

// pub fn is_anagram(s: String, t: String) -> bool {
//     if s.len() != t.len() {
//         return false;
//     }

//     let mut s_map: HashMap<char, u32> = HashMap::new();
//     let mut t_map: HashMap<char, u32> = HashMap::new();

//     for char in s.chars() {
//         s_map
//             .entry(char)
//             .and_modify(|counter| *counter += 1)
//             .or_insert(1);
//     }

//     for char in t.chars() {
//         t_map
//             .entry(char)
//             .and_modify(|counter| *counter += 1)
//             .or_insert(1);
//     }

//     if s_map.len() != t_map.len() {
//         return false;
//     }

//     for (char, count) in s_map.iter() {
//         match t_map.get(char) {
//             Some(c) if c == count => {}
//             _ => return false,
//         }
//     }

//     true
// }

pub fn is_anagram(s: String, t: String) -> bool {
    if s.chars().count() != t.chars().count() {
        return false;
    }

    let mut counts: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
        match counts.get_mut(&c) {
            Some(count) => {
                *count -= 1;
                if *count < 0 {
                    return false;
                }
            }
            None => return false,
        }
    }

    true
}
