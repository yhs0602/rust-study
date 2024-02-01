fn main() {
    let mut s1 = String::from("Hello");
    let len = calculate_length(&mut s1);
    println!("The length of '{}' is {}.", s1, len);

    {
        let r1 = &mut s1;
    }

    let r2 = &mut s1;
    let r3 = &s1;

    //println!("The value of r2 is {}.", r2);
    println!("The value of r3 is {}.", r3);

}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
