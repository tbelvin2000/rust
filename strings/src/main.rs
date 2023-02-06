fn main() {
    let mut s1 = String::from("hello wordl");
    // let s2 = &mut s1;
    // let s3 = &s1;
    // let (s1, len) = calculate_length(&s1);
    let fw = first_word(&s1);
    println!("The first word of {} is {}", s1, fw);
    s1.clear();
    println!("The first word was {}", fw);
    
}

// fn calculate_length(s: &String) -> (&String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }

fn first_word(s: &String) -> String {
    for (i, ch) in s.chars().enumerate() {
        if ch == ' ' {return s[..i].to_string()};
    }
    s[..].to_string()
}

